use crate::common::array_deque::ArrayDeque;
use crate::common::array_stack::ArrayStack;
use crate::common::u256::U256;
use crate::input::Input;

const MAX_GRID_SIZE: usize = 192;

type Grid = ArrayStack<MAX_GRID_SIZE, U256>;
type Point = (i32, i32);

/// Solution adapted from @maneatingape - https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day21.rs
pub fn solve(input: &Input) -> Result<u64, String> {
    // Bit set if cell is a rock, represented by a '#'.
    let mut rocks = ArrayStack::<MAX_GRID_SIZE, U256>::new();
    let mut grid_width = 0;
    let mut start_position = None;

    for (y, row_str) in input.text.lines().enumerate() {
        let mut rock_row = U256::default();
        for (col_idx, col_byte) in row_str.bytes().enumerate() {
            grid_width = col_idx as i32 + 1;
            match col_byte {
                b'#' => {
                    rock_row.set_bit(col_idx);
                }
                b'S' => {
                    start_position = Some((col_idx as i32, y as i32));
                }
                _ => {}
            }
        }
        rocks.push(rock_row)?;
    }

    let Some(start_position) = start_position else {
        return Err("No start position".into());
    };

    // Search from the center tile outwards.
    let (even_inner, even_outer, odd_inner, odd_outer) =
        bfs(&rocks, grid_width, &[start_position], 130)?;

    if input.is_part_one() {
        return Ok(even_inner);
    }

    let even_full = even_inner + even_outer;
    let odd_full = odd_inner + odd_outer;
    let remove_corners = odd_outer;

    // Search from the 4 corners inwards.
    let corners = [
        (0, 0),
        (grid_width - 1, 0),
        (0, grid_width - 1),
        (grid_width - 1, grid_width - 1),
    ];
    let (even_inner, ..) = bfs(&rocks, grid_width, &corners, 64)?;
    let add_corners = even_inner;

    // Sum the components of the diamond.
    let n = 202300;
    let first = n * n * even_full;
    let second = (n + 1) * (n + 1) * odd_full;
    let third = n * add_corners;
    let fourth = (n + 1) * remove_corners;
    let part_two = first + second + third - fourth;
    Ok(part_two)
}

/// Breadth first search from any number of starting locations with a limit on maximum steps.
fn bfs(
    grid: &Grid,
    grid_width: i32,
    starts: &[Point],
    limit: u32,
) -> Result<(u64, u64, u64, u64), String> {
    let mut grid = grid.clone();
    let mut todo = ArrayDeque::<512, (Point, u32)>::new();

    let mut even_inner = 0;
    let mut even_outer = 0;
    let mut odd_inner = 0;
    let mut odd_outer = 0;

    for &start in starts {
        grid.elements[start.1 as usize].set_bit(start.0 as usize);
        todo.push_back((start, 0))?;
    }

    while let Some((position, cost)) = todo.pop_front() {
        // First split by odd or even parity then by distance from the starting point.
        if cost % 2 == 1 {
            if (position.0 - grid_width / 2).abs() + (position.1 - grid_width / 2).abs()
                <= (grid_width / 2)
            {
                odd_inner += 1;
            } else {
                odd_outer += 1;
            }
        } else if cost <= 64 {
            even_inner += 1;
        } else {
            even_outer += 1;
        }

        if cost < limit {
            for next in
                [(0, -1), (0, 1), (-1, 0), (1, 0)].map(|o| (position.0 + o.0, position.1 + o.1))
            {
                if next.0 >= 0
                    && next.0 < grid_width
                    && next.1 >= 0
                    && next.1 < grid_width
                    && !grid.elements[next.1 as usize].is_bit_set(next.0 as usize)
                {
                    grid.elements[next.1 as usize].set_bit(next.0 as usize);
                    todo.push_back((next, cost + 1))?;
                }
            }
        }
    }

    Ok((even_inner, even_outer, odd_inner, odd_outer))
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let real_input = include_str!("day21_input.txt");
    test_part_one_no_allocations!(real_input => 3773);
    test_part_two_no_allocations!(real_input => 625_628_021_226_274);
}
