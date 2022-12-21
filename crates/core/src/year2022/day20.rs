use std::collections::VecDeque;

use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<i64, String> {
    #![allow(clippy::unwrap_used)]
    let iterations = input.part_values(1, 10);
    let decryption_key = input.part_values(1, 811_589_153);

    let numbers = input
        .text
        .lines()
        .map(|line| Some(i64::from(line.parse::<i16>().ok()?) * decryption_key))
        .collect::<Option<Vec<_>>>()
        .ok_or("Invalid input")?;
    let mut pointers = numbers.iter().collect::<VecDeque<_>>();

    for n in numbers.iter().cycle().take(numbers.len() * iterations) {
        let current_idx = pointers
            .iter()
            .position(|&e| std::ptr::eq(e, n))
            .unwrap_or_default();

        pointers.remove(current_idx).unwrap();
        pointers.insert(
            (n + current_idx as i64).rem_euclid(pointers.len() as i64) as usize,
            n,
        );
    }

    let zero_idx = pointers.iter().position(|&&n| n == 0).unwrap();
    Ok([1000, 2000, 3000]
        .iter()
        .map(|offset| pointers[(zero_idx + offset) % pointers.len()])
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
