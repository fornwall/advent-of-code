use crate::common::map_windows::MapWindowsIterator;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    Ok(input
        .text
        .lines()
        .map(str::as_bytes)
        .map_windows_stable(|[&above, &middle, &below]| {
            middle
                .iter()
                .enumerate()
                .map(|(col_idx, &col_byte)| {
                    let interesting = if input.is_part_one() {
                        !col_byte.is_ascii_digit() && col_byte != b'.'
                    } else {
                        col_byte == b'*'
                    } && col_idx > 0
                        && col_idx + 1 < middle.len();

                    if interesting {
                        let mut num_neighbour_count = 0;
                        let mut this_value = input.part_values(0, 1);

                        for dx in -1_i32..=1 {
                            for row in [above, middle, below] {
                                if dx == 0 && row == middle {
                                    continue;
                                }
                                let x = col_idx as i32 + dx;

                                if matches!(dx, 0 | 1) && row[(x - 1) as usize].is_ascii_digit() {
                                    // Avoid counting same number multiple times.
                                    continue;
                                }

                                if let Some(adjacent_num) = parse_num_at(row, x as usize) {
                                    num_neighbour_count += 1;
                                    if input.is_part_one() {
                                        this_value += u64::from(adjacent_num);
                                    } else if adjacent_num != 0 {
                                        this_value *= u64::from(adjacent_num);
                                    }
                                }
                            }
                        }
                        if input.is_part_one() || (num_neighbour_count == 2) {
                            this_value
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum())
}

fn parse_num_at(str: &[u8], idx: usize) -> Option<u16> {
    if !str[idx].is_ascii_digit() {
        return None;
    }
    let mut start_idx = idx;
    while start_idx > 0 && str[start_idx - 1].is_ascii_digit() {
        start_idx -= 1;
    }
    let mut end_idx = idx;
    while end_idx + 1 < str.len() && str[end_idx + 1].is_ascii_digit() {
        end_idx += 1;
    }
    if end_idx - start_idx > 2 {
        return None;
    }
    let mut result = u16::from(str[start_idx] - b'0');
    while start_idx < end_idx {
        start_idx += 1;
        result = result * 10 + u16::from(str[start_idx] - b'0');
    }
    Some(result)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    assert_eq!(parse_num_at(b"a123", 1), Some(123));
    assert_eq!(parse_num_at(b"a123", 2), Some(123));
    assert_eq!(parse_num_at(b"a123", 3), Some(123));
    assert_eq!(parse_num_at(b"a12312212121212", 3), None);

    let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    test_part_one_no_allocations!(test_input => 4361);
    test_part_two_no_allocations!(test_input => 467_835);

    let real_input = include_str!("day03_input.txt");
    test_part_one_no_allocations!(real_input => 546_563);
    test_part_two_no_allocations!(real_input => 91_031_374);
}
