use super::int_code::Program;
use crate::Input;

pub fn solve(input: &mut Input) -> Result<String, String> {
    let mut program = Program::parse(input.text)?;
    program.input(input.part_values(1, 2));

    let output = program.run_for_output()?;
    Ok(output
        .iter()
        .map(|&value| value.to_string())
        .collect::<Vec<String>>()
        .join(","))
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!("104,1125899906842624,99" => "1125899906842624".into());
    test_part_one!("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99" => "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".into());
    test_part_one!("1102,34915192,34915192,7,4,7,99,0" => "1219070632396864".into());

    let input = include_str!("day09_input.txt");
    test_part_one!(input => "3601950151".into());
    test_part_two!(input => "64236".into());
}
