use crate::input::Input;
use std::collections::HashSet;

fn is_valid(password: &[u8]) -> bool {
    // "Passwords must include one increasing straight of at least three letters,
    // like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count."
    if !password
        .windows(3)
        .any(|w| w[1] == w[0] + 1 && w[2] == w[1] + 1)
    {
        return false;
    }

    // "Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing."
    if password.iter().any(|b| [b'i', b'o', b'l'].contains(b)) {
        return false;
    }

    // Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
    let mut pairs = HashSet::new();
    for window in password.windows(2) {
        if window[0] == window[1] {
            pairs.insert(window);
        }
    }
    pairs.len() > 1
}

pub fn solve(input: &mut Input) -> Result<String, String> {
    let bytes = input.text.as_bytes();
    if bytes.len() != 8 || bytes.iter().any(|b| !b.is_ascii_lowercase()) {
        return Err("Invalid current password (not 8 lower ASCII characters)".to_string());
    }

    let mut current_password = [0_u8; 8];
    current_password.copy_from_slice(bytes);
    let mut return_next_password = input.is_part_one();

    'outer: loop {
        if is_valid(&current_password) {
            if return_next_password {
                return Ok(std::str::from_utf8(&current_password)
                    .map_err(|_| "Invalid utf-8 in password")?
                    .to_string());
            } else {
                return_next_password = true;
            }
        }
        if current_password[7] == b'z' {
            for idx in (0..=6).rev() {
                if current_password[idx] != b'z' {
                    current_password[idx] += 1;
                    for c in current_password.iter_mut().skip(idx + 1) {
                        *c = b'a';
                    }
                    continue 'outer;
                }
            }
            return Err("Unable to generate valid password".to_string());
        } else {
            current_password[7] += 1;
        }
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("abcdefgh" => "abcdffaa".to_string());

    let real_input = include_str!("day11_input.txt");
    test_part_one!(real_input => "hepxxyzz".to_string());
    test_part_two!(real_input => "heqaabcc".to_string());
}
