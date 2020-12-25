use crate::input::Input;
use std::collections::HashSet;

pub fn solve(input: &mut Input) -> Result<String, String> {
    let number_of_cups = input.part_values(9, 1_000_000);
    let max_cup_value = number_of_cups;
    let crab_moves = input.part_values(100, 10_000_000);

    let input_bytes = input.text.as_bytes();
    let input_cup_values = if input_bytes.len() != 9 {
        return Err(format!(
            "Invalid input: Expected 9 characters as input, was {}",
            input_bytes.len()
        ));
    } else if !input_bytes.iter().all(u8::is_ascii_digit) {
        return Err("Invalid input: Not all characters are digits".to_string());
    } else if input_bytes.iter().collect::<HashSet<_>>().len() == 9 {
        input_bytes
            .iter()
            .map(|b| u32::from(b - b'0'))
            .collect::<Vec<u32>>()
    } else {
        return Err("Invalid input: Not 9 distinct digits".to_string());
    };

    // Indexed by cup values, the array contains information about the circuler
    // structure - the next cup value in the circle:
    let mut cups: Vec<u32> = vec![0; number_of_cups as usize + 1];

    for (input_idx, &cup_value) in input_cup_values.iter().enumerate() {
        cups[cup_value as usize] = if input.is_part_one() || input_idx < 8 {
            input_cup_values[(input_idx + 1) % 9]
        } else {
            10
        };
    }

    if input.is_part_two() {
        for cup_value in 10..=number_of_cups {
            cups[cup_value as usize] = if cup_value == max_cup_value {
                input_cup_values[0]
            } else {
                cup_value + 1
            };
        }
    }

    let mut current_cup_value = input_cup_values[0];

    for _move in 0..crab_moves {
        let pickup_1 = cups[current_cup_value as usize];
        let pickup_2 = cups[pickup_1 as usize];
        let pickup_3 = cups[pickup_2 as usize];

        let mut destination_cup = if current_cup_value == 1 {
            max_cup_value
        } else {
            current_cup_value - 1
        };
        while destination_cup == pickup_1
            || destination_cup == pickup_2
            || destination_cup == pickup_3
        {
            destination_cup = if destination_cup == 1 {
                max_cup_value
            } else {
                destination_cup - 1
            };
        }

        // Pick up the three cups following the current one:
        let (before_picked_up_sequence, after_picked_up_sequence) =
            (current_cup_value, cups[pickup_3 as usize]);
        cups[before_picked_up_sequence as usize] = after_picked_up_sequence;

        // Insert the picked up sequence after the destination cup:
        let after_destination_cup = cups[destination_cup as usize];
        cups[destination_cup as usize] = pickup_1;
        cups[pickup_3 as usize] = after_destination_cup;

        current_cup_value = cups[current_cup_value as usize];
    }

    let cup_after_one_value = cups[1];
    Ok(if input.is_part_one() {
        let mut result_string = String::new();
        let mut current_cup_value = cup_after_one_value;
        while (result_string.len() as u32) < number_of_cups - 1 {
            result_string.push((current_cup_value as u8 + b'0') as char);
            current_cup_value = cups[current_cup_value as usize];
        }
        result_string
    } else {
        let cup_after_that_value = cups[cup_after_one_value as usize];
        (u64::from(cup_after_one_value) * u64::from(cup_after_that_value)).to_string()
    })
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "389125467";
    test_part_one!(example => "67384529".to_string());
    test_part_two!(example => "149245887792".to_string());

    let real_input = include_str!("day23_input.txt");
    test_part_one!(real_input => "65432978".to_string());
    test_part_two!(real_input => "287230227046".to_string());
}
