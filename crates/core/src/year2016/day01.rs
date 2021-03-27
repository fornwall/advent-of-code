use crate::Input;
use std::collections::HashSet;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    const fn turn(direction: (i32, i32), right: bool) -> (i32, i32) {
        if right {
            (-direction.1, direction.0)
        } else {
            (direction.1, -direction.0)
        }
    }

    let mut visited_locations = HashSet::new();
    let mut position = (0, 0);
    let mut direction = (0, -1);

    'outer: for part in input.text.split(", ") {
        let on_error = || "Invalid input".to_string();
        if part.len() < 2 {
            return Err(on_error());
        }

        let (turn_str, number_str) = part.split_at(1);
        let number = number_str.parse::<i32>().map_err(|_| on_error())?;
        match turn_str {
            "L" => {
                direction = turn(direction, false);
            }
            "R" => {
                direction = turn(direction, true);
            }
            _ => {
                return Err(on_error());
            }
        }

        for _ in 0..number {
            if input.is_part_two() && !visited_locations.insert(position) {
                break 'outer;
            }

            position = (position.0 + direction.0, position.1 + direction.1);
        }
    }

    Ok((position.0.abs() + position.1.abs()) as u32)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day01_input.txt");
    test_part_one!(real_input => 239);
    test_part_two!(real_input => 141);
}
