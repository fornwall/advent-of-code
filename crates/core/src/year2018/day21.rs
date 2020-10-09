use super::elfcode::Program;
use std::collections::HashSet;

fn parse(input_string: &str) -> Result<Program, String> {
    let program = Program::parse(input_string)?;
    if program.instructions.len() != 31 {
        return Err("Expected 31 instructions in program".to_string());
    }
    Ok(program)
}

const MAX_INSTRUCTIONS: u64 = 1_000_000;

pub fn part1(input_string: &str) -> Result<u64, String> {
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

    let mut program = parse(input_string)?;
    program.pretty_print("Initial program");
    let mut loop_count = 0;
    while program.instruction_pointer()? != 29 {
        program.execute_one_instruction()?;

        loop_count += 1;
        if loop_count > MAX_INSTRUCTIONS {
            return Err(format!("Aborted after {} instructions", loop_count));
        }
    }
    Ok(program.registers.values[program.instructions[28].a as usize])
}

pub fn part2(input_string: &str) -> Result<u64, String> {
    let mut seen = HashSet::new();
    let mut last_value = 0;
    let mut program = parse(input_string)?;
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

#[test]
fn tests_part1() {
    assert_eq!(Ok(7_216_956), part1(include_str!("day21_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(14_596_916), part2(include_str!("day21_input.txt")));
}
