use crate::common::parser::parse_lines;
use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    Ok(parse_lines::<u32>(input.text)?
        .windows(input.part_values(2, 4))
        .filter(|data| data.last() > data.first())
        .count())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day01_input.txt");
    test_part_one!(real_input => 1766);
    test_part_two!(real_input => 1797);
}
