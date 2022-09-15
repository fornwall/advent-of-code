use std::collections::VecDeque;

pub type RegisterSpecifier = u8;
pub type NumberValue = i64;

#[derive(Copy, Clone)]
pub enum Value {
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
            let c = input.as_bytes()[0];
            if (b'a'..=b'z').contains(&c) {
                return Some(c - b'a');
            }
        }
        None
    }
}

#[derive(Copy, Clone)]
pub enum Instruction {
    Snd(Value),
    Set(RegisterSpecifier, Value),
    Add(RegisterSpecifier, Value),
    Sub(RegisterSpecifier, Value),
    Mul(RegisterSpecifier, Value),
    Mod(RegisterSpecifier, Value),
    Rcv(Value),
    Jgz(Value, Value),
    Jnz(Value, Value),
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
            Some("sub") => Some(Self::Sub(
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
            Some("jnz") => Some(Self::Jnz(
                Value::parse(parts.next()?)?,
                Value::parse(parts.next()?)?,
            )),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Program {
    pub(crate) registers: Vec<NumberValue>,
    instruction_pointer: u8,
    pub(crate) terminated: bool,
    pub(crate) last_played_frequency: NumberValue,
    pub(crate) instructions: Vec<Instruction>,
    executed_instructions_count: u32,
    pub(crate) input_queue: VecDeque<NumberValue>,
    pub(crate) sent_value_count: NumberValue,
    pub(crate) mul_count: u32,
}

impl Program {
    const MAX_INSTRUCTIONS: u32 = 100_000;

    pub(crate) fn parse(input: &str) -> Result<Self, String> {
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
            mul_count: 0,
        })
    }

    fn value_of(&self, value: Value) -> NumberValue {
        match value {
            Value::Register(register_specifier) => self.registers[register_specifier as usize],
            Value::Number(number_value) => number_value,
        }
    }

    pub(crate) fn run_until_recover(
        &mut self,
        mut output_queue: Option<&mut VecDeque<NumberValue>>,
    ) {
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
                Instruction::Sub(x, y) => {
                    self.registers[x as usize] -= self.value_of(y);
                }
                Instruction::Mul(x, y) => {
                    self.mul_count += 1;
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
                Instruction::Jnz(x, y) => {
                    if self.value_of(x) != 0 {
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
