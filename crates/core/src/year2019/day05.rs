use super::int_code::Program;

fn run_with_input(input_string: &str, input: i64) -> String {
    let mut program = Program::parse(input_string);
    program.input(input);
    program.run_for_output().last().unwrap().to_string()
}

pub fn part1(input_string: &str) -> String {
    run_with_input(input_string, 1)
}

pub fn part2(input_string: &str) -> String {
    run_with_input(input_string, 5)
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day05_input.txt")), "15097178");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day05_input.txt")), "1558663");
}
