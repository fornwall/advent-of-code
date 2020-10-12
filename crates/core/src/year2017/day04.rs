fn solution(input_string: &str, anagrams_are_equal: bool) -> Result<usize, String> {
    Ok(input_string
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

pub fn part1(input_string: &str) -> Result<usize, String> {
    solution(input_string, false)
}

pub fn part2(input_string: &str) -> Result<usize, String> {
    solution(input_string, true)
}

#[test]
fn test_part1() {
    assert_eq!(Ok(325), part1(include_str!("day04_input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(Ok(119), part2(include_str!("day04_input.txt")));
}
