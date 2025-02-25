use crate::input::Input;

type CalculatorValue = u64;

struct Parser {
    operand_stack: Vec<CalculatorValue>,
    operator_stack: Vec<u8>,
    postfix_expression: Vec<u8>,
    addition_precedence: u8,
    multiplication_precedence: u8,
}

impl Parser {
    fn new(addition_has_higher_precedence: bool) -> Self {
        let (addition_precedence, multiplication_precedence) = if addition_has_higher_precedence {
            (2, 1)
        } else {
            (1, 1)
        };

        Self {
            operand_stack: Vec::with_capacity(64),
            operator_stack: Vec::with_capacity(64),
            postfix_expression: Vec::with_capacity(64),
            addition_precedence,
            multiplication_precedence,
        }
    }

    fn reset(&mut self) {
        self.operand_stack.clear();
        self.operator_stack.clear();
        self.postfix_expression.clear();
    }

    const fn operator_precedence(&self, char: u8, top_of_stack: bool) -> u8 {
        match char {
            b'+' => self.addition_precedence,
            b'*' => self.multiplication_precedence,
            b'(' => {
                // '(' has lowest precedence when top of stack:
                if top_of_stack { 0 } else { 4 }
            }
            _ => 3,
        }
    }

    fn consume(&mut self, char: u8) -> Result<(), String> {
        match char {
            b'(' | b')' | b'+' | b'*' => {
                let this_operator_precedence = self.operator_precedence(char, false);
                loop {
                    if let Some(&operator) = self.operator_stack.last() {
                        let top_of_stack_operator_precendence =
                            self.operator_precedence(operator, true);

                        if this_operator_precedence > top_of_stack_operator_precendence {
                            self.operator_stack.push(char);
                            break;
                        } else {
                            self.operator_stack.pop();
                            if !matches!(operator, b'(' | b')') {
                                self.postfix_expression.push(operator);
                            } else if operator == b')' {
                                // Pop everything off the operator stack until we see the matching left parentheses:
                                while let Some(operator) = self.operator_stack.pop() {
                                    if operator == b'(' {
                                        break;
                                    } else if operator != b')' {
                                        self.postfix_expression.push(operator);
                                    }
                                }
                            }
                        }
                    } else {
                        self.operator_stack.push(char);
                        break;
                    }
                }
                Ok(())
            }
            b'0'..=b'9' => {
                self.postfix_expression.push(char);
                Ok(())
            }
            b' ' => Ok(()),
            _ => Err(format!("Invalid char: {char}")),
        }
    }

    fn finish(&mut self) -> Result<CalculatorValue, String> {
        let on_error = || "Unbalanced operators".to_string();
        while let Some(operator) = self.operator_stack.pop() {
            if !matches!(operator, b'(' | b')') {
                self.postfix_expression.push(operator);
            }
        }

        for &c in self.postfix_expression.iter() {
            match c {
                b'+' | b'*' => {
                    let o1 = self.operand_stack.pop().ok_or_else(on_error)?;
                    let o2 = self.operand_stack.pop().ok_or_else(on_error)?;
                    let result = if c == b'+' { o1 + o2 } else { o1 * o2 };
                    self.operand_stack.push(result);
                }
                _ => {
                    let digit_value = c - b'0';
                    self.operand_stack.push(CalculatorValue::from(digit_value));
                }
            }
        }

        self.operand_stack.last().copied().ok_or_else(on_error)
    }
}

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut parser = Parser::new(input.is_part_two());
    input
        .text
        .lines()
        .map(|line| {
            parser.reset();
            for char in line.bytes() {
                parser.consume(char)?;
            }
            parser.finish()
        })
        .sum::<Result<CalculatorValue, String>>()
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "1 + 2 * 3 + 4 * 5 + 6";
    test_part_one!(example => 71);
    test_part_two!(example => 231);

    let real_input = include_str!("day18_input.txt");
    test_part_one!(real_input => 16_332_191_652_452);
    test_part_two!(real_input => 351_175_492_232_654);
}
