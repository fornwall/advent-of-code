use crate::common::array_stack::ArrayStack;
use crate::input::Input;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn solve(input: &Input) -> Result<usize, String> {
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
        return Ok(moving
            .slice()
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, row)| (idx + 1) * row.count_ones() as usize)
            .sum());
    }

    let mut hashes = ArrayStack::<500, u64>::new();
    hashes.push(calculate_hash(moving.slice()))?;

    let mut tortoise = 0;
    let mut hare = 0;
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
        hashes.push(hash)?;
        remaining -= 1;
        if hare + 2 < hashes.len() && remaining != 0 {
            tortoise += 1;
            hare += 2;
            if hashes.elements[tortoise] == hashes.elements[hare] {
                let cycle_length = hare - tortoise;
                remaining %= cycle_length;
            }
        }
        if remaining == 0 {
            return Ok(moving
                .slice()
                .iter()
                .rev()
                .enumerate()
                .map(|(idx, row)| (idx + 1) * row.count_ones() as usize)
                .sum());
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
    let mut move_back_amount = if moving[y] & (1 << x) == 0 && fixed[y] & (1 << x) == 0 {
        0
    } else {
        usize::MAX
    };
    loop {
        if (x_dir == -1 && x == 0)
            || (y_dir == -1 && y == 0)
            || (x_dir == 1 && x == num_cols - 1)
            || (y_dir == 1 && y == moving.len() - 1)
        {
            break;
        }
        let new_x = (x as i32 + x_dir) as usize;
        let new_y = (y as i32 + y_dir) as usize;

        move_back_amount = move_back_amount.saturating_add(1);

        let this_is_moving = (moving[new_y] & (1 << new_x)) != 0;
        if this_is_moving {
            if move_back_amount != usize::MAX {
                let old_x = (new_x as i32 - x_dir * move_back_amount as i32) as usize;
                let old_y = (new_y as i32 - y_dir * move_back_amount as i32) as usize;
                moving[old_y] |= 1 << old_x;
                moving[new_y] &= !(1 << new_x);
                move_back_amount -= 1;
            }
        } else {
            let this_is_fixed = (fixed[new_y] & (1 << new_x)) != 0;
            if this_is_fixed {
                move_back_amount = usize::MAX;
            } else if move_back_amount == usize::MAX {
                move_back_amount = 0;
            }
        }

        x = new_x;
        y = new_y;
    }
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
}
