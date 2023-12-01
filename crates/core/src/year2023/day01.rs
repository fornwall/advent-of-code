use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    Ok(input
        .text
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let first_digit = find_digit(line.iter(), input.is_part_two());
            let last_digit = find_digit(line.iter().rev(), input.is_part_two());
            u32::from(first_digit * 10 + last_digit)
        })
        .sum())
}

fn find_digit<'a, I: Iterator<Item = &'a u8>>(bytes: I, part2: bool) -> u8 {
    let mut start_idx = [0; 9];
    let mut continues = [0; 9];

    for (byte_idx, byte) in bytes.enumerate() {
        match (byte, part2) {
            ((b'0'..=b'9'), _) => {
                return byte - b'0';
            }
            (b'e', true) => {
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
                }
            }
            (b'n', true) => {
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
            (b'o', true) => {
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
            (b'r', true) => {
                // th[r]ee
                if continues[2] + 1 == byte_idx && start_idx[2] + 2 == byte_idx {
                    continues[2] = byte_idx;
                }
                // fou[r]
                if continues[3] + 1 == byte_idx && start_idx[3] + 3 == byte_idx {
                    return 4;
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
                    return 8;
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
                    return 6;
                }
            }
            _ => {}
        }
    }
    0
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};
    let real_input = include_str!("day01_input.txt");
    test_part_one_no_allocations!(real_input => 55386);
    test_part_two_no_allocations!(real_input => 54824);
}
