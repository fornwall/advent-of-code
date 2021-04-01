use crate::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    fn is_nice_part_1(string: &&str) -> bool {
        let mut vowel_count = 0;
        let mut twice_in_a_row = false;
        let mut contains_forbidden = false;
        let mut last_char = ' ';

        for c in string.chars() {
            if "aeiou".contains(c) {
                vowel_count += 1;
            }

            if c == last_char {
                twice_in_a_row = true;
            }

            // "It does not contain the strings ab, cd, pq, or xy,
            // even if they are part of one of the other requirements":
            if (last_char == 'a' && c == 'b')
                || (last_char == 'c' && c == 'd')
                || (last_char == 'p' && c == 'q')
                || (last_char == 'x' && c == 'y')
            {
                contains_forbidden = true;
            }

            last_char = c;
        }

        !contains_forbidden && vowel_count >= 3 && twice_in_a_row
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
    use crate::{test_part_one, test_part_two};

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
