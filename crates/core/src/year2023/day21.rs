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
    let num_steps = 130;
    let (even_inner, even_outer, odd_inner, odd_outer) =
        bfs(&rocks, grid_width, &[start_position], num_steps)?;

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

    let num_steps = 26501365;
    //
    let diamond_tile_width = (num_steps - start_position.1) as u64 / grid_width as u64;

    // With diamond_tile_width=2:
    //   O
    //  OEO
    // OEOEO
    //  OEO
    //   O
    // => (diamond_tile_width being even)
    // diamond_tile_width^2 even plots, (diamond_tile_width+1)^2 odd plots
    let even_plots = diamond_tile_width * diamond_tile_width * even_full;
    let odd_plots = (diamond_tile_width + 1) * (diamond_tile_width + 1) * odd_full;

    //       ┌--┐
    //       |◸◹|
    //      ◢|  |◣
    //    ┌--┼--┼--┐
    //    |◸ |  | ◹|
    //   ◢|  |  |  |◣
    // ┌--┼--┼--┼--┼--┐
    // |◸ |  |  |  | ◹|
    // |◺ |  |  |  | ◿|
    // └--┼--┼--┼--┼--┘
    //   ◥|  |  |  |◤
    //    |◺ |  | ◿|
    //    └--┼--┼--┘
    //      ◥|  |◤
    //       |◺◿|
    //       └--┘
    //
    // The total area is adjusted by:
    // * Adding `n` extra even corners
    //   ◤◥
    //   ◣◢
    // * Subtracting `n + 1` odd corners
    //   ◸◹
    //   ◺◿
    let extra_even_corners = diamond_tile_width * add_corners;
    let minus_odd_corners = (diamond_tile_width + 1) * remove_corners;

    Ok(even_plots + odd_plots + extra_even_corners - minus_odd_corners)
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

    let mut even_inside = 0;
    let mut even_outside = 0;
    let mut odd_inside = 0;
    let mut odd_outside = 0;

    for &start in starts {
        grid.elements[start.1 as usize].set_bit(start.0 as usize);
        todo.push_back((start, 0))?;
    }

    while let Some((position, cost)) = todo.pop_front() {
        let is_odd = cost % 2 == 1;
        let is_inside = cost <= (grid_width as u32 / 2);
        *match (is_odd, is_inside) {
            (true, true) => &mut odd_inside,
            (true, false) => &mut odd_outside,
            (false, true) => &mut even_inside,
            (false, false) => &mut even_outside,
        } += 1;

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

    Ok((even_inside, even_outside, odd_inside, odd_outside))
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let real_input = include_str!("day21_input.txt");
    test_part_one_no_allocations!(real_input => 3773);
    test_part_two_no_allocations!(real_input => 625_628_021_226_274);
}
