use crate::common::array_deque::ArrayDeque;
use crate::common::array_stack::ArrayStack;
use crate::common::u256::U256;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    const MAX_SIZE: usize = 192;
    let mut grid = ArrayStack::<MAX_SIZE, U256>::new();
    let mut grid_cols = 0;
    let (mut player_x, mut player_y) = (0, 0);
    for (row_idx, row_str) in input.text.lines().enumerate() {
        let mut row = U256::default();
        for (col_idx, col_byte) in row_str.bytes().enumerate() {
            grid_cols = col_idx + 1;
            match col_byte {
                b'#' => {
                    row.set_bit(col_idx);
                }
                b'S' => {
                    (player_x, player_y) = (col_idx as u8, row_idx as u8);
                }
                _ => {}
            }
        }
        grid.push(row)?;
    }

    let mut reachable_current = [U256::default(); MAX_SIZE];
    let mut work_queue = ArrayDeque::<10000, (u8, u8, u8)>::new();
    work_queue.push_back((player_x, player_y, 0))?;

    let mut last_round = 0;
    while let Some((player_x, player_y, round)) = work_queue.pop_front() {
        if round == 64 {
            if input.is_part_two() {
                // FIXME
                return Ok(0);
            }
            return Ok(reachable_current.iter().map(U256::count_ones).sum());
        }
        if round != last_round {
            last_round = round;
            reachable_current.fill(U256::default());
        }
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_x = i32::from(player_x) + dx;
            let new_y = i32::from(player_y) + dy;
            if new_x < 0
                || new_x > grid_cols as i32
                || new_y < 0
                || new_y > grid.len() as i32
                || grid.elements[new_y as usize].is_bit_set(new_x as usize)
                || reachable_current[new_y as usize].is_bit_set(new_x as usize)
            {
                continue;
            }
            reachable_current[new_y as usize].set_bit(new_x as usize);
            work_queue.push_back((new_x as u8, new_y as u8, round + 1))?;
        }
    }

    Ok(0)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let real_input = include_str!("day21_input.txt");
    test_part_one_no_allocations!(real_input => 3773);
    test_part_two_no_allocations!(real_input => 0);
}
