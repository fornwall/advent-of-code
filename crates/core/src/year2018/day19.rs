use super::elfcode::Program;
use crate::Input;

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut program = Program::parse(input.text)?;

    if input.is_part_one() {
        return program.execute_until_halt(10_000_000);
    }

    program.registers.values[0] = 1;

    #[cfg(feature = "debug-output")]
    program.pretty_print("Initial");
    program.optimize();
    #[cfg(feature = "debug-output")]
    program.pretty_print("Optimized");

    if program.instructions.len() < 3 {
        return Err("Too few instructions".to_string());
    }
    let register = program.instructions[2].c as usize;
    if register > 5 {
        return Err("Register outside bounds".to_string());
    }
    while program.registers.values[register] == 0 {
        program.execute_one_instruction()?;
    }

    let mut sum = 0;
    // Assuming number to be factored is highest register value:
    let &seed = program
        .registers
        .values
        .iter()
        .max()
        .ok_or("Internal error: No registers")?;
    for i in 1..=seed {
        if seed % i == 0 {
            sum += i;
        }
    }
    Ok(sum)
}

#[test]
fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!(
            "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5" => 7);

    let input = include_str!("day19_input.txt");
    test_part_one!(input => 978);
    test_part_two!(input => 10_996_992);
}
