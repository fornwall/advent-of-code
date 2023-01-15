use super::elfcode::Program;
use crate::input::Input;
use std::collections::HashSet;

fn parse(input_string: &str) -> Result<Program, String> {
    let program = Program::parse(input_string)?;
    if program.instructions.len() != 31 {
        return Err("Expected 31 instructions in program".to_string());
    }
    Ok(program)
}

const MAX_INSTRUCTIONS: u64 = 1_000_000;

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut program = parse(input.text)?;
    if input.is_part_one() {
        // The last three instructions are (as seen with program.pretty_print()):
        //
        // 28: r4 = (r3 == r0) ? 1 : 0
        // 29: goto r4 + 30
        // 30: goto 6
        //
        // which exits on instruction 29 only if r4 is non-zero, which means r0 must equal r3.
        //
        // Since this is the only place in the program where register 0 is referenced, we can
        // set register 0 to the value it's first compared with here to exit as soon as possible.

        let mut loop_count = 0;
        while program.instruction_pointer()? != 29 {
            program.execute_one_instruction()?;

            loop_count += 1;
            if loop_count > MAX_INSTRUCTIONS {
                return Err(format!("Aborted after {} instructions", loop_count));
            }
        }
        Ok(program.registers.values[program.instructions[28].a as usize])
    } else {
        let mut seen = HashSet::new();
        let mut last_value = 0;
        let mut loop_count = 0;
        loop {
            let ip = program.instruction_pointer()?;
            if ip == 14 {
                if program.registers.values[program.instructions[13].c as usize] == 0 {
                    program.registers.values[program.instructions[6].c as usize] /= 256;
                    program.registers.values[program.instruction_pointer_index as usize] = 8;
                }
            } else if ip == 29 {
                let value = program.registers.values[program.instructions[28].a as usize];
                if seen.insert(value) {
                    last_value = value;
                } else {
                    return Ok(last_value);
                }
            }
            program.execute_one_instruction()?;

            loop_count += 1;
            if loop_count > MAX_INSTRUCTIONS {
                return Err(format!("Aborted after {} instructions", loop_count));
            }
        }
    }
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};
    let input = include_str!("day21_input.txt");
    test_part_one!(input => 7_216_956);
    test_part_two!(input => 14_596_916);
}
