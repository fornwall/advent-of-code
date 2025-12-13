use crate::input::{Input, Part};

pub fn solve(input: &Input) -> Result<i32, String> {
    let mut horizontal_position = 0_i32;
    let mut depth = 0_i32;
    let mut aim = 0_i32;

    for (line_idx, line) in input.text.lines().enumerate() {
        match line
            .split_once(' ')
            .map(|(d, x)| (d, x.parse::<i16>().map(i32::from), input.part))
        {
            Some(("forward", Ok(amount), _)) => {
                horizontal_position += amount;
                depth += aim * amount;
            }
            Some(("down", Ok(amount), Part::One)) => {
                depth += amount;
            }
            Some(("down", Ok(amount), Part::Two)) => {
                aim += amount;
            }
            Some(("up", Ok(amount), Part::One)) => {
                depth -= amount;
            }
            Some(("up", Ok(amount), Part::Two)) => {
                aim -= amount;
            }
            _ => {
                return Err(format!("Line {}: Invalid format", line_idx + 1));
            }
        }
    }
    Ok(depth * horizontal_position)
}

#[test]
pub fn tests() {
    let real_input = include_str!("day02_input.txt");
    test_part_one!(real_input => 1_693_300);
    test_part_two!(real_input => 1_857_958_050);
}
