use std::collections::HashMap;

use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut stones = HashMap::with_capacity(4096);
    for s in input.text.split_ascii_whitespace() {
        let stone_value: u64 = s.parse().map_err(|_| on_error())?;
        *stones.entry(stone_value).or_insert(0) += 1;
    }
    for _ in 0..input.part_values(25, 75) {
        stones = blink_evolve(stones);
    }
    Ok(stones.values().sum())
}

fn blink_evolve(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut evolved = HashMap::with_capacity(stones.len());
    for (&stone_value, &num_stones) in stones.iter() {
        if stone_value == 0 {
            // "If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1."
            *evolved.entry(1).or_insert(0) += num_stones;
        } else if let Some((left_value, right_value)) = split_if_even_num_digits(stone_value) {
            // "If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
            // The left half of the digits are engraved on the new left stone, and the right half of the digits are
            // engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become
            // stones 10 and 0.)."
            *evolved.entry(left_value).or_insert(0) += num_stones;
            *evolved.entry(right_value).or_insert(0) += num_stones;
        } else {
            // "If none of the other rules apply, the stone is replaced by a new stone; the old stone's number
            // multiplied by 2024 is engraved on the new stone.""
            let new_value = stone_value * 2024;
            *evolved.entry(new_value).or_insert(0) += num_stones;
        }
    }
    evolved
}

fn split_if_even_num_digits(number: u64) -> Option<(u64, u64)> {
    let num_digits = num_digits(number);
    if num_digits % 2 == 0 {
        let raised = 10_u64.pow(num_digits / 2);
        let left_value = number / raised;
        let right_value = number % raised;
        Some((left_value, right_value))
    } else {
        None
    }
}

fn num_digits(number: u64) -> u32 {
    number.checked_ilog10().unwrap_or_default() + 1
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "125 17";
    test_part_one!(test_input => 55312);

    let real_input = include_str!("day11_input.txt");
    test_part_one!(real_input => 220_722);
    test_part_two!(real_input => 261_952_051_690_787);
}
