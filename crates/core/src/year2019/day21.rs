use super::int_code::{Program, Word};
use crate::input::Input;

fn run(intcode_program_string: &str, ascii_program_string: &str) -> Result<Word, String> {
    let mut intcode_program = Program::parse(intcode_program_string)?;
    intcode_program.run_for_output()?;
    intcode_program.input_string(ascii_program_string);

    let program_output = intcode_program.run_for_output()?;
    if let Some(&value) = program_output.iter().find(|&&value| value > 255) {
        Ok(value)
    } else {
        let output_bytes: Vec<u8> = program_output.iter().map(|&value| value as u8).collect();
        let output_string =
            std::str::from_utf8(&output_bytes).map_err(|_| "Output is not utf-8")?;
        Err(format!(
            "No non-ASCII value found - showing last moments:\n{}",
            output_string
        ))
    }
}

pub fn solve(input: &mut Input) -> Result<Word, String> {
    let mut ascii_program = String::new();

    if input.is_part_one() {
        // Jump if there is a hole at A, B or C ...
        ascii_program.push_str("NOT A T\nOR T J\n");
        ascii_program.push_str("NOT B T\nOR T J\n");
        ascii_program.push_str("NOT C T\nOR T J\n");
        // ... AND ground at D:
        ascii_program.push_str("AND D J\n");
        ascii_program.push_str("WALK\n");
    } else {
        // ABCDEFGH
        // ???_?..?
        // Do not jump to D if E and H are holes, since we cannot jump again.
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
    }

    run(input.text, &ascii_program)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};
    let input = include_str!("day21_input.txt");
    test_part_one!(input => 19_358_688);
    test_part_two!(input => 1_141_236_756);
}
