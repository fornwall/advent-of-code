use super::int_code::Program;
use super::int_code::Word;

fn run_with_input(input_string: &str, input: Word) -> Result<Word, String> {
    let mut program = Program::try_parse(input_string)?;
    program.input(input);
    program
        .run_for_output()
        .last()
        .ok_or_else(|| "No output produced".to_string())
        .map(|word| *word)
}

pub fn part1(input_string: &str) -> Result<Word, String> {
    run_with_input(input_string, 1)
}

pub fn part2(input_string: &str) -> Result<Word, String> {
    run_with_input(input_string, 5)
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day05_input.txt")), Ok(15097178));
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day05_input.txt")), Ok(1558663));
}
