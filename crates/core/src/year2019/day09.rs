use super::int_code::Program;

fn output_from_input(input_string: &str, input: i64) -> Result<String, String> {
    let mut program = Program::parse(input_string);
    program.input(input);

    Ok(program
        .run_for_output()
        .iter()
        .map(|&value| value.to_string())
        .collect::<Vec<String>>()
        .join(","))
}

pub fn part1(input_string: &str) -> Result<String, String> {
    output_from_input(input_string, 1)
}

pub fn part2(input_string: &str) -> Result<String, String> {
    output_from_input(input_string, 2)
}

#[test]
pub fn tests_part1() {
    assert_eq!(
        part1("104,1125899906842624,99").unwrap(),
        "1125899906842624"
    );
    assert_eq!(
        part1("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99").unwrap(),
        "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
    );
    assert_eq!(
        part1("1102,34915192,34915192,7,4,7,99,0").unwrap(),
        "1219070632396864"
    );

    assert_eq!(
        part1(include_str!("day09_input.txt")).unwrap(),
        "3601950151"
    );
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day09_input.txt")).unwrap(), "64236");
}
