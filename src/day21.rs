use crate::int_code::Program;

// Three instructions: AND, OR and NOT.
// Six possible first registers: A, B, C, D, T and J.
// Two possible second registers: T, J.
// Possibilities: 3*6*2 = 36.
pub fn part1(input_string: &str) -> String {
    let mut intcode_program = Program::parse(input_string);
    intcode_program.run();
    intcode_program.output_values.clear();

    let mut ascii_program = String::new();
    ascii_program.push_str("NOT A T\nOR T J\n");
    ascii_program.push_str("NOT B T\nOR T J\n");
    ascii_program.push_str("NOT C T\nOR T J\n");
    ascii_program.push_str("AND D J\n");
    ascii_program.push_str("WALK\n");

    intcode_program.input_string(ascii_program.as_str());
    intcode_program.run();

    intcode_program
        .output_values
        .iter()
        .find(|&&value| value > 255)
        .unwrap()
        .to_string()
}

pub fn part2(input_string: &str) -> String {
    let mut intcode_program = Program::parse(input_string);
    intcode_program.run();
    intcode_program.output_values.clear();

    let mut ascii_program = String::new();
    ascii_program.push_str("NOT A T\nOR T J\n");
    ascii_program.push_str("NOT B T\nOR T J\n");
    ascii_program.push_str("NOT C T\nOR T J\n");
    ascii_program.push_str("AND D J\n");
    ascii_program.push_str("RUN\n");

    intcode_program.input_string(ascii_program.as_str());
    intcode_program.run();

    if let Some(value) = intcode_program
        .output_values
        .iter()
        .find(|&&value| value > 255)
    {
        return value.to_string();
    } else {
        let u8_array: Vec<u8> = intcode_program
            .output_values
            .iter()
            .map(|&value| value as u8)
            .collect();
        println!("{}", std::str::from_utf8(&u8_array).unwrap());
    }
    String::from("")
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day21_input.txt")), "19358688");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day21_input.txt")), "");
}
