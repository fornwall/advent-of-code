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
    fn set_if_idx(candidates: &mut usize, idx: usize, start_idx: usize) {}

    // candidates[N] is the possible start idx of digit N:
    let mut candidates = [0; 9];
    let mut last_checked = [0; 9];

    for (byte_idx, byte) in bytes.enumerate() {
        if byte.is_ascii_digit() {
            return byte - b'0';
        } else if part2 {
            match byte {
                b'e' => {
                    if candidates[0] + 2 == byte_idx {
                        // on[e]
                        return 1;
                    } else if candidates[2] + 4 == byte_idx {
                        // thre[e]
                        return 3;
                    } else if candidates[4] + 3 == byte_idx {
                        // fiv[e]
                        return 5;
                    } else if candidates[8] + 3 == byte_idx {
                        // nin[e]
                        return 9;
                    }
                }
                b'o' => {
                    if candidates[1] + 2 == byte_idx {
                        // tw[o]
                        return 2;
                    }
                }
                b'r' => {
                    if candidates[3] + 3 == byte_idx {
                        // fou[r]
                        return 4;
                    }
                }
                b'x' => {
                    if candidates[5] + 2 == byte_idx {
                        // si[x]
                        return 6;
                    }
                }
                b'n' => {
                    if candidates[6] + 4 == byte_idx {
                        // seve[n]
                        return 7;
                    }
                }
                b't' => {
                    if candidates[7] + 4 == byte_idx {
                        // eigh[t]
                        return 8;
                    }
                }
                _ => {}
            }
            for (candidate_idx, candidate_start_idx) in candidates.iter_mut().enumerate() {
                let candidate_num = candidate_idx as u8 + 1;
                let candidate_len = byte_idx - *candidate_start_idx;
                let continue_match = match (byte, candidate_num, candidate_len) {
                    // thr[e]e | s[e]ven | sev[e]n | [e]ight
                    (b'e', 3, 3) | (b'e', 7, 1 | 3) | (b'e', 8, 0) |
                    // [f]our | [f]ive
                    (b'f', 4 | 5, 0) |
                    // ei[g]ht
                    (b'g', 8, 2) |
                    // t[h]ree | eig[h]t
                    (b'h', 3, 1) | (b'h', 8, 3) |
                    // f[i]ve | s[i]x | e[i]ght | n[i]ne
                    (b'i', 5 | 6 | 8 | 9, 1) |
                    // o[n]e | [n]ine | ni[n]e
                    (b'n', 1, 1) | (b'n', 9, 0 | 2) |
                    // [o]ne | tw[o] | f[o]ur
                    (b'o', 1, 0) | (b'o', 4, 1) |
                    // th[r]ee
                    (b'r', 3, 2) |
                    // [s]ix, [s]even
                    (b's', 6 | 7, 0) |
                    // [t]wo | [t]hree
                    (b't', 2 | 3, 0) |
                    // fo[u]r
                    (b'u', 4, 2) |
                    // fi[v]e | se[v]en
                    (b'v', 5 | 7, 2) |
                    // t[w]o
                    (b'w', 2, 1) => true,
                    _ => false,
                };
                if !continue_match {
                    *candidate_start_idx = byte_idx + 1;
                };
            }
        }
    }
    0
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};
    assert_eq!(1, find_digit(b" one2".iter(), true));
    assert_eq!(2, find_digit(b" two3".iter(), true));
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
