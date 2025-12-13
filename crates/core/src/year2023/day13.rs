use crate::common::array_stack::ArrayStack;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    let expected_smudges = input.part_values(0, 1);

    input
        .text
        .split("\n\n")
        .map(|part| {
            let mut cols = ArrayStack::<24, u32>::new();
            let mut rows = ArrayStack::<24, u32>::new();
            let mut num_cols = 0;
            for (row_idx, row) in part.lines().enumerate() {
                let mut row_bits = 0;
                for (col_idx, b) in row.bytes().enumerate() {
                    if row_idx == 0 {
                        cols.push(0)?;
                    }
                    let col_bits = &mut cols.elements[col_idx];
                    num_cols = num_cols.max(col_idx);
                    if b == b'#' {
                        row_bits |= 1 << col_idx;
                        *col_bits |= 1 << row_idx;
                    }
                }
                rows.push(row_bits)?;
            }

            Ok((1..cols.len())
                .find(|&col| is_reflection(cols.slice(), col, expected_smudges))
                .unwrap_or_default()
                + 100
                    * (1..rows.len())
                        .find(|&row| is_reflection(rows.slice(), row, expected_smudges))
                        .unwrap_or_default())
        })
        .sum()
}

fn is_reflection(bits: &[u32], cols_to_left: usize, expected_smudges: u32) -> bool {
    let offset = cols_to_left.min(bits.len() - cols_to_left);
    let mut num_smudges = 0;
    for o in 0..offset {
        num_smudges += (bits[cols_to_left - o - 1] ^ bits[cols_to_left + o]).count_ones();
        if num_smudges > expected_smudges {
            return false;
        }
    }
    num_smudges == expected_smudges
}

#[test]
pub fn tests() {
    let test_input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    test_part_one_no_allocations!(test_input => 405);

    let real_input = include_str!("day13_input.txt");
    test_part_one_no_allocations!(real_input => 32_035);
    test_part_two_no_allocations!(real_input => 24_847);
}
