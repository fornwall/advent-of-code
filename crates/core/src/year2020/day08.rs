use crate::input::Input;

type Word = i32;

#[derive(Copy, Clone)]
pub enum Instruction {
    Acc(Word),
    Jmp(Word),
    Nop(Word),
}

pub struct Computer {
    pub instructions: Vec<Instruction>,
    pub accumulator: Word,
    pub instruction_pointer: Word,
}

impl Computer {
    pub fn parse(program_text: &str) -> Result<Self, String> {
        let instructions = program_text
            .lines()
            .enumerate()
            .map(|(line_idx, line)| {
                if line.len() < 6 {
                    return Err(format!(
                        "Line {}: Too short line ({})",
                        line_idx + 1,
                        line.len()
                    ));
                }

                let argument = line[4..]
                    .parse::<Word>()
                    .map_err(|e| format!("Line {}: Cannot parse argument - {}", line_idx + 1, e))?;

                Ok(match &line[0..3] {
                    "acc" => Instruction::Acc(argument),
                    "jmp" => Instruction::Jmp(argument),
                    "nop" => Instruction::Nop(argument),
                    _ => {
                        return Err(format!(
                            "Line {}: Invalid line not starting with acc/jmp/nop",
                            line_idx + 1
                        ));
                    }
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            instructions,
            accumulator: 0,
            instruction_pointer: 0,
        })
    }

    pub fn execute_instruction(&mut self) -> Result<(), String> {
        if self.has_exited() {}
        match self.instructions.get(self.instruction_pointer as usize) {
            Some(Instruction::Acc(parameter)) => {
                self.accumulator += parameter;
            }
            Some(Instruction::Jmp(parameter)) => {
                self.instruction_pointer += parameter;
                return Ok(());
            }
            Some(Instruction::Nop(_)) => {}
            None => {
                return Err("Cannot executed an exited program".to_string());
            }
        };

        self.instruction_pointer += 1;
        Ok(())
    }

    pub fn has_exited(&self) -> bool {
        self.instruction_pointer < 0 || self.instruction_pointer >= self.instructions.len() as Word
    }
}

struct ComputerChecker {
    executed_instructions: Vec<bool>,
}

impl ComputerChecker {
    fn new(computer: &Computer) -> Self {
        Self {
            executed_instructions: vec![false; computer.instructions.len()],
        }
    }

    fn check_if_exits(&mut self, computer: &mut Computer) -> Result<bool, String> {
        self.executed_instructions
            .iter_mut()
            .for_each(|v| *v = false);

        while !computer.has_exited()
            && !self.executed_instructions[computer.instruction_pointer as usize]
        {
            self.executed_instructions[computer.instruction_pointer as usize] = true;
            computer.execute_instruction()?;
        }
        Ok(computer.has_exited())
    }
}

pub fn solve(input: &Input) -> Result<Word, String> {
    let mut computer = Computer::parse(input.text)?;
    let mut computer_checker = ComputerChecker::new(&computer);

    computer_checker.check_if_exits(&mut computer)?;

    if input.is_part_one() {
        Ok(computer.accumulator)
    } else {
        // We only need to patch instructions that are actually executed in the unpatched program:
        let executed_instructions_without_patch = computer_checker.executed_instructions.clone();

        for i in executed_instructions_without_patch
            .iter()
            .enumerate()
            .filter_map(|(idx, &executed)| if executed { Some(idx) } else { None })
        {
            let instruction = computer.instructions[i];
            match instruction {
                Instruction::Jmp(parameter) | Instruction::Nop(parameter) => {
                    computer.instructions[i] = if matches!(instruction, Instruction::Jmp(_)) {
                        Instruction::Nop(parameter)
                    } else {
                        Instruction::Jmp(parameter)
                    };

                    if computer_checker.check_if_exits(&mut computer)? {
                        return Ok(computer.accumulator);
                    }

                    computer.instruction_pointer = 0;
                    computer.accumulator = 0;
                    computer.instructions[i] = instruction;
                }
                _ => {
                    continue;
                }
            }
        }
        Err("No instruction modification causes program to exit".to_string())
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    test_part_one!(example => 5);
    test_part_two!(example => 8);

    let real_input = include_str!("day08_input.txt");
    test_part_one!(real_input => 1684);
    test_part_two!(real_input => 2188);
}
