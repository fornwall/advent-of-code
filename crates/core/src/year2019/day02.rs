use super::int_code::Program;
use super::int_code::Word;
use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<Word, String> {
    const DESIRED_OUTPUT: Word = 19_690_720;

    let initial_program = Program::parse(input.text)?;

    for noun in input.part_values(12..=12, 0..=99) {
        for verb in input.part_values(2..=2, 0..=99) {
            let mut program = initial_program.clone();
            program.write_memory(1, noun);
            program.write_memory(2, verb);
            program.run_until_halt_or_input(10_000)?;
            let memory_value = program.read_memory(0);
            if input.is_part_one() {
                return Ok(memory_value);
            } else if memory_value == DESIRED_OUTPUT {
                return Ok(100 * noun + verb);
            }
        }
    }

    Err(format!(
        "Desired output {} is never produced",
        DESIRED_OUTPUT
    ))
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};

    let solution = include_str!("day02_input.txt");
    test_part_one!(solution => 4_570_637);
    test_part_two!(solution => 5485);

    test_part_one_error!( "hi" => "Unable to parse program word (invalid digit found in string)");
}
