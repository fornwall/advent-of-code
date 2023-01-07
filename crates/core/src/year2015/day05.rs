use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    fn is_nice_part_1(string: &&str) -> bool {
        if string.contains("ab")
            || string.contains("cd")
            || string.contains("pq")
            || string.contains("xy")
        {
            return false;
        }

        string.chars().filter(|&c| "aeiou".contains(c)).count() >= 3
            && string
                .as_bytes()
                .windows(2)
                .any(|window| window[0] == window[1])
    }

    fn is_nice_part_2(string: &&str) -> bool {
        fn find_subsequence(haystack: &[u8], needle: &[u8]) -> bool {
            haystack
                .windows(needle.len())
                .any(|window| window == needle)
        }

        let bytes = string.as_bytes();

        let repeated_with_one_letter_between =
            bytes.windows(3).any(|window| window[0] == window[2]);
        if !repeated_with_one_letter_between {
            return false;
        }

        for i in 0..bytes.len() - 3 {
            let this_char = bytes[i];
            let next_char = bytes[i + 1];
            if find_subsequence(&bytes[(i + 2)..], &[this_char, next_char]) {
                return true;
            }
        }
        false
    }

    Ok(input
        .text
        .lines()
        .filter(if input.is_part_one() {
            is_nice_part_1
        } else {
            is_nice_part_2
        })
        .count())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("ugknbfddgicrmopn" => 1);
    test_part_one!("aaa" => 1);
    test_part_one!("jchzalrnumimnmhp" => 0);
    test_part_one!("haegwjzuvuyypxyu" => 0);
    test_part_one!("dvszwmarrgswjxmb" => 0);

    test_part_two!("qjhvhtzxzqqjkmpb" => 1);
    test_part_two!("xxyxx" => 1);
    test_part_two!("uurcxstgmygtbstg" => 0);
    test_part_two!("ieodomkazucvgmuy" => 0);

    let real_input = include_str!("day05_input.txt");
    test_part_one!(real_input => 238);
    test_part_two!(real_input => 69);
}
