use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let steps = input
        .text
        .parse::<u32>()
        .map_err(|e| format!("Unable to parse input - {}", e))?;

    // "It starts with a circular buffer containing only the value 0":
    let mut circular_buffer = vec![0];
    // "[..] which it marks as the current position":
    let mut current_position = 0;

    for new_value in 1..2018 {
        // "It then steps forward through the circular buffer some number of
        // steps (your puzzle input) before inserting the first new value, 1,
        // after the value it stopped on. The inserted value becomes the current
        // position. Then, it steps forward from there the same number of steps,
        // and wherever it stops, inserts after it the second new value, 2, and
        // uses that as the new current position again."
        current_position = (current_position + steps + 1) % new_value;
        circular_buffer.insert(current_position as usize + 1, new_value);
    }

    if input.is_part_one() {
        return Ok(circular_buffer[(current_position as usize + 2) % circular_buffer.len()]);
    }

    current_position = 0;
    let mut value_after_zero = 0;
    for new_value in 1..50_000_000 {
        current_position = (current_position + steps + 1) % new_value;
        if current_position == 0 {
            // Since the new value is inserted after the value it stopped on,
            // the value 0 will always be at position 0.
            value_after_zero = new_value;
        }
    }

    Ok(value_after_zero)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day17_input.txt");
    test_part_one!(real_input => 1914);
    test_part_two!(real_input => 41_797_835);
}
