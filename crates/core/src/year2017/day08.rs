use crate::input::Input;
use std::cmp::max;
use std::collections::HashMap;

pub fn solve(input: &mut Input) -> Result<i32, String> {
    let mut registers: HashMap<&str, i32> = HashMap::new();

    let mut highest_value = 0;

    for line in input.text.lines() {
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

    if input.is_part_one() {
        registers
            .values()
            .max()
            .copied()
            .ok_or_else(|| "Internal error".to_string())
    } else {
        Ok(highest_value)
    }
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};
    let real_input = include_str!("day08_input.txt");
    test_part_one!(real_input => 6061);
    test_part_two!(real_input => 6696);
}
