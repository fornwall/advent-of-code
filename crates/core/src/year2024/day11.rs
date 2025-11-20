use std::collections::HashMap;

use crate::common::array_stack::ArrayStack;
use crate::input::{Input, on_error};

// Based on https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2024/day11.rs
pub fn solve(input: &Input) -> Result<u64, String> {
    let mut stone_evolution_by_idx = ArrayStack::<5000, (u16, Option<u16>)>::new();
    let mut stone_value_to_idx = HashMap::with_capacity(5000);
    let mut stone_values_to_process = Vec::new();
    let mut occurences_by_idx = [0_u64; 5000];

    for s in input.text.split_ascii_whitespace() {
        let stone_value: u64 = s.parse().map_err(|_| on_error())?;
        let indices_len = stone_value_to_idx.len() as u16;
        let index = *stone_value_to_idx.entry(stone_value).or_insert_with(|| {
            stone_values_to_process.push(stone_value);
            indices_len
        });
        occurences_by_idx[index as usize] += 1;
    }

    for _ in 0..input.part_values(25, 75) {
        let mut next_stone_values_to_process = Vec::with_capacity(200);
        let mut next_occurences_by_idx = [0; 5000];

        let mut index_of = |stone_value| {
            let size = stone_value_to_idx.len() as u16;
            *stone_value_to_idx.entry(stone_value).or_insert_with(|| {
                next_stone_values_to_process.push(stone_value);
                size
            })
        };

        for &stone_value in stone_values_to_process.iter() {
            let (left, right) = evolve_stone(stone_value);
            stone_evolution_by_idx.push((index_of(left), right.map(&mut index_of)))?;
        }

        for (&(first_idx, second_idx), amount) in
            stone_evolution_by_idx.slice().iter().zip(occurences_by_idx)
        {
            next_occurences_by_idx[first_idx as usize] += amount;
            if let Some(second_idx) = second_idx {
                next_occurences_by_idx[second_idx as usize] += amount;
            }
        }

        occurences_by_idx = next_occurences_by_idx;
        stone_values_to_process = next_stone_values_to_process;
    }

    Ok(occurences_by_idx.iter().sum())
}

fn evolve_stone(stone_value: u64) -> (u64, Option<u64>) {
    let num_digits = num_digits(stone_value);
    if num_digits.is_multiple_of(2) {
        let raised = 10_u64.pow(num_digits / 2);
        let left_value = stone_value / raised;
        let right_value = stone_value % raised;
        (left_value, Some(right_value))
    } else {
        let new_stone_value = if stone_value == 0 {
            1
        } else {
            stone_value * 2024
        };
        (new_stone_value, None)
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
