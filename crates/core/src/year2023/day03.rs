use crate::common::u256::U256;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAP_SIZE: usize = 256;
    let mut symbols = vec![U256::default(); MAP_SIZE];

    for (row_idx, row_str) in input.text.lines().enumerate() {
        if row_idx >= MAP_SIZE || row_str.len() >= MAP_SIZE {
            return Err("Too big schematic".to_string());
        }

        for (col_idx, col_byte) in row_str.bytes().enumerate() {
            let is_symbol = !(col_byte.is_ascii_digit() || col_byte == b'.');
            if is_symbol {
                symbols[row_idx].set_bit(col_idx);
            }
        }
    }

    let mut sum = 0;

    for (row_idx, row_str) in input.text.lines().enumerate() {
        let mut num_start_idx = None;
        for (col_idx, col_byte) in row_str.bytes().chain(std::iter::once(b'.')).enumerate() {
            if col_byte.is_ascii_digit() {
                if num_start_idx.is_none() {
                    num_start_idx = Some(col_idx);
                }
            } else {
                if let Some(start_idx) = num_start_idx {
                    let interval_start_col = start_idx.saturating_sub(1);
                    let interval_end_col = col_idx;
                    let adjacent_to_symbol =
                        symbols[row_idx.saturating_sub(1)].range_non_zero(interval_start_col..=interval_end_col)
                            || symbols[row_idx].range_non_zero(interval_start_col..=interval_end_col)
                            || symbols[row_idx + 1].range_non_zero(interval_start_col..=interval_end_col);
                    if adjacent_to_symbol {
                        let num_str = &row_str[start_idx..col_idx];
                        sum += num_str.parse::<u16>().unwrap() as u64;
                    }
                    num_start_idx = None;
                }
            }
        }
    }

    Ok(sum)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

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
    test_part_two_no_allocations!(test_input => 467835);

    let real_input = include_str!("day03_input.txt");
    test_part_one_no_allocations!(real_input => 546563);
    //test_part_two_no_allocations!(real_input => 0);
}
