use super::int_code::{Program, Word};

fn check(program: &Program, x: i32, y: i32) -> Result<bool, String> {
    let mut program_copy = program.clone();
    program_copy.input(x as Word);
    program_copy.input(y as Word);
    let output = program_copy.run_for_output()?;
    if output.is_empty() {
        return Err("No output produced".to_string());
    } else if output.len() != 1 || !matches!(output[0], 0 | 1) {
        return Err("Invalid output from program (expected only 0 or 1)".to_string());
    }
    Ok(output[0] == 1)
}

pub fn part1(input_string: &str) -> Result<usize, String> {
    let program = Program::parse(input_string)?;

    let mut result = 0;
    for (x, y) in (0..50).flat_map(|x| (0..50).map(move |y| (x, y))) {
        let checked = check(&program, x, y)?;
        if checked {
            result += 1;
        }
    }
    Ok(result)
}

pub fn part2(input_string: &str) -> Result<i32, String> {
    let program = Program::parse(input_string)?;

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

    Ok((current_x - 99) * 10000 + current_y)
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day19_input.txt")), Ok(112));
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day19_input.txt")), Ok(18_261_982));
}
