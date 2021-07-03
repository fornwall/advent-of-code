use super::elfcode::Opcode;
use crate::Input;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq)]
struct Registers {
    values: [u16; 4],
}

impl Registers {
    const fn of(r0: u16, r1: u16, r2: u16, r3: u16) -> Self {
        Self {
            values: [r0, r1, r2, r3],
        }
    }

    fn reg(&mut self, index: u16) -> u16 {
        self.values[index as usize]
    }

    fn apply(&mut self, opcode: Opcode, a: u16, b: u16, c: u16) {
        let c = c as usize;
        self.values[c] = match opcode {
            Opcode::Addr => self.reg(a) + self.reg(b),
            Opcode::Addi => self.reg(a) + b,
            Opcode::Mulr => self.reg(a) * self.reg(b),
            Opcode::Muli => self.reg(a) * b,
            Opcode::Banr => self.reg(a) & self.reg(b),
            Opcode::Bani => self.reg(a) & b,
            Opcode::Borr => self.reg(a) | self.reg(b),
            Opcode::Bori => self.reg(a) | b,
            Opcode::Setr => self.reg(a),
            Opcode::Seti => a,
            Opcode::Gtir => (a > self.reg(b)) as u16,
            Opcode::Gtri => (self.reg(a) > b) as u16,
            Opcode::Gtrr => (self.reg(a) > self.reg(b)) as u16,
            Opcode::Eqir => (a == self.reg(b)) as u16,
            Opcode::Eqri => (self.reg(a) == b) as u16,
            Opcode::Eqrr => (self.reg(a) == self.reg(b)) as u16,
        }
    }
}

struct Sample {
    registers_before: Registers,
    instruction: [u16; 4],
    registers_after: Registers,
}

struct ProblemInput {
    pub samples: Vec<Sample>,
    pub program: Vec<Vec<u16>>,
}

impl ProblemInput {
    fn parse(input_string: &str) -> Result<Self, String> {
        let mut samples = Vec::new();
        let mut registers_before = Registers::of(0, 0, 0, 0);
        let mut instruction: Vec<u16> = Vec::new();
        let mut last_blank = true;
        let mut in_program = false;

        let mut program: Vec<Vec<u16>> = Vec::new();

        for (line_index, line) in input_string.lines().enumerate() {
            let error_mapper = |_| format!("Invalid input at line {}", line_index + 1);
            if line.is_empty() {
                if last_blank {
                    in_program = true;
                } else {
                    last_blank = true;
                }
                continue;
            }
            if in_program {
                let instruction = line
                    .split_whitespace()
                    .map(|n| n.trim().parse::<u16>().map_err(error_mapper))
                    .collect::<Result<_, _>>()?;
                program.push(instruction);
            }

            last_blank = false;

            let before = line.starts_with("Before:");
            let after = line.starts_with("After:");
            if before || after {
                let parts: Vec<u16> = line[9..]
                    .split(|c| c == '[' || c == ']' || c == ',')
                    .filter_map(|s| {
                        if s.is_empty() {
                            None
                        } else {
                            Some(s.trim().parse::<u16>().map_err(|_| "Invalid input"))
                        }
                    })
                    .collect::<Result<_, _>>()?;

                if before {
                    registers_before = Registers::of(parts[0], parts[1], parts[2], parts[3]);
                } else {
                    let registers_after = Registers::of(parts[0], parts[1], parts[2], parts[3]);
                    samples.push(Sample {
                        registers_before,
                        instruction: [
                            instruction[0],
                            instruction[1],
                            instruction[2],
                            instruction[3],
                        ],
                        registers_after,
                    });
                }
            } else {
                instruction = line
                    .split_whitespace()
                    .map(|n| n.trim().parse::<u16>().map_err(error_mapper))
                    .collect::<Result<_, _>>()?;
            }
        }

        if samples.is_empty() {
            return Err("Invalid input - no samples".to_string());
        }
        Ok(Self { samples, program })
    }
}

pub fn solve(input: &mut Input) -> Result<i32, String> {
    let problem_input = ProblemInput::parse(input.text)?;

    let all_opcodes = [
        Opcode::Addr,
        Opcode::Addi,
        Opcode::Mulr,
        Opcode::Muli,
        Opcode::Banr,
        Opcode::Bani,
        Opcode::Borr,
        Opcode::Bori,
        Opcode::Setr,
        Opcode::Seti,
        Opcode::Gtir,
        Opcode::Gtri,
        Opcode::Gtrr,
        Opcode::Eqir,
        Opcode::Eqri,
        Opcode::Eqrr,
    ];

    if input.is_part_one() {
        let mut result = 0;

        for sample in problem_input.samples {
            let mut sum = 0;
            for opcode in all_opcodes.iter() {
                let mut registers_applied = sample.registers_before;
                registers_applied.apply(
                    *opcode,
                    sample.instruction[1],
                    sample.instruction[2],
                    sample.instruction[3],
                );
                if registers_applied == sample.registers_after {
                    sum += 1;
                }
            }
            if sum >= 3 {
                result += 1;
            }
        }
        Ok(result)
    } else {
        let mut possible_meanings: Vec<HashSet<Opcode>> = Vec::new();
        for _ in 0..16 {
            let s: HashSet<Opcode> = all_opcodes.iter().cloned().collect();
            possible_meanings.push(s);
        }

        for sample in problem_input.samples {
            let s: &mut HashSet<Opcode> = &mut possible_meanings[sample.instruction[0] as usize];

            s.retain(|opcode| {
                let mut registers_applied = sample.registers_before;
                registers_applied.apply(
                    *opcode,
                    sample.instruction[1],
                    sample.instruction[2],
                    sample.instruction[3],
                );
                registers_applied == sample.registers_after
            });
        }

        let mut assigned_opcodes = HashSet::new();
        for s in possible_meanings.iter_mut() {
            if s.len() == 1 {
                assigned_opcodes
                    .insert(*s.iter().next().ok_or("Internal error - no elements in s")?);
            }
        }

        let mut new_assign = Vec::new();
        while assigned_opcodes.len() != 16 {
            for new in assigned_opcodes.iter() {
                for s in possible_meanings.iter_mut() {
                    if s.len() > 1 && s.remove(new) && s.len() == 1 {
                        new_assign.push(
                            *s.iter()
                                .next()
                                .ok_or("Internal error - no elements in s for new_assign")?,
                        );
                    }
                }
            }

            for new in new_assign.iter() {
                assigned_opcodes.insert(*new);
            }
        }

        let meanings: Vec<Opcode> = possible_meanings
            .iter()
            .map(|s| {
                s.iter()
                    .next()
                    .copied()
                    .ok_or("Internal error - no elements in s for meanings")
            })
            .collect::<Result<_, _>>()?;

        let mut regs = Registers::of(0, 0, 0, 0);
        for instruction in problem_input.program {
            let opcode = meanings[instruction[0] as usize];
            regs.apply(opcode, instruction[1], instruction[2], instruction[3]);
        }

        Ok(i32::from(regs.values[0]))
    }
}

#[test]
fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!(
            "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]"
        => 1);

    let input = include_str!("day16_input.txt");
    test_part_one!(input => 624);
    test_part_two!(input => 584);
}
