use super::elfcode::Program;

pub fn part1(input_string: &str) -> Result<u64, String> {
    let mut program = Program::parse(input_string)?;
    Ok(program.execute())
}

pub fn part2(input_string: &str) -> Result<u64, String> {
    let mut program = Program::parse(input_string)?;

    program.registers.values[0] = 1;

    program.pretty_print("Initial");
    program.optimize();
    program.pretty_print("Optimized");

    // Assuming som of all factors program starts at instruction 1:
    while program.registers.values[1] == 0 {
        program.execute_one_instruction();
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
fn tests_part1() {
    assert_eq!(
        Ok(7),
        part1(
            "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"
        )
    );

    assert_eq!(Ok(978), part1(include_str!("day19_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(10_996_992), part2(include_str!("day19_input.txt")));
}
