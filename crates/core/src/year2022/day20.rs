use std::collections::VecDeque;

use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<i64, String> {
    #![allow(clippy::unwrap_used)]
    let iterations = input.part_values(1, 10);
    let decryption_key = input.part_values(1, 811_589_153);

    let mut numbers = input
        .text
        .lines()
        .enumerate()
        .map(|(idx, line)| Some((idx, i64::from(line.parse::<i16>().ok()?) * decryption_key)))
        .collect::<Option<VecDeque<_>>>()
        .ok_or_else(|| "Invalid input".to_string())?;

    for original_idx in (0..numbers.len()).cycle().take(numbers.len() * iterations) {
        let current_idx = numbers
            .iter()
            .position(|(idx, _)| *idx == original_idx)
            .unwrap();

        let (_, number) = numbers.remove(current_idx).unwrap();
        numbers.insert(
            (number + current_idx as i64).rem_euclid(numbers.len() as i64) as usize,
            (original_idx, number),
        );
    }

    let zero_idx = numbers.iter().position(|&(_, n)| n == 0).unwrap();
    Ok([1000, 2000, 3000]
        .iter()
        .map(|offset| numbers[(zero_idx + offset) % numbers.len()].1)
        .sum())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "1\n2\n-3\n3\n-2\n0\n4";
    test_part_one!(test_input => 3);
    test_part_two!(test_input => 1_623_178_306);

    let real_input = include_str!("day20_input.txt");
    test_part_one!(real_input => 7_225);
    test_part_two!(real_input => 548_634_267_428);
}
