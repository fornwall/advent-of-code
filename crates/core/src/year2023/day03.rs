use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    let rows = input.text.lines().map(str::as_bytes).collect::<Vec<_>>();
    let mut sum = 0;

    for (row_idx, row_str) in input.text.lines().enumerate() {
        let row_bytes = row_str.as_bytes();
        for (col_idx, col_byte) in row_bytes
            .iter()
            .copied()
            .chain(std::iter::once(b'.'))
            .enumerate()
        {
            let interesting = if input.is_part_one() {
                !col_byte.is_ascii_digit() && col_byte != b'.'
            } else {
                col_byte == b'*'
            };
            if interesting {
                let mut num_neighbour_count = 0;
                let mut this_value = input.part_values(0, 1);
                for dx in -1_i32..=1 {
                    for dy in -1_i32..=1 {
                        if (col_idx, dx) == (0, -1)
                            || (row_idx, dy) == (0, -1)
                            || (dx == 0 && dy == 0)
                        {
                            continue;
                        }
                        let x = col_idx as i32 + dx;
                        let y = row_idx as i32 + dy;
                        if matches!((dx, dy), (0 | 1, -1 | 1))
                            && rows[y as usize][(x - 1) as usize].is_ascii_digit()
                        {
                            // Avoid counting same number multiple times.
                            continue;
                        }
                        if let Some(adjacent_num) = parse_num_at(rows[y as usize], x as usize) {
                            num_neighbour_count += 1;
                            if input.is_part_one() {
                                this_value += u64::from(adjacent_num);
                            } else if adjacent_num != 0 {
                                this_value *= u64::from(adjacent_num);
                            }
                        }
                    }
                }
                sum += if input.is_part_one() || (num_neighbour_count == 2) {
                    this_value
                } else {
                    0
                };
            }
        }
    }

    Ok(sum)
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
    let mut result = u16::from(str[start_idx] - b'0');
    while start_idx < end_idx {
        start_idx += 1;
        result = result * 10 + u16::from(str[start_idx] - b'0');
    }
    Some(result)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_num_allocations, test_part_two_num_allocations};

    assert_eq!(parse_num_at(b"a123", 1), Some(123));
    assert_eq!(parse_num_at(b"a123", 2), Some(123));
    assert_eq!(parse_num_at(b"a123", 3), Some(123));

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

    test_part_one_num_allocations!(test_input => 4361, 1);
    test_part_two_num_allocations!(test_input => 467_835, 1);

    let real_input = include_str!("day03_input.txt");
    test_part_one_num_allocations!(real_input => 546_563, 1);
    test_part_two_num_allocations!(real_input => 91_031_374, 1);
}
