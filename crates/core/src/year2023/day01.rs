use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut sum = 0;
    for line in input.text.lines() {
        let line = line.as_bytes();
        let first_digit = find_digit(line.iter(), input.is_part_two());
        let last_digit = find_digit(line.iter().rev(), input.is_part_two());
        let calibration_value = u64::from(first_digit * 10 + last_digit);
        sum += calibration_value;
    }
    Ok(sum)
}

fn find_digit<'a, I: Iterator<Item=&'a u8>>(bytes: I, part2: bool) -> u8 {
    // candidates[N] is the possible start idx of digit N:
    let mut start_idx = [0; 9];
    let mut continues = [0; 9];

    for (byte_idx, byte) in bytes.enumerate() {
        if byte.is_ascii_digit() {
            return byte - b'0';
        } else if part2 {
            match byte {
                b'e' => {
                    if continues[0] + 1 == byte_idx && start_idx[0] + 2 == byte_idx {
                        // on[e]
                        return 1;
                    } else if continues[2] + 1 == byte_idx && start_idx[2] + 4 == byte_idx {
                        // thre[e]
                        return 3;
                    } else if continues[4] + 1 == byte_idx && start_idx[4] + 3 == byte_idx {
                        // fiv[e]
                        return 5;
                    } else if continues[8] + 1 == byte_idx && start_idx[8] + 3 == byte_idx {
                        // nin[e]
                        return 9;
                    }
                    // thr[e]e
                    if continues[2] + 1 == byte_idx && start_idx[2] + 3 == byte_idx {
                        continues[2] = byte_idx;
                    }
                    // s[e]v[e]en
                    if start_idx[6] + 1 == byte_idx || (continues[6] + 1 == byte_idx && start_idx[6] + 3 == byte_idx) {
                        continues[6] = byte_idx;
                    }
                    // [e]ight:
                    start_idx[7] = byte_idx;
                }
                b'f' => {
                    // [f]our
                    start_idx[3] = byte_idx;
                    // [f]ive
                    start_idx[4] = byte_idx;
                }
                b'g' => {
                    // ei[g]ht
                    if continues[7] + 1 == byte_idx && start_idx[7] + 2 == byte_idx {
                        continues[7] = byte_idx;
                    }
                }
                b'h' => {
                    // t[h]ree
                    if start_idx[2] + 1 == byte_idx {
                        continues[2] = byte_idx;
                    }
                    // eig[h]t
                    if continues[7] + 1 == byte_idx && start_idx[7] + 3 == byte_idx {
                        continues[7] = byte_idx;
                    }
                }
                b'i' => {
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
                    }
                }
                b'n' => {
                    // o[n]e
                    if start_idx[0] + 1 == byte_idx {
                        continues[0] = byte_idx;
                    }
                    // seve[n]
                    if continues[6] + 1 == byte_idx && start_idx[6] + 4 == byte_idx {
                        return 7;
                    }
                    // ni[n]e or [n]ine:
                    if continues[8] + 1 == byte_idx && start_idx[8] + 2 == byte_idx {
                        continues[8] = byte_idx;
                    } else {
                        start_idx[8] = byte_idx;
                    }
                }
                b'o' => {
                    // [o]ne
                    start_idx[0] = byte_idx;
                    // tw[o]
                    if continues[1] + 1 == byte_idx && start_idx[1] + 2 == byte_idx {
                        return 2;
                    }
                    // f[o]ur
                    if start_idx[3] + 1 == byte_idx {
                        continues[3] = byte_idx;
                    }
                }
                b'r' => {
                    // th[r]ee
                    if continues[2] + 1 == byte_idx && start_idx[2] + 2 == byte_idx {
                        continues[2] = byte_idx;
                    }
                    // fou[r]
                    if continues[3] + 1 == byte_idx && start_idx[3] + 3 == byte_idx {
                        return 4;
                    }
                }
                b's' => {
                    // [s]ix
                    start_idx[5] = byte_idx;
                    // [s]even
                    start_idx[6] = byte_idx;
                }
                b't' => {
                    // eigh[t]
                    if continues[7] + 1 == byte_idx && start_idx[7] + 4 == byte_idx {
                        return 8;
                    }
                    // [t]wo:
                    start_idx[1] = byte_idx;
                    // [t]hree:
                    start_idx[2] = byte_idx;
                }
                b'u' => {
                    // fo[u]r
                    if continues[3] + 1 == byte_idx && start_idx[3] + 2 == byte_idx {
                        continues[3] = byte_idx;
                    }
                }
                b'v' => {
                    // fi[v]e
                    if continues[4] + 1 == byte_idx && start_idx[4] + 2 == byte_idx {
                        continues[4] = byte_idx;
                    }
                    // se[v]en
                    if continues[6] + 1 == byte_idx && start_idx[6] + 2 == byte_idx {
                        continues[6] = byte_idx;
                    }
                }
                b'w' => {
                    // t[w]o
                    if start_idx[1] + 1 == byte_idx {
                        continues[1] = byte_idx;
                    }
                }
                b'x' => {
                    if continues[5] + 1 == byte_idx && start_idx[5] + 2 == byte_idx {
                        // si[x]
                        return 6;
                    }
                }
                _ => {}
            }
        }
    }
    0
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};
    assert_eq!(1, find_digit(b" one2".iter(), true));
    assert_eq!(2, find_digit(b" two7".iter(), true));
    assert_eq!(3, find_digit(b" three1".iter(), true));
    assert_eq!(4, find_digit(b" four1".iter(), true));
    assert_eq!(5, find_digit(b" five1".iter(), true));
    assert_eq!(6, find_digit(b" six1".iter(), true));
    assert_eq!(7, find_digit(b" seven1".iter(), true));
    assert_eq!(8, find_digit(b" eight1".iter(), true));
    assert_eq!(9, find_digit(b" nine1".iter(), true));

    assert_eq!(0, find_digit(b" 01one".iter(), true));
    assert_eq!(1, find_digit(b" 11one".iter(), true));
    assert_eq!(2, find_digit(b" 21one".iter(), true));
    assert_eq!(3, find_digit(b" 31one".iter(), true));
    assert_eq!(4, find_digit(b" 41one".iter(), true));
    assert_eq!(5, find_digit(b" 51one".iter(), true));
    assert_eq!(6, find_digit(b" 61one".iter(), true));
    assert_eq!(7, find_digit(b" 71one".iter(), true));
    assert_eq!(8, find_digit(b" 81one".iter(), true));
    assert_eq!(9, find_digit(b" 91one".iter(), true));

    let real_input = include_str!("day01_input.txt");
    test_part_one_no_allocations!(real_input => 55386);
    test_part_two_no_allocations!(real_input => 54824);
}
