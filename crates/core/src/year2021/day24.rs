use crate::input::Input;

const NUM_DIGITS_IN_MODEL_NUMBER: usize = 14;

pub fn solve(input: &Input) -> Result<u64, String> {
    let instructions = input
        .text
        .lines()
        .map(Instruction::parse)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "Invalid input".to_string())?;

    let input_blocks = extract_input_blocks(&instructions)?;

    let mut model_number = [0; NUM_DIGITS_IN_MODEL_NUMBER];
    let mut stack = Vec::new();

    for (block_idx, block) in input_blocks.iter().enumerate() {
        // See steps in extract_input_blocks():
        //   x = 0 if (z % 26 + $X_NUMBER) == w else 1
        //   z = (z / $Z_DIVISION) * (25 * x + 1) + (w + $Y_NUMBER) * x
        // Written out when x is 1:
        //   z = (z / $Z_DIVISION) * 26 + w + $Y_NUMBER
        // Written out when x is 0:
        //   z = (z / $Z_DIVISION)
        // This can be seen as a stack, with the following stack operations:
        //    push($NUMBER_TO_PUSH): z = z*26 + $NUMBER_TO_PUSH
        //    pop(): z = z/26
        //    peek(): z % 26
        // Using that notation:
        //    do_push = stack.peek() + $X_ADDITION != w
        //    if z_division==26: stack.pop()
        //    if do_push: stack.push(w + $Y_ADDITION)
        // Restriction on input: if z_division is 1, X_NUMBER is always > 10, which means that
        // "stack.peek() + $X_NUMBER != w" will always be true, since w is a digit. So when z_division is 1:
        //    stack.push(w + $Y_ADDITION)
        // There are 7 input blocks of this type, so 7 values pushed on the stack. These needs to be popped in order for z to end up zero.
        // So with z_division is 26:
        //    do_push = stack.peek() + $X_NUMBER != w
        // We need all these blocks to only pop and not push, so need "stack.peek() + $X_NUMBER == w" for these.
        if block.z_division == 1 {
            stack.push((block_idx, /* w_pushed + */ block.y_addition));
        } else {
            let (pushing_block_idx, /* w_pushed + */ y_addition) = stack
                .pop()
                .ok_or_else(|| "Assumption broken: pop() has no matching push()".to_string())?;
            // w_pushed + y_addition + x_addition == w_current
            // =>
            // w_pushed = w_current - (y_addition + x_addition)
            // w_current = w_pushed + (y_addition + x_addition)
            // Pushing block index comes first (higher decimal position), so most important (but must be one digit):
            let input_difference = y_addition + block.x_addition;
            if !(-8..=8).contains(&input_difference) {
                return Err(
                    "Assumption broken: input difference is not in the range [-8,8]".to_string(),
                );
            }
            let w_pushed = if input.is_part_one() {
                // We need highest value on this leftmost digit that results in rightmost digit <= 9.
                std::cmp::min(9 - input_difference, 9)
            } else {
                // We need lowest value on this leftmost digit that results in rightmost digit >= 1.
                std::cmp::max(1 - input_difference, 1)
            };
            model_number[pushing_block_idx] = w_pushed;
            model_number[block_idx] = w_pushed + input_difference;
        }
    }

    Ok(model_number
        .iter()
        .fold(0, |acc, digit| acc * 10 + *digit as u64))
}

