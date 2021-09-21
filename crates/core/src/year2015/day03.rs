use crate::Input;
use std::collections::HashSet;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut visited_houses = HashSet::new();

    let mut santa_position = (0, 0);
    visited_houses.insert(santa_position);

    let mut robo_santa_position = (0, 0);

    for (idx, c) in input.text.chars().enumerate() {
        let mover = if input.is_part_one() || idx % 2 == 0 {
            &mut santa_position
        } else {
            &mut robo_santa_position
        };
        match c {
            '>' => {
                mover.0 += 1;
            }
            '<' => {
                mover.0 -= 1;
            }
            '^' => {
                mover.1 -= 1;
            }
            'v' => {
                mover.1 += 1;
            }
            _ => {
                return Err(format!("Invalid input char '{}'", c));
            }
        }

        visited_houses.insert(*mover);
    }

    Ok(visited_houses.len() as u32)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day03_input.txt");
    test_part_one!(real_input => 2572);
    test_part_two!(real_input => 2631);
}
