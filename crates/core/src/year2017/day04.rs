use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    let anagrams_are_equal = input.is_part_two();

    Ok(input
        .text
        .lines()
        .filter(|passphrase| {
            let mut words: Vec<Vec<char>> = passphrase
                .split_ascii_whitespace()
                .map(|word| {
                    let mut chars: Vec<char> = word.chars().collect();
                    if anagrams_are_equal {
                        chars.sort_unstable();
                    }
                    chars
                })
                .collect();
            words.sort();
            let initial_len = words.len();
            words.dedup();
            initial_len == words.len()
        })
        .count())
}

#[test]
fn test() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day04_input.txt");
    test_part_one!(real_input => 325);
    test_part_two!(real_input => 119);
}
