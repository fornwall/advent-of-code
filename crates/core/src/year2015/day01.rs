use crate::input::Input;

pub fn solve(input: &Input) -> Result<i32, String> {
    let mut floor = 0;
    for (idx, c) in input.text.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => {
                return Err(format!("Invalid char at offset {}: '{}'", idx, c));
            }
        };
        if input.is_part_two() && floor == -1 {
            return Ok(idx as i32 + 1);
        }
    }
    Ok(floor)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day01_input.txt");
    test_part_one!(real_input => 280);
    test_part_two!(real_input => 1797);
}
