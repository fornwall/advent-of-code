use crate::common::array_stack::ArrayStack;
use crate::input::Input;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut moving = ArrayStack::<128, u128>::new();
    let mut fixed = ArrayStack::<128, u128>::new();
    let mut num_cols = 0;

    for row_str in input.text.lines() {
        let mut moving_bits = 0;
        let mut fixed_bits = 0;
        for (col_idx, b) in row_str.bytes().enumerate() {
            num_cols = num_cols.max(col_idx + 1);
            match b {
                b'#' => {
                    fixed_bits |= 1 << col_idx;
                }
                b'O' => {
                    moving_bits |= 1 << col_idx;
                }
                _ => {}
            }
        }
        moving.push(moving_bits)?;
        fixed.push(fixed_bits)?;
    }

    if input.is_part_one() {
        for x in 0..num_cols {
            move_dir(0, 1, x, 0, num_cols, moving.slice_mut(), fixed.slice());
        }
        return Ok(total_load(moving.slice()));
    }

    let mut hashes = ArrayStack::<256, u64>::new();
    hashes.push(calculate_hash(moving.slice()))?;

    let mut remaining = 1_000_000_000;
    loop {
        for x in 0..num_cols {
            move_dir(0, 1, x, 0, num_cols, moving.slice_mut(), fixed.slice());
        }
        for y in 0..moving.len() {
            move_dir(1, 0, 0, y, num_cols, moving.slice_mut(), fixed.slice());
        }
        for x in 0..num_cols {
            move_dir(
                0,
                -1,
                x,
                moving.len() - 1,
                num_cols,
                moving.slice_mut(),
                fixed.slice(),
            );
        }
        for y in 0..moving.len() {
            move_dir(
                -1,
                0,
                num_cols - 1,
                y,
                num_cols,
                moving.slice_mut(),
                fixed.slice(),
            );
        }
        let hash = calculate_hash(moving.slice());
        remaining -= 1;
        for i in 0..hashes.len() {
            if hashes.elements[i] == hash {
                let cycle_length = hashes.len() - i;
                remaining %= cycle_length;
            }
        }
        hashes.push(hash)?;
        if remaining == 0 {
            return Ok(total_load(moving.slice()));
        }
    }
}

fn move_dir(
    x_dir: i32,
    y_dir: i32,
    mut x: usize,
    mut y: usize,
    num_cols: usize,
    moving: &mut [u128],
    fixed: &[u128],
) {
    let mut position_at = (x, y);

    loop {
        if (moving[y] & (1 << x)) != 0 {
            moving[y] &= !(1 << x);
            moving[position_at.1] |= 1 << position_at.0;
            position_at = (
                (position_at.0 as i32 + x_dir) as usize,
                (position_at.1 as i32 + y_dir) as usize,
            );
        } else if (fixed[y] & (1 << x)) != 0 {
            position_at = ((x as i32 + x_dir) as usize, (y as i32 + y_dir) as usize);
        }

        if (x_dir == -1 && x == 0)
            || (y_dir == -1 && y == 0)
            || (x_dir == 1 && x == num_cols - 1)
            || (y_dir == 1 && y == moving.len() - 1)
        {
            break;
        }
        x = (x as i32 + x_dir) as usize;
        y = (y as i32 + y_dir) as usize;
    }
}

fn total_load(moving: &[u128]) -> u64 {
    moving
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, row)| (idx + 1) as u64 * u64::from(row.count_ones()))
        .sum()
}

fn calculate_hash(t: &[u128]) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    test_part_one_no_allocations!(test_input => 136);
    test_part_two_no_allocations!(test_input => 64);

    let real_input = include_str!("day14_input.txt");
    test_part_one_no_allocations!(real_input => 108_641);
    test_part_two_no_allocations!(real_input => 84_328);

    let real_input = include_str!("day14_input_other.txt");
    test_part_two_no_allocations!(real_input => 91_286);
}
