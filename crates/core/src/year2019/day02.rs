use super::int_code::Program;
use super::int_code::Word;

fn solution(input_string: &str, part1: bool) -> Result<Word, String> {
    const DESIRED_OUTPUT: Word = 19_690_720;

    let initial_program = Program::parse(input_string)?;

    for noun in if part1 { 12..=12 } else { 0..=99 } {
        for verb in if part1 { 2..=2 } else { 0..=99 } {
            let mut program = initial_program.clone();
            program.write_memory(1, noun);
            program.write_memory(2, verb);
            program.run_until_halt_or_input(10_000)?;
            let memory_value = program.read_memory(0);
            if part1 {
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

pub fn part1(input_string: &str) -> Result<Word, String> {
    solution(input_string, true)
}

pub fn part2(input_string: &str) -> Result<Word, String> {
    solution(input_string, false)
}

#[test]
pub fn tests_part1() {
    assert_eq!(Ok(4_570_637), part1(include_str!("day02_input.txt")));

    assert_eq!(
        Err("Unable to parse program word: invalid digit found in string".to_string()),
        part1("hi")
    );
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(5485), part2(include_str!("day02_input.txt")));
}
