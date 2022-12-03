use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let lines = input.text.lines();
    Ok(if input.is_part_one() {
        lines
            .map(|line| item_priority(common(line.split_at(line.len() / 2))))
            .sum()
    } else {
        lines
            .collect::<Vec<_>>()
            .chunks_exact(3)
            .map(|group| item_priority(common_three(group[0], group[1], group[2])))
            .sum()
    })
}

fn common(pair: (&str, &str)) -> u8 {
    pair.0
        .bytes()
        .find(|d| pair.1.as_bytes().contains(d))
        .unwrap_or_default()
}

fn common_three(a: &str, b: &str, c: &str) -> u8 {
    a.bytes()
        .find(|d| b.as_bytes().contains(d) && c.as_bytes().contains(d))
        .unwrap_or_default()
}

fn item_priority(b: u8) -> u32 {
    u32::from(match b {
        b'a'..=b'z' => b - b'a' + 1,
        b'A'..=b'Z' => b - b'A' + 27,
        _ => 0,
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
