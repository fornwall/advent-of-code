use crate::common::chunk_iterator::ChunkIteratorExt;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    let lines = input.text.lines();
    Ok(if input.is_part_one() {
        lines
            .map(|line| {
                let compartments = line.split_at(line.len() / 2);
                common_item_priority([compartments.0, compartments.1])
            })
            .sum()
    } else {
        lines.chunks_exact::<3>().map(common_item_priority).sum()
    })
}

fn common_item_priority<const N: usize>(item_groups: [&str; N]) -> u32 {
    item_groups
        .iter()
        .map(|items| items_bitset(items))
        .fold(u64::MAX, |acc, x| acc & x)
        .trailing_zeros()
}

fn items_bitset(items: &str) -> u64 {
    items
        .bytes()
        .map(|item_char| {
            1 << u64::from(match item_char {
                b'a'..=b'z' => item_char - b'a' + 1,
                b'A'..=b'Z' => item_char - b'A' + 27,
                _ => 0,
            })
        })
        .fold(0, |acc, x| acc | x)
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

#[cfg(feature = "count-allocations")]
#[test]
pub fn no_memory_allocations() {
    use crate::input::{test_part_one, test_part_two};
    let real_input = include_str!("day03_input.txt");
    let allocations = allocation_counter::count(|| {
        test_part_one!(real_input => 8176);
        test_part_two!(real_input => 2689);
    });
    assert_eq!(allocations, 0);
}
