use crate::input::Input;
use std::cmp::Ordering;

fn parse_input(input_string: &str) -> Result<(i32, i32), String> {
    let (from_str, to_str) = input_string
        .trim()
        .split_once('-')
        .ok_or_else(|| "Invalid parts - should be in FROM-TO form".to_string())?;
    let from = from_str.parse::<i32>().or(Err("Invalid range"))?;
    let to = to_str.parse::<i32>().or(Err("Invalid range"))?;
    Ok((from, to))
}

pub fn solve(input: &Input) -> Result<u32, String> {
    let (from, to) = parse_input(input.text)?;

    let mut meeting_criteria_count: u32 = 0;
    'outer: for i in from..=to {
        let mut divider = 1;
        let mut last_digit = 10;
        let mut two_digits_adjacent = false;

        let mut digits_adjacent_streak = 1;

        while divider <= 100_000 {
            let digit = (i / divider) % 10;
            match digit.cmp(&last_digit) {
                Ordering::Greater => {
                    continue 'outer;
                }
                Ordering::Equal => {
                    if input.is_part_one() {
                        two_digits_adjacent = true;
                    } else {
                        digits_adjacent_streak += 1;
                    }
                }
                Ordering::Less => {
                    if digits_adjacent_streak == 2 {
                        two_digits_adjacent = true;
                    }
                    digits_adjacent_streak = 1;
                }
            }

            last_digit = digit;
            divider *= 10;
        }

        if digits_adjacent_streak == 2 {
            two_digits_adjacent = true;
        }

        if two_digits_adjacent {
            meeting_criteria_count += 1;
        }
    }

    Ok(meeting_criteria_count)
}

#[test]
pub fn tests() {
    test_part_one!("100010-100011" => 0);
    test_part_one!("111110-111111" => 1);

    test_part_two!("112233-112233" => 1);
    test_part_two!("123444-123444" => 0);
    test_part_two!("111122-111122" => 1);

    let input = include_str!("day04_input.txt");
    test_part_one!(input => 1675);
    test_part_two!(input => 1142);
}
