use crate::common::array_stack::ArrayStack;
use crate::common::u256::U256;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<i64, String> {
    let (mut rows, mut cols) = (U256::default(), U256::default());
    for (row_idx, row) in input.text.lines().enumerate() {
        for (col_idx, b) in row.bytes().enumerate() {
            if b == b'#' {
                rows.set_bit(row_idx);
                cols.set_bit(col_idx);
            }
        }
    }

    let mut galaxies = ArrayStack::<512, (u32, u32)>::new();
    let mut row_offset = 0;
    let empty_expansion = input.part_values(1, 999_999);
    for (row_idx, row) in input.text.lines().enumerate() {
        let mut col_offset = 0;
        for (col_idx, b) in row.bytes().enumerate() {
            if b == b'#' {
                galaxies.push((row_offset, col_offset))?;
            }
            col_offset += 1 + u32::from(!cols.is_bit_set(col_idx)) * empty_expansion;
        }
        row_offset += 1 + u32::from(!rows.is_bit_set(row_idx)) * empty_expansion;
    }

    let num_galaxies = galaxies.len();
    Ok(galaxies
        .slice()
        .iter()
        .enumerate()
        .flat_map(|(idx, g1)| {
            (idx + 1..num_galaxies).map(|other_idx| {
                let g2 = galaxies.elements[other_idx];
                (i64::from(g1.0) - i64::from(g2.0)).abs()
                    + (i64::from(g1.1) - i64::from(g2.1)).abs()
            })
        })
        .sum())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    test_part_one_no_allocations!(test_input => 374);

    let real_input = include_str!("day11_input.txt");
    test_part_one_no_allocations!(real_input => 10_422_930);
    test_part_two_no_allocations!(real_input => 699_909_023_130);
}
