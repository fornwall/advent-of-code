use super::int_code::{Program, Word};
use crate::input::Input;

fn check(program: &Program, x: i32, y: i32) -> Result<bool, String> {
    let mut program_copy = program.clone();
    program_copy.input(Word::from(x));
    program_copy.input(Word::from(y));
    let output = program_copy.run_for_output()?;
    if output.is_empty() {
        return Err("No output produced".to_string());
    } else if output.len() != 1 || !matches!(output[0], 0 | 1) {
        return Err("Invalid output from program (expected only 0 or 1)".to_string());
    }
    Ok(output[0] == 1)
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let program = Program::parse(input.text)?;

    Ok(if input.is_part_one() {
        let mut result = 0;
        for (x, y) in (0..50).flat_map(|x| (0..50).map(move |y| (x, y))) {
            let checked = check(&program, x, y)?;
            if checked {
                result += 1;
            }
        }
        result
    } else {
        // Find the initial start of the beam (skipping (0,0),
        // as it is not connected to rest of the beam.
        let mut current_x = 0;
        let mut current_y = 0;
        for (y, x) in (1..50).flat_map(|x| (0..50).map(move |y| (x, y))) {
            let checked = check(&program, x, y)?;
            if checked && current_x == 0 {
                current_x = x;
                current_y = y;
            }
        }

        // Track the top right of the beam as long as square does not fit.
        while !check(&program, current_x - 99, current_y + 99)? {
            current_y += 1;
            while check(&program, current_x + 1, current_y)? {
                current_x += 1;
            }
        }

        ((current_x - 99) * 10000 + current_y) as u32
    })
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let input = include_str!("day19_input.txt");
    test_part_one!(input => 112);
    test_part_two!(input => 18_261_982);
}
