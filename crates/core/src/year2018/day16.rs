use super::elfcode::{Opcode, Registers};
use crate::Input;
use std::collections::HashSet;

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
        let mut registers_before = Registers::new();
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
                    for (i, &value) in parts.iter().enumerate() {
                        registers_before.values[i] = u64::from(value);
                    }
                } else {
                    let mut registers_after = Registers::new();
                    for (i, &value) in parts.iter().enumerate() {
                        registers_after.values[i] = u64::from(value);
                    }
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

pub fn solve(input: &mut Input) -> Result<u64, String> {
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
                    u64::from(sample.instruction[1]),
                    u64::from(sample.instruction[2]),
                    u64::from(sample.instruction[3]),
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
                    u64::from(sample.instruction[1]),
                    u64::from(sample.instruction[2]),
                    u64::from(sample.instruction[3]),
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

        let mut regs = Registers::new();
        for instruction in problem_input.program {
            let opcode = meanings[instruction[0] as usize];
            regs.apply(
                opcode,
                u64::from(instruction[1]),
                u64::from(instruction[2]),
                u64::from(instruction[3]),
            );
        }

        Ok(regs.values[0])
    }
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!(
            "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]"
        => 1);

    let input = include_str!("day16_input.txt");
    test_part_one!(input => 624);
    test_part_two!(input => 584);
}