fn extract_input_blocks(instructions: &[Instruction]) -> Result<InputBlocks, String> {
    let mut input_blocks = [InputBlock {
        z_division: 0,
        x_addition: 0,
        y_addition: 0,
    }; NUM_DIGITS_IN_MODEL_NUMBER];
    let mut input_instructions_count = 0;
    for (instruction_idx, instruction) in instructions.iter().enumerate() {
        if let Instruction::Input(variable) = instruction {
            if *variable == Variable::W {
                // After every input to w:
                // Start: "x = z % 26"
                if !(matches!(
                    instructions[instruction_idx + 1],
                    Instruction::Multiply(Variable::X, VariableOrNumber::Number(0))
                ) && matches!(
                    instructions[instruction_idx + 2],
                    Instruction::Add(Variable::X, VariableOrNumber::Variable(Variable::Z))
                ) && matches!(
                    instructions[instruction_idx + 3],
                    Instruction::Modulo(Variable::X, VariableOrNumber::Number(26))
                )) {
                    return Err("Assumption broken: Not every input followed by 'mul x 0; add x z; mod x 26'".to_string());
                }

                // Then: "z = z / 1|26" (divide z by either 1 or 26)
                //   x = z % 26
                //   z = z / 1|26
                match instructions[instruction_idx + 4] {
                    Instruction::Divide(Variable::Z, VariableOrNumber::Number(z_division))
                        if (z_division == 1 || z_division == 26) =>
                    {
                        input_blocks[input_instructions_count].z_division = z_division as u8;
                    }
                    _ => {
                        return Err("Assumption broken: z is divided by 1 or 26".to_string());
                    }
                }

                // Then: "x = x + $X_NUMBER"
                //   x = z % 26 + $X_NUMBER
                //   z = z / 1|26
                if let Instruction::Add(Variable::X, VariableOrNumber::Number(x_addition)) =
                    instructions[instruction_idx + 5]
                {
                    input_blocks[input_instructions_count].x_addition = x_addition;
                } else {
                    return Err("Assumption broken: x is not added to".to_string());
                }
                // Then: "eql x w; eql x 0", which is "x = 0 if x == w else 1", expanded to "x = 0 if (z % 26 + $NUMBER) == w else 1"
                //   x = 0 if (z % 26 + $X_NUMBER) == w else 1
                //   z = z / 26    (optionally)
                if !(matches!(
                    instructions[instruction_idx + 6],
                    Instruction::Equal(Variable::X, VariableOrNumber::Variable(Variable::W))
                ) && matches!(
                    instructions[instruction_idx + 7],
                    Instruction::Equal(Variable::X, VariableOrNumber::Number(0))
                )) {
                    return Err(
                        "Assumption broken: x is not checked for equality to w as expected"
                            .to_string(),
                    );
                }
                // Then: "mul y 0; add y 25; mul y x; add y 1"
                //   x = 0 if (z % 26 + $X_NUMBER) == w else 1
                //   y = 25 * x + 1
                //   z = z / 26    (optionally)
                if !(matches!(
                    instructions[instruction_idx + 8],
                    Instruction::Multiply(Variable::Y, VariableOrNumber::Number(0))
                ) && matches!(
                    instructions[instruction_idx + 9],
                    Instruction::Add(Variable::Y, VariableOrNumber::Number(25))
                ) && matches!(
                    instructions[instruction_idx + 10],
                    Instruction::Multiply(Variable::Y, VariableOrNumber::Variable(Variable::X))
                ) && matches!(
                    instructions[instruction_idx + 11],
                    Instruction::Add(Variable::Y, VariableOrNumber::Number(1))
                )) {
                    return Err("Assumption broken: y is not set to 25*x+1".to_string());
                }
                // Then: "mul z y; mul y 0; add y w"
                //   x = 0 if (z % 26 + $X_NUMBER) == w else 1
                //   y = w
                //   z = (z / 1|26) * (25 * x + 1)
                if !(matches!(
                    instructions[instruction_idx + 12],
                    Instruction::Multiply(Variable::Z, VariableOrNumber::Variable(Variable::Y))
                ) && matches!(
                    instructions[instruction_idx + 13],
                    Instruction::Multiply(Variable::Y, VariableOrNumber::Number(0))
                ) && matches!(
                    instructions[instruction_idx + 14],
                    Instruction::Add(Variable::Y, VariableOrNumber::Variable(Variable::W))
                )) {
                    return Err(
                        "Assumption broken: z is not multiplied by y and then y reset".to_string(),
                    );
                }
                // Then: "add y $Y_NUMBER", simplified: "y = w + $Y_NUMBER"
                //   x = 0 if (z % 26 + $X_NUMBER) == w else 1
                //   y = w + $Y_NUMBER
                //   z = (z / 1|26) * (25 * x + 1)
                if let Instruction::Add(Variable::Y, VariableOrNumber::Number(y_addition)) =
                    instructions[instruction_idx + 15]
                {
                    if y_addition > 16 {
                        return Err("Assumption broken: y addition is > 16".to_string());
                    }
                    input_blocks[input_instructions_count].y_addition = y_addition;
                } else {
                    return Err("Assumption broken: y is not added to".to_string());
                }

                // Then: "mul y x; add z y"
                //   x = 0 if (z % 26 + $X_NUMBER) == w else 1
                //   z = (w + $Y_NUMBER) * x + (z / 1|26) * (25 * x + 1)
                if !(matches!(
                    instructions[instruction_idx + 16],
                    Instruction::Multiply(Variable::Y, VariableOrNumber::Variable(Variable::X))
                ) && matches!(
                    instructions[instruction_idx + 17],
                    Instruction::Add(Variable::Z, VariableOrNumber::Variable(Variable::Y))
                )) {
                    return Err("Assumption broken: y is not added to".to_string());
                }

                if input_blocks[input_instructions_count].z_division == 1
                    && input_blocks[input_instructions_count].x_addition < 10
                {
                    return Err(
                        "Assumption broken: when z division is 1, x addition is not > 9"
                            .to_string(),
                    );
                }

                input_instructions_count += 1;
            } else {
                return Err("Assumption broken: Not every input stored to variable 'w'".to_string());
            }
        }
    }

    if input_instructions_count != 14 {
        return Err("Assumption broken: Not 14 input instructions to variable 'w'".to_string());
    }

    if input_blocks.iter().filter(|b| b.z_division == 1).count()
        != input_blocks.iter().filter(|b| b.z_division == 26).count()
    {
        return Err("Assumption broken: Not equal amount of push and pop operations".to_string());
    }

    Ok(input_blocks)
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

impl Variable {
    fn parse(text: &str) -> Option<Self> {
        Some(match text {
            "w" => Self::W,
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            _ => {
                return None;
            }
        })
    }
}

#[derive(Copy, Clone)]
enum VariableOrNumber {
    Variable(Variable),
    Number(i8),
}

impl VariableOrNumber {
    fn parse(text: &str) -> Option<Self> {
        Some(match Variable::parse(text) {
            Some(variable) => Self::Variable(variable),
            None => Self::Number(text.parse().ok()?),
        })
    }
}

enum Instruction {
    // Read an input value and write it to variable a.
    Input(Variable),
    // Add the value of a to the value of b, then store the result in variable a.
    Add(Variable, VariableOrNumber),
    // Multiply the value of a by the value of b, then store the result in variable a.
    Multiply(Variable, VariableOrNumber),
    // Divide the value of a by the value of b, truncate the result to an integer, then store the result in variable a. (Here, "truncate" means to round the value toward zero.)
    Divide(Variable, VariableOrNumber),
    // Divide the value of a by the value of b, then store the remainder in variable a.
    Modulo(Variable, VariableOrNumber),
    // If the value of a and b are equal, then store the value 1 in variable a. Otherwise, store the value 0 in variable a.
    Equal(Variable, VariableOrNumber),
}

impl Instruction {
    fn parse(text: &str) -> Option<Self> {
        let mut words = text.split(' ');
        let first_word = words.next()?;
        let second_word = words.next()?;

        let first_parameter = Variable::parse(second_word)?;
        Some(if first_word == "inp" {
            Self::Input(first_parameter)
        } else {
            let third_word = words.next()?;
            let second_parameter = VariableOrNumber::parse(third_word)?;
            match first_word {
                "add" => Self::Add(first_parameter, second_parameter),
                "mul" => Self::Multiply(first_parameter, second_parameter),
                "div" => Self::Divide(first_parameter, second_parameter),
                "mod" => Self::Modulo(first_parameter, second_parameter),
                "eql" => Self::Equal(first_parameter, second_parameter),
                _ => {
                    return None;
                }
            }
        })
    }
}

#[derive(Copy, Clone)]
struct InputBlock {
    z_division: u8,
    x_addition: i8,
    y_addition: i8,
}

type InputBlocks = [InputBlock; NUM_DIGITS_IN_MODEL_NUMBER];

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day24_input.txt");
    test_part_one!(real_input => 99_299_513_899_971);
    test_part_two!(real_input => 93_185_111_127_911);
}
