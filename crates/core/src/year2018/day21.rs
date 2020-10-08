use super::elfcode::Program;
use std::collections::HashSet;

pub fn part1(input_string: &str) -> Result<u64, String> {
    // The last three instructions are:
    //
    // 28: eqrr 3 0 4
    // 29: addr 4 1 1
    // 30: seti 5 9 1
    //
    // or (since the instruction pointer is incremented before executing the next instruction):
    //
    // 28: r4 = (r0 == r3)
    // 29: goto 30 + r4
    // 30: goto 6
    //
    // which exits on instruction 29 only if r4 is non-zero, which means r0 must equal r3.
    //
    // Since this is the only place in the program where register 0 is referenced, we can
    // set register 0 to the value it's first compared with here to exit as soon as possible.

    let mut program = Program::parse(input_string)?;
    const MAX_INSTRUCTIONS: i32 = 1_000_000;
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
    let mut program = Program::parse(input_string)?;
    const MAX_INSTRUCTIONS: u64 = 10_000_000_000;
    let mut loop_count = 0;
    loop {
        if program.instruction_pointer()? == 29 {
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
