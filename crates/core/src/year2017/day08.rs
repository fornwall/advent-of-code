use std::cmp::max;
use std::collections::HashMap;

fn solution(input_string: &str, part1: bool) -> Result<i32, String> {
    let mut registers: HashMap<&str, i32> = HashMap::new();

    let mut highest_value = 0;

    for line in input_string.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() != 7 {
            return Err("Invalid input - every line should have 7 words".to_string());
        }

        let condition_register = parts[4];
        let &condition_value = registers.get(condition_register).unwrap_or(&0);
        let comparison = parts[5];
        let compared_with = parts[6]
            .parse::<i32>()
            .map_err(|_| "Invalid TODO".to_string())?;

        if match comparison {
            ">" => condition_value > compared_with,
            "<" => condition_value < compared_with,
            ">=" => condition_value >= compared_with,
            "==" => condition_value == compared_with,
            "<=" => condition_value <= compared_with,
            "!=" => condition_value != compared_with,
            _ => {
                return Err(format!("Unknown comparison {}", comparison));
            }
        } {
            let target_register = parts[0];
            let current_value = registers.entry(target_register).or_insert(0);
            let change = parts[2]
                .parse::<i32>()
                .map_err(|_| "Invalid input".to_string())?
                * if parts[1] == "inc" { 1 } else { -1 };
            *current_value += change;
            highest_value = max(highest_value, *current_value);
        }
    }

    if part1 {
        registers
            .values()
            .max()
            .copied()
            .ok_or_else(|| "Internal error".to_string())
    } else {
        Ok(highest_value)
    }
}

pub fn part1(input_string: &str) -> Result<i32, String> {
    solution(input_string, true)
}

pub fn part2(input_string: &str) -> Result<i32, String> {
    solution(input_string, false)
}

#[test]
fn test_part1() {
    assert_eq!(Ok(6061), part1(include_str!("day08_input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(Ok(6696), part2(include_str!("day08_input.txt")));
}
