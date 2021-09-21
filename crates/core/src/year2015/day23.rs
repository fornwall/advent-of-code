use crate::Input;

#[derive(Copy, Clone)]
enum Instruction {
    Half(u8),
    Triple(u8),
    Increment(u8),
    Jump(i16),
    JumpIfEven(u8, i16),
    JumpIfOne(u8, i16),
}

struct Computer {
    registers: [u32; 2],
    instructions: Vec<Instruction>,
}

impl Computer {
    fn parse_register(specifier: &str) -> Result<u8, String> {
        Ok(match specifier {
            "a" | "a," => 0,
            "b" => 1,
            _ => {
                return Err("Invalid register (not 'a' or 'b')".to_string());
            }
        })
    }

    fn parse(input: &str) -> Result<Self, String> {
        let mut instructions = Vec::new();

        for line in input.lines() {
            let parts = line.split(' ').collect::<Vec<_>>();
            instructions.push(match parts[0] {
                "hlf" => Instruction::Half(Self::parse_register(parts[1])?),
                "tpl" => Instruction::Triple(Self::parse_register(parts[1])?),
                "inc" => Instruction::Increment(Self::parse_register(parts[1])?),
                "jmp" => Instruction::Jump(
                    parts[1]
                        .parse::<i16>()
                        .map_err(|_| "Invalid jmp parameter")?,
                ),
                "jie" => Instruction::JumpIfEven(
                    Self::parse_register(parts[1])?,
                    parts[2]
                        .parse::<i16>()
                        .map_err(|_| "Invalid jie parameter")?,
                ),
                "jio" => Instruction::JumpIfOne(
                    Self::parse_register(parts[1])?,
                    parts[2]
                        .parse::<i16>()
                        .map_err(|_| "Invalid jio parameter")?,
                ),
                _ => {
                    return Err("Invalid instruction".to_string());
                }
            });
        }

        Ok(Self {
            registers: [0, 0],
            instructions,
        })
    }

    fn run(&mut self) {
        let mut instruction_pointer = 0_i16;
        loop {
            if instruction_pointer < 0 {
                break;
            }
            match self.instructions.get(instruction_pointer as usize) {
                Some(&Instruction::Increment(register)) => {
                    self.registers[register as usize] += 1;
                    instruction_pointer += 1;
                }
                Some(&Instruction::Half(register)) => {
                    self.registers[register as usize] /= 2;
                    instruction_pointer += 1;
                }
                Some(&Instruction::Triple(register)) => {
                    self.registers[register as usize] *= 3;
                    instruction_pointer += 1;
                }
                Some(&Instruction::Jump(offset)) => {
                    instruction_pointer += offset;
                }
                Some(&Instruction::JumpIfEven(register, offset)) => {
                    instruction_pointer += if self.registers[register as usize] % 2 == 0 {
                        offset
                    } else {
                        1
                    }
                }
                Some(&Instruction::JumpIfOne(register, offset)) => {
                    instruction_pointer += if self.registers[register as usize] == 1 {
                        offset
                    } else {
                        1
                    }
                }
                None => {
                    break;
                }
            }
        }
    }
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut computer = Computer::parse(input.text)?;
    computer.registers[0] = input.part_values(0, 1);
    computer.run();
    Ok(computer.registers[1])
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day23_input.txt");
    test_part_one!(real_input => 307);
    test_part_two!(real_input => 160);
}
