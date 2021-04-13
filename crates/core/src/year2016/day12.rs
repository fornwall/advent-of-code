use crate::Input;

type Word = i32;
type Register = u8;

#[derive(Copy, Clone)]
enum ValueOrRegister {
    Value(Word),
    Register(Register),
}

impl ValueOrRegister {
    fn parse(input: &str) -> Result<Self, String> {
        Ok(if ["a", "b", "c", "d"].contains(&input) {
            Self::Register(input.as_bytes()[0] - b'a')
        } else {
            Self::Value(input.parse::<Word>().map_err(|_| "Invalid value")?)
        })
    }
}

fn parse_register(input: &str) -> Result<Register, String> {
    if ["a", "b", "c", "d"].contains(&input) {
        Ok(input.as_bytes()[0] - b'a')
    } else {
        Err("Invalid register - not a/b/c/d".to_string())
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    // cpy x y copies x (either an integer or the value of a register) into register y.
    Copy(ValueOrRegister, Register),
    // inc x increases the value of register x by one.
    Increase(Register),
    // dec x decreases the value of register x by one.
    Decrease(Register),
    // jnz x y jumps to an instruction y away (positive means forward; negative means backward), but only if x is not zero.
    Jump(ValueOrRegister, ValueOrRegister),
}

impl Instruction {
    fn parse(input: &str) -> Result<Self, String> {
        let words = input.split(' ').collect::<Vec<_>>();
        match words[0] {
            "cpy" => {
                if words.len() == 3 {
                    let first_parameter = ValueOrRegister::parse(words[1])?;
                    let second_parameter = parse_register(words[2])?;
                    Ok(Self::Copy(first_parameter, second_parameter))
                } else {
                    return Err(format!(
                        "Invalid cpy instruction with {} arguments",
                        words.len() - 1
                    ));
                }
            }
            "inc" => {
                if words.len() == 2 {
                    Ok(Self::Increase(parse_register(words[1])?))
                } else {
                    Err(format!(
                        "Invalid inc instruction with {} arguments",
                        words.len() - 1
                    ))
                }
            }
            "dec" => {
                if words.len() == 2 {
                    Ok(Self::Decrease(parse_register(words[1])?))
                } else {
                    Err(format!(
                        "Invalid dec instruction with {} arguments",
                        words.len() - 1
                    ))
                }
            }
            "jnz" => {
                if words.len() == 3 {
                    let first_parameter = ValueOrRegister::parse(words[1])?;
                    let second_parameter = ValueOrRegister::parse(words[2])?;
                    Ok(Self::Jump(first_parameter, second_parameter))
                } else {
                    return Err(format!(
                        "Invalid jnz instruction with {} arguments",
                        words.len() - 1
                    ));
                }
            }
            _ => Err("Invalid instruction not starting with cpy, inc, dec or jnz".to_string()),
        }
    }
}

struct Computer {
    // The assembunny code you've extracted operates on four registers (a, b, c, and d) that start at 0 and can hold any integer
    registers: [Word; 4],
    instructions: Vec<Instruction>,
}

impl Computer {
    fn parse(input: &str) -> Result<Self, String> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            instructions.push(Instruction::parse(line)?);
        }
        Ok(Self {
            registers: [0, 0, 0, 0],
            instructions,
        })
    }

    fn execute(&mut self) -> Word {
        let mut current_instruction = 0;
        'outer: while let Some(&instruction) = self.instructions.get(current_instruction) {
            match instruction {
                Instruction::Copy(value_or_register, register) => {
                    let value = self.value_of(value_or_register);
                    self.registers[register as usize] = value;
                }
                Instruction::Increase(register) => {
                    self.registers[register as usize] += 1;
                }
                Instruction::Decrease(register) => {
                    self.registers[register as usize] -= 1;
                }
                Instruction::Jump(first, second) => {
                    if self.value_of(first) != 0 {
                        current_instruction =
                            (current_instruction as Word + self.value_of(second)) as usize;
                        continue 'outer;
                    }
                }
            }

            current_instruction += 1;
        }

        self.registers[0]
    }

    const fn value_of(&self, value_or_register: ValueOrRegister) -> Word {
        match value_or_register {
            ValueOrRegister::Value(word) => word,
            ValueOrRegister::Register(register_idx) => self.registers[register_idx as usize],
        }
    }
}

pub fn solve(input: &mut Input) -> Result<Word, String> {
    let mut computer = Computer::parse(input.text)?;
    if input.is_part_two() {
        computer.registers[2] = 1;
    }
    Ok(computer.execute())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day12_input.txt");
    test_part_one!(real_input => 318_020);
    test_part_two!(real_input => 9_227_674);
}
