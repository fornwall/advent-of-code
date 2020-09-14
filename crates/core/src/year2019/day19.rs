use super::int_code::Program;

fn check(program: &Program, x: i32, y: i32) -> bool {
    let mut program_copy = program.clone();
    program_copy.input(x as i64);
    program_copy.input(y as i64);
    program_copy.run_for_output()[0] == 1
}

pub fn part1(input_string: &str) -> Result<usize, String> {
    let program = Program::parse(input_string)?;

    Ok((0..50)
        .flat_map(|x| (0..50).map(move |y| (x, y)))
        .filter(|&(x, y)| check(&program, x, y))
        .count())
}

pub fn part2(input_string: &str) -> Result<i32, String> {
    let program = Program::parse(input_string)?;

    // Find the initial start of the beam (skipping (0,0),
    // as it is not connected to rest of the beam.
    let (mut current_x, mut current_y) = (1..50)
        .flat_map(|x| (0..50).map(move |y| (x, y)))
        .find(|&(x, y)| check(&program, x, y))
        .unwrap();

    // Track the top right of the beam as long as square does not fit.
    while !check(&program, current_x - 99, current_y + 99) {
        current_y += 1;
        if check(&program, current_x + 1, current_y) {
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
    assert_eq!(part2(include_str!("day19_input.txt")), Ok(18261982));
}
