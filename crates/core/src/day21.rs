use crate::int_code::Program;

fn run(intcode_program_string: &str, ascii_program_string: &str) -> String {
    let mut intcode_program = Program::parse(intcode_program_string);
    intcode_program.run_for_output();
    intcode_program.input_string(ascii_program_string);

    let program_output = intcode_program.run_for_output();
    if let Some(value) = program_output.iter().find(|&&value| value > 255) {
        value.to_string()
    } else {
        let output_bytes: Vec<u8> = program_output.iter().map(|&value| value as u8).collect();
        let output_string = std::str::from_utf8(&output_bytes).unwrap();
        panic!(
            "No non-ASCII value found - showing last moments:\n{}",
            output_string
        );
    }
}

pub fn part1(input_string: &str) -> String {
    let mut ascii_program = String::new();
    // Jump if there is a hole at A, B or C ...
    ascii_program.push_str("NOT A T\nOR T J\n");
    ascii_program.push_str("NOT B T\nOR T J\n");
    ascii_program.push_str("NOT C T\nOR T J\n");
    // ... AND ground at D:
    ascii_program.push_str("AND D J\n");
    ascii_program.push_str("WALK\n");

    run(input_string, &ascii_program)
}

pub fn part2(input_string: &str) -> String {
    // ABCDEFGH
    // ???_?..?
    // Do not jump to D if E and H are holes, since we cannot jump again.
    let mut ascii_program = String::new();
    // Jump if hole is in A ...
    ascii_program.push_str("NOT A J\n");
    // ... OR hole at B ...
    ascii_program.push_str("NOT B T\nOR T J\n");
    // ... OR hole at C:
    ascii_program.push_str("NOT C T\nOR T J\n");
    // ... AND ground at E:
    ascii_program.push_str("AND E T\n");
    // ... OR ground at H, so we can go to either E or jump to H:
    ascii_program.push_str("OR H T\n");
    // ... then jump ...
    ascii_program.push_str("AND T J\n");
    // ... if there there is ground at D:
    ascii_program.push_str("AND D J\n");

    ascii_program.push_str("RUN\n");

    run(input_string, &ascii_program)
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day21_input.txt")), "19358688");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day21_input.txt")), "1141236756");
}
