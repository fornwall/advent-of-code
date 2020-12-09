use super::computer::{Computer, Instruction, Word};
#[cfg(feature = "visualization")]
use super::day08_renderer::{render, start_rendering};
use crate::input::Input;
#[cfg(feature = "visualization")]
use crate::painter::PainterRef;

struct ComputerChecker {
    executed_instructions: Vec<bool>,
}

impl ComputerChecker {
    fn new(computer: &Computer) -> Self {
        Self {
            executed_instructions: vec![false; computer.instructions.len()],
        }
    }

    fn check_if_exits(
        &mut self,
        computer: &mut Computer,
        #[cfg(feature = "visualization")] mut painter: &mut PainterRef,
        #[cfg(feature = "visualization")] switched_instruction_idx: Option<usize>,
    ) -> Result<bool, String> {
        self.executed_instructions
            .iter_mut()
            .for_each(|v| *v = false);

        while !computer.has_exited()
            && !self.executed_instructions[computer.instruction_pointer as usize]
        {
            self.executed_instructions[computer.instruction_pointer as usize] = true;
            computer.execute_instruction()?;
        }
        #[cfg(feature = "visualization")]
        render(
            &mut painter,
            &computer,
            &self.executed_instructions,
            switched_instruction_idx,
        );
        Ok(computer.has_exited())
    }
}

pub fn solve(input: &mut Input) -> Result<Word, String> {
    let mut computer = Computer::parse(input.text)?;
    let mut computer_checker = ComputerChecker::new(&computer);

    #[cfg(feature = "visualization")]
    start_rendering(&mut input.painter);

    computer_checker.check_if_exits(
        &mut computer,
        #[cfg(feature = "visualization")]
        &mut input.painter,
        #[cfg(feature = "visualization")]
        None,
    )?;

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

                    if computer_checker.check_if_exits(
                        &mut computer,
                        #[cfg(feature = "visualization")]
                        &mut input.painter,
                        #[cfg(feature = "visualization")]
                        Some(i),
                    )? {
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
    use crate::{test_part_one, test_part_two};

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
