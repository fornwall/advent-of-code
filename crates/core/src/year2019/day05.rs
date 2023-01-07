use super::int_code::Program;
use super::int_code::Word;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<Word, String> {
    let mut program = Program::parse(input.text)?;
    program.input(input.part_values(1, 5));
    let output = program.run_for_output()?;
    output
        .last()
        .ok_or_else(|| "No output produced".to_string())
        .map(|word| *word)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};
    let input = include_str!("day05_input.txt");
    test_part_one!(input => 15_097_178);
    test_part_two!(input => 1_558_663);
}
