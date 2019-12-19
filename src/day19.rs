use crate::int_code::Program;

pub fn part1(input_string: &str) -> String {
    let initial_program = Program::parse(input_string);
    let mut result = 0;
    for x in 0..50 {
        for y in 0..50 {
            let mut program = initial_program.clone();
            program.input(x);
            program.input(y);
            program.run();
            if program.output_values[0] == 1 {
                result += 1;
            }
        }
    }

    result.to_string()
}

pub fn part2(_input_string: &str) -> String {
    String::from("0")
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day19_input.txt")), "112");
}

#[test]
fn tests_part2() {
    //assert_eq!(part2(include_str!("day19_input.txt")), "");
}
