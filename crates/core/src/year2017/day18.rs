use crate::input::Input;

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
            Some("rcv") => Some(Self::Rcv(Value::parse(parts.next()?)?)),
            Some("jgz") => Some(Self::Jgz(
                Value::parse(parts.next()?)?,
                Value::parse(parts.next()?)?,
            )),
            _ => None,
        }
    }
}

struct Computer {
    registers: Vec<NumberValue>,
    instruction_pointer: u8,
    last_played_frequency: NumberValue,
    instructions: Vec<Instruction>,
}

impl Computer {
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
            registers: vec![0; 26],
            instruction_pointer: 0,
            last_played_frequency: 0,
            instructions,
        })
    }

    fn value_of(&self, value: Value) -> NumberValue {
        match value {
            Value::Register(register_specifier) => self.registers[register_specifier as usize],
            Value::Number(number_value) => number_value,
        }
    }

    fn run_until_recover(&mut self) {
        loop {
            match self.instructions[self.instruction_pointer as usize] {
                Instruction::Snd(frequency) => {
                    self.last_played_frequency = self.value_of(frequency);
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
                    if self.value_of(x) != 0 {
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
    let mut computer = Computer::parse(input.text)?;
    computer.run_until_recover();
    Ok(computer.last_played_frequency)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day18_input.txt");
    test_part_one!(real_input => 3423);
    test_part_two!(real_input => 0);
}
