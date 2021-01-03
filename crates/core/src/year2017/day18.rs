use crate::input::Input;
use std::collections::VecDeque;

type RegisterSpecifier = u8;
type NumberValue = i64;

#[derive(Copy, Clone)]
enum Value {
    Register(RegisterSpecifier),
    Number(NumberValue),
}

impl Value {
    fn parse(input: &str) -> Option<Self> {
        Some(if let Some(register_vaule) = Self::parse_register(input) {
            Self::Register(register_vaule)
        } else {
            Self::Number(input.parse::<NumberValue>().ok()?)
        })
    }

    fn parse_register(input: &str) -> Option<RegisterSpecifier> {
        if input.len() == 1 {
            match input.chars().next() {
                Some(c) if ('a'..='z').contains(&c) => Some(c as u8 - b'a'),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Snd(Value),
    Set(RegisterSpecifier, Value),
    Add(RegisterSpecifier, Value),
    Mul(RegisterSpecifier, Value),
    Mod(RegisterSpecifier, Value),
    Rcv(Value),
    Jgz(Value, Value),
}

impl Instruction {
    fn parse(input: &str) -> Option<Self> {
        let mut parts = input.split(' ');
        match parts.next() {
            Some("snd") => Some(Self::Snd(Value::parse(parts.next()?)?)),
            Some("set") => Some(Self::Set(
                Value::parse_register(parts.next()?)?,
                Value::parse(parts.next()?)?,
            )),
            Some("add") => Some(Self::Add(
                Value::parse_register(parts.next()?)?,
                Value::parse(parts.next()?)?,
            )),
            Some("mul") => Some(Self::Mul(
                Value::parse_register(parts.next()?)?,
                Value::parse(parts.next()?)?,
            )),
            Some("mod") => Some(Self::Mod(
                Value::parse_register(parts.next()?)?,
                Value::parse(parts.next()?)?,
            )),
            Some("rcv") => {
                let value = Value::parse(parts.next()?)?;
                if matches!(value, Value::Number(_)) {
                    // rcv expects a register.
                    return None;
                }
                Some(Self::Rcv(value))
            }
            Some("jgz") => Some(Self::Jgz(
                Value::parse(parts.next()?)?,
                Value::parse(parts.next()?)?,
            )),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Program {
    registers: Vec<NumberValue>,
    instruction_pointer: u8,
    terminated: bool,
    last_played_frequency: NumberValue,
    instructions: Vec<Instruction>,
    executed_instructions_count: u32,
    input_queue: VecDeque<NumberValue>,
    sent_value_count: NumberValue,
}

impl Program {
    const MAX_INSTRUCTIONS: u32 = 100_000;

    fn parse(input: &str) -> Result<Self, String> {
        let instructions = input
            .lines()
            .enumerate()
            .map(|(line_idx, line)| {
                Instruction::parse(line)
                    .ok_or_else(|| format!("Line {}: Invalid instruction", line_idx + 1))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            registers: vec![0; 28],
            instruction_pointer: 0,
            terminated: false,
            last_played_frequency: 0,
            instructions,
            executed_instructions_count: 0,
            input_queue: VecDeque::new(),
            sent_value_count: 0,
        })
    }

    fn value_of(&self, value: Value) -> NumberValue {
        match value {
            Value::Register(register_specifier) => self.registers[register_specifier as usize],
            Value::Number(number_value) => number_value,
        }
    }

    fn run_until_recover(&mut self, mut output_queue: Option<&mut VecDeque<NumberValue>>) {
        loop {
            if self.executed_instructions_count >= Self::MAX_INSTRUCTIONS
                || self.instruction_pointer as usize >= self.instructions.len()
            {
                self.terminated = true;
                return;
            }

            self.executed_instructions_count += 1;

            match self.instructions[self.instruction_pointer as usize] {
                Instruction::Snd(x) => {
                    let x_value = self.value_of(x);
                    if let Some(ref mut queue) = output_queue {
                        self.sent_value_count += 1;
                        queue.push_back(x_value);
                    } else {
                        self.last_played_frequency = x_value;
                    }
                }
                Instruction::Set(x, y) => {
                    self.registers[x as usize] = self.value_of(y);
                }
                Instruction::Add(x, y) => {
                    self.registers[x as usize] += self.value_of(y);
                }
                Instruction::Mul(x, y) => {
                    self.registers[x as usize] *= self.value_of(y);
                }
                Instruction::Mod(x, y) => {
                    self.registers[x as usize] %= self.value_of(y);
                }
                Instruction::Rcv(x) => {
                    if output_queue.is_none() {
                        if self.value_of(x) != 0 {
                            return;
                        }
                    } else if let Some(value) = self.input_queue.pop_front() {
                        if let Value::Register(specifier) = x {
                            self.registers[specifier as usize] = value;
                        }
                    } else {
                        // TODO: When returning, need to store. but is perhaps working
                        // if we're not incrementing instruction pointer here?
                        return;
                    }
                }
                Instruction::Jgz(x, y) => {
                    if self.value_of(x) > 0 {
                        self.instruction_pointer =
                            (NumberValue::from(self.instruction_pointer) + self.value_of(y)) as u8;
                        continue;
                    }
                }
            }

            self.instruction_pointer += 1;
        }
    }
}

pub fn solve(input: &mut Input) -> Result<NumberValue, String> {
    let mut program_zero = Program::parse(input.text)?;
    if input.is_part_one() {
        program_zero.run_until_recover(None);
        Ok(program_zero.last_played_frequency)
    } else {
        let mut program_one = program_zero.clone();

        // "Each program also has its own program ID (one 0 and the other 1);
        // the register p should begin with this value."
        program_zero.registers[(b'p' - b'a') as usize] = 0;
        program_one.registers[(b'p' - b'a') as usize] = 1;

        loop {
            if !program_zero.terminated {
                program_zero.run_until_recover(Some(&mut program_one.input_queue));
            }
            if !program_one.terminated {
                program_one.run_until_recover(Some(&mut program_zero.input_queue));
            }

            if (program_zero.terminated && program_one.terminated)
                || (program_zero.input_queue.is_empty() && program_one.input_queue.is_empty())
            {
                // Both programs terminated, or both programs deadlocking.
                break;
            }
        }

        Ok(program_one.sent_value_count)
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example_input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
    test_part_two!(example_input => 3);

    let real_input = include_str!("day18_input.txt");
    test_part_one!(real_input => 3423);
    test_part_two!(real_input => 7493);
}
