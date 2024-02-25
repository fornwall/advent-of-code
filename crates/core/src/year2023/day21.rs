use crate::common::array_stack::ArrayStack;
use crate::common::u256::U256;
use crate::input::Input;

const MAX_GRID_SIZE: usize = 192;

type Grid = ArrayStack<MAX_GRID_SIZE, U256>;

pub fn solve(input: &Input) -> Result<u64, String> {
    // Bit set if cell is a rock, represented by a '#'.
    let mut rocks = ArrayStack::<MAX_GRID_SIZE, U256>::new();
    // Bit set if cell is currently reachable, initially represented by a 'S'.
    let mut reachable = ArrayStack::<MAX_GRID_SIZE, U256>::new();
    let mut grid_width = 0;

    for row_str in input.text.lines() {
        let mut rock_row = U256::default();
        let mut reachable_row = U256::default();
        for (col_idx, col_byte) in row_str.bytes().enumerate() {
            grid_width = col_idx + 1;
            match col_byte {
                b'#' => {
                    rock_row.set_bit(col_idx);
                }
                b'S' => {
                    reachable_row.set_bit(col_idx);
                }
                _ => {}
            }
        }
        rocks.push(rock_row)?;
        reachable.push(reachable_row)?;
    }

    Ok(if input.is_part_one() {
        for _round in 0..64 {
            let mut reachable_now = Grid::with_len(reachable.len());
            for y in 1..rocks.len() {
                for x in 1..grid_width {
                    if !rocks.elements[y].is_bit_set(x)
                        && (reachable.elements[y].is_bit_set(x - 1)
                            || reachable.elements[y].is_bit_set(x + 1)
                            || reachable.elements[y - 1].is_bit_set(x)
                            || reachable.elements[y + 1].is_bit_set(x))
                    {
                        reachable_now.elements[y].set_bit(x);
                    }
                }
            }
            std::mem::swap(&mut reachable_now, &mut reachable);
        }
        u64::from(reachable.slice().iter().map(U256::count_ones).sum::<u32>())
    } else {
        // Assumption: Any non-reachable positions are single squares directly enclosed.
        // Fill unreachable cells with rocks given the above assumption.
        for y in 1..rocks.len() {
            for x in 1..grid_width {
                if rocks.elements[y].is_bit_set(x - 1)
                    && rocks.elements[y].is_bit_set(x + 1)
                    && rocks.elements[y - 1].is_bit_set(x)
                    && rocks.elements[y + 1].is_bit_set(x)
                {
                    rocks.elements[y].set_bit(x);
                }
            }
        }

        let y0 = reachable_at_step(&rocks, grid_width, 1);
        let y1 = reachable_at_step(&rocks, grid_width, 3);
        let y2 = reachable_at_step(&rocks, grid_width, 5);

        // Assumption: (26_501_365 - 65) / grid_width has no remainder.
        let num_reached_grids = (26_501_365 - 65) / grid_width as u64;

        let delta1 = y1 - y0;
        let delta2 = y2 - y1;

        y0 + num_reached_grids * delta1
            + (num_reached_grids * (num_reached_grids - 1) / 2) * (delta2 - delta1)
    })
}

fn reachable_at_step(rocks: &Grid, grid_width: usize, num_halves: usize) -> u64 {
    let step = ((num_halves * grid_width) / 2) as i32;
    let mut count = 0;
    for dx in -step..=step {
        let y_range = step - dx.abs();
        for dy in (-y_range..=y_range).step_by(2) {
            if is_clear(rocks, grid_width, 65 + dx, 65 + dy) {
                count += 1;
            }
        }
    }
    count
}

fn is_clear(rocks: &Grid, grid_width: usize, mut x: i32, mut y: i32) -> bool {
    let grid_width = grid_width as i32;
    let grid_height = rocks.len() as i32;

    x = i32::from(x < 0) * grid_width + x % grid_width;
    y = i32::from(y < 0) * grid_height + y % grid_height;

    !rocks.elements[y as usize].is_bit_set(x as usize)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let real_input = include_str!("day21_input.txt");
    test_part_one_no_allocations!(real_input => 3773);
    test_part_two_no_allocations!(real_input => 625_628_021_226_274);
}
