use crate::input::Input;
use std::collections::HashSet;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    Ok(if input.is_part_one() {
        input
            .text
            .lines()
            .map(|line| {
                HashSet::<u8>::from_iter((line[0..line.len() / 2]).bytes())
                    .intersection(&HashSet::from_iter((line[line.len() / 2..]).bytes()))
                    .copied()
                    .map(item_priority)
                    .sum::<u32>()
            })
            .sum()
    } else {
        input
            .text
            .lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|group| {
                HashSet::<u8>::from_iter(group[0].bytes())
                    .intersection(&HashSet::from_iter(group[1].bytes()))
                    .copied()
                    .collect::<HashSet<_>>()
                    .intersection(&HashSet::from_iter(group[2].bytes()))
                    .copied()
                    .map(item_priority)
                    .sum::<u32>()
            })
            .sum()
    })
}

fn item_priority(b: u8) -> u32 {
    u32::from(if b.is_ascii_lowercase() {
        b - b'a' + 1
    } else if b.is_ascii_uppercase() {
        b - b'A' + 27
    } else {
        0
    })
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    test_part_one!(test_input => 157);
    test_part_two!(test_input => 70);

    let real_input = include_str!("day03_input.txt");
    test_part_one!(real_input => 8176);
    test_part_two!(real_input => 2689);
}
