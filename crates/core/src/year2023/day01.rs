use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    Ok(input
        .text
        .lines()
        .map(|line| calibration_value(line, input.is_part_two()))
        .sum())
}

fn calibration_value(line: &str, part2: bool) -> u32 {
    let mut start_idx = [usize::MAX - 10; 9];
    let mut continues = [0; 9];

    let mut first_digit = None;
    let mut last_digit = 0;

    let mut on_digit = |digit| {
        if first_digit.is_none() {
            first_digit = Some(digit);
        }
        last_digit = digit;
    };

    for (byte_idx, byte) in line.bytes().enumerate() {
        match (byte, part2) {
            ((b'0'..=b'9'), _) => {
                on_digit(byte - b'0');
            }
            (b'e', true) => {
                // on[e], thre[e], fiv[e] or nin[e]
                for (digit, digit_len) in [(1, 2), (3, 4), (5, 3), (9, 3)] {
                    if continues[digit - 1] + 1 == byte_idx
                        && start_idx[digit - 1] + digit_len == byte_idx
                    {
                        on_digit(digit as u8);
                    }
                }
                // thr[e]e
                if continues[2] + 1 == byte_idx && start_idx[2] + 3 == byte_idx {
                    continues[2] = byte_idx;
                }
                // s[e]v[e]en
                if start_idx[6] + 1 == byte_idx
                    || (continues[6] + 1 == byte_idx && start_idx[6] + 3 == byte_idx)
                {
                    continues[6] = byte_idx;
                }
                // [e]ight:
                start_idx[7] = byte_idx;
            }
            (b'f', true) => {
                // [f]our
                start_idx[3] = byte_idx;
                // [f]ive
                start_idx[4] = byte_idx;
            }
            (b'g', true) => {
                // ei[g]ht
                if continues[7] + 1 == byte_idx && start_idx[7] + 2 == byte_idx {
                    continues[7] = byte_idx;
                }
            }
            (b'h', true) => {
                // t[h]ree
                if start_idx[2] + 1 == byte_idx {
                    continues[2] = byte_idx;
                }
                // eig[h]t
                if continues[7] + 1 == byte_idx && start_idx[7] + 3 == byte_idx {
                    continues[7] = byte_idx;
                }
            }
            (b'i', true) => {
                // f[i]ve
                if start_idx[4] + 1 == byte_idx {
                    continues[4] = byte_idx;
                }
                // s[i]x
                if start_idx[5] + 1 == byte_idx {
                    continues[5] = byte_idx;
                }
                // e[i]ght
                if start_idx[7] + 1 == byte_idx {
                    continues[7] = byte_idx;
                }
                // n[i]ne
                if start_idx[8] + 1 == byte_idx {
                    continues[8] = byte_idx;
                } else if continues[8] + 1 == byte_idx && start_idx[8] + 3 == byte_idx {
                    // Handle ni[n]ine:
                    start_idx[8] = byte_idx - 1;
                    continues[8] = byte_idx;
                }
            }
            (b'n', true) => {
                // o[n]e
                if start_idx[0] + 1 == byte_idx {
                    continues[0] = byte_idx;
                }
                // seve[n]
                if continues[6] + 1 == byte_idx && start_idx[6] + 4 == byte_idx {
                    on_digit(7);
                }
                // ni[n]e or [n]ine (we handle the case of ni[n]ine, where we here
                // will incorrectly set [n] as the third character, when handling 'i'):
                if continues[8] + 1 == byte_idx && start_idx[8] + 2 == byte_idx {
                    continues[8] = byte_idx;
                } else {
                    start_idx[8] = byte_idx;
                }
            }
            (b'o', true) => {
                // [o]ne
                start_idx[0] = byte_idx;
                // tw[o]
                if continues[1] + 1 == byte_idx && start_idx[1] + 2 == byte_idx {
                    on_digit(2);
                }
                // f[o]ur
                if start_idx[3] + 1 == byte_idx {
                    continues[3] = byte_idx;
                }
            }
            (b'r', true) => {
                // th[r]ee
                if continues[2] + 1 == byte_idx && start_idx[2] + 2 == byte_idx {
                    continues[2] = byte_idx;
                }
                // fou[r]
                if continues[3] + 1 == byte_idx && start_idx[3] + 3 == byte_idx {
                    on_digit(4);
                }
            }
            (b's', true) => {
                // [s]ix
                start_idx[5] = byte_idx;
                // [s]even
                start_idx[6] = byte_idx;
            }
            (b't', true) => {
                // eigh[t]
                if continues[7] + 1 == byte_idx && start_idx[7] + 4 == byte_idx {
                    on_digit(8);
                }
                // [t]wo:
                start_idx[1] = byte_idx;
                // [t]hree:
                start_idx[2] = byte_idx;
            }
            (b'u', true) => {
                // fo[u]r
                if continues[3] + 1 == byte_idx && start_idx[3] + 2 == byte_idx {
                    continues[3] = byte_idx;
                }
            }
            (b'v', true) => {
                // fi[v]e
                if continues[4] + 1 == byte_idx && start_idx[4] + 2 == byte_idx {
                    continues[4] = byte_idx;
                }
                // se[v]en
                if continues[6] + 1 == byte_idx && start_idx[6] + 2 == byte_idx {
                    continues[6] = byte_idx;
                }
            }
            (b'w', true) => {
                // t[w]o
                if start_idx[1] + 1 == byte_idx {
                    continues[1] = byte_idx;
                }
            }
            (b'x', true) => {
                if continues[5] + 1 == byte_idx && start_idx[5] + 2 == byte_idx {
                    // si[x]
                    on_digit(6);
                }
            }
            _ => {}
        }
    }

    u32::from(first_digit.unwrap_or_default() * 10 + last_digit)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    test_part_one_no_allocations!(test_input => 142);

    let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
five3threeqgtwone
twone
oneight
nine
ninine";
    test_part_two_no_allocations!(test_input => 281 + 51 + 21 + 18 + 99 + 99);
    let test_input = "cneightwotdkfxxxjfdpz3zkkthree";
    test_part_two_no_allocations!(test_input => 83);

    let real_input = include_str!("day01_input.txt");
    test_part_one_no_allocations!(real_input => 55386);
    test_part_two_no_allocations!(real_input => 54824);
}
