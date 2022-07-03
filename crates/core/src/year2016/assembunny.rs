pub type Word = i32;
type Register = u8;

#[derive(Copy, Clone)]
pub enum ValueOrRegister {
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
pub enum Instruction {
    // cpy x y copies x (either an integer or the value of a register) into register y.
    Copy(ValueOrRegister, Register),
    // inc x increases the value of register x by one.
    Increase(Register),
    // dec x decreases the value of register x by one.
    Decrease(Register),
    // jnz x y jumps to an instruction y away (positive means forward; negative means backward), but only if x is not zero.
    Jump(ValueOrRegister, ValueOrRegister),
    // tgl x toggles the instruction x away (see day 23).
    Toggle(Register),
    // Instruction that does nothing. Used to implement Toggle.
    Nop,
    // out x transmits x (either an integer or the value of a register) as the next value for the clock signal.
    Out(ValueOrRegister),
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
                    Err(format!(
                        "Invalid cpy instruction with {} arguments",
                        words.len() - 1
                    ))
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
                    Err(format!(
                        "Invalid jnz instruction with {} arguments",
                        words.len() - 1
                    ))
                }
            }
            "tgl" => {
                let register = parse_register(words[1])?;
                Ok(Self::Toggle(register))
            }
            "out" => {
                let parameter = ValueOrRegister::parse(words[1])?;
                Ok(Self::Out(parameter))
            }
            _ => Err("Invalid instruction not starting with cpy, inc, dec or jnz".to_string()),
        }
    }

    const fn toggle(self) -> Self {
        match self {
            Self::Copy(a, b) => Self::Jump(a, ValueOrRegister::Register(b)),
            Self::Increase(a) => Self::Decrease(a),
            Self::Decrease(a) | Self::Toggle(a) => Self::Increase(a),
            Self::Out(a) => match a {
                ValueOrRegister::Register(register_value) => Self::Increase(register_value),
                _ => Self::Nop,
            },
            Self::Jump(a, b) => match b {
                ValueOrRegister::Register(register_value) => Self::Copy(a, register_value),
                _ => Self::Nop,
            },
            Self::Nop => Self::Nop,
        }
    }
}

pub struct Computer {
    // The assembunny code you've extracted operates on four registers (a, b, c, and d) that start at 0 and can hold any integer
    pub(crate) registers: [Word; 4],
    pub(crate) instructions: Vec<Instruction>,
}

impl Computer {
    pub(crate) fn parse(input: &str) -> Result<Self, String> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            instructions.push(Instruction::parse(line)?);
        }
        Ok(Self {
            registers: [0, 0, 0, 0],
            instructions,
        })
    }

    pub(crate) fn execute(&mut self) -> Word {
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
                Instruction::Toggle(register) => {
                    let value = self.registers[register as usize];
                    let ptr = current_instruction as Word + value;
                    if ptr < 0 || ptr >= self.instructions.len() as Word {
                        // If an attempt is made to toggle an instruction outside the program, nothing happens.
                    } else {
                        self.instructions[ptr as usize] = self.instructions[ptr as usize].toggle();
                    }
                }
                Instruction::Nop => {}
                Instruction::Out(_a) => {}
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
