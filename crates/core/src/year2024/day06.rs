use crate::common::u256::U256;
use crate::input::{Input, on_error};

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const JUMP_MAP_WIDTH: usize = 192;

pub fn solve(input: &Input) -> Result<u32, String> {
    let width = input
        .text
        .lines()
        .next()
        .map(|line| line.len())
        .ok_or_else(on_error)? as i32;

    let grid = Grid {
        s: input.text.as_bytes(),
        width,
    };

    if grid.s.len() != ((grid.width + 1) * grid.width - 1) as usize {
        return Err("Invalid input - not a rectangle".to_string());
    }

    let mut visited = [U256::default(); 192];
    // Indexed by direction idx:
    let mut repeated_visit = [[U256::default(); 192]; 4];
    // Indexed by direction idx - how many positions to jump if not obstacle placed:
    let mut jump_map = [[u8::MAX; JUMP_MAP_WIDTH * JUMP_MAP_WIDTH]; 4];

    let mut current_direction_idx = 0;
    let mut current_position = (0, 0);
    let mut initial_position = (0, 0);
    'outer: for x in 0..grid.width {
        for y in 0..grid.width {
            if grid.at((x, y)) == b'^' {
                initial_position = (x, y);
                current_position = initial_position;
                visited[y as usize].set_bit(x as usize);
                break 'outer;
            }
        }
    }

    let mut placed_obstacles = [U256::default(); 192];
    placed_obstacles[initial_position.1 as usize].set_bit(initial_position.0 as usize);

    let mut num_loops = 0;

    let mut current_jumps_len = 0;
    let mut current_jumps_start_position = (0, 0);

    loop {
        let dir = DIRECTIONS[current_direction_idx];
        let new_position = (current_position.0 + dir.0, current_position.1 + dir.1);
        match grid.at(new_position) {
            b'L' => {
                break;
            }
            b'#' => {
                jump_map[current_direction_idx][current_jumps_start_position.1 as usize
                    * JUMP_MAP_WIDTH
                    + current_jumps_start_position.0 as usize] = current_jumps_len;
                current_jumps_len = 0;

                current_direction_idx = (current_direction_idx + 1) % 4;
                repeated_visit[current_direction_idx][current_position.1 as usize]
                    .set_bit(current_position.0 as usize);
            }
            _ => {
                // Free space.
                if input.is_part_two()
                    && !placed_obstacles[new_position.1 as usize]
                        .is_bit_set(new_position.0 as usize)
                {
                    // What if we placed an obstacle here? But only if not initial position:
                    // "The new obstruction can't be placed at the guard's starting position
                    // - the guard is there right now and would notice."
                    placed_obstacles[new_position.1 as usize].set_bit(new_position.0 as usize);
                    if does_movements_repeat(
                        &grid,
                        &mut jump_map,
                        &repeated_visit,
                        new_position,
                        current_position,
                        current_direction_idx,
                    ) {
                        num_loops += 1;
                    }
                }
                if current_jumps_len == 0 {
                    current_jumps_start_position = current_position;
                }
                current_jumps_len += 1;
                current_position = new_position;
                visited[current_position.1 as usize].set_bit(current_position.0 as usize);
            }
        }
    }

    Ok(if input.is_part_one() {
        visited.iter().map(U256::count_ones).sum()
    } else {
        num_loops
    })
}

fn does_movements_repeat(
    grid: &Grid,
    jump_map: &mut [[u8; JUMP_MAP_WIDTH * JUMP_MAP_WIDTH]; 4],
    repeated_visit: &[[U256; 192]; 4],
    obstacle_position: (i32, i32),
    mut current_position: (i32, i32),
    mut current_direction_idx: usize,
) -> bool {
    let mut repeated_visit = *repeated_visit;
    repeated_visit[current_direction_idx][current_position.1 as usize]
        .set_bit(current_position.0 as usize);

    let mut current_jumps_len = 0;
    let mut current_jumps_start_position = (0, 0);

    loop {
        let dir = DIRECTIONS[current_direction_idx];
        let new_position = (current_position.0 + dir.0, current_position.1 + dir.1);
        let stopped_by_placed_obstacle = new_position == obstacle_position;

        match (stopped_by_placed_obstacle, grid.at(new_position)) {
            (true, _) | (_, b'#') => {
                if !stopped_by_placed_obstacle {
                    jump_map[current_direction_idx][current_jumps_start_position.1 as usize
                        * JUMP_MAP_WIDTH
                        + current_jumps_start_position.0 as usize] = current_jumps_len;
                }

                current_direction_idx = (current_direction_idx + 1) % 4;
                if repeated_visit[current_direction_idx][current_position.1 as usize]
                    .is_bit_set(current_position.0 as usize)
                {
                    return true;
                }
                repeated_visit[current_direction_idx][current_position.1 as usize]
                    .set_bit(current_position.0 as usize);

                let mut jump_len = jump_map[current_direction_idx]
                    [current_position.1 as usize * JUMP_MAP_WIDTH + current_position.0 as usize];
                if jump_len != u8::MAX {
                    let jump_dir = DIRECTIONS[current_direction_idx];
                    // Check if placed obstacle blocks jump:
                    let dist = (
                        obstacle_position.0 - current_position.0,
                        obstacle_position.1 - current_position.1,
                    );
                    if (dist.0 == 0 || dist.1 == 0)
                        && (dist.0.signum(), dist.1.signum())
                            == (jump_dir.0.signum(), jump_dir.1.signum())
                    {
                        jump_len = (dist.0.abs() + dist.1.abs() - 1).min(jump_len as i32) as u8;
                    }

                    current_position = (
                        current_position.0 + jump_dir.0 * jump_len as i32,
                        current_position.1 + jump_dir.1 * jump_len as i32,
                    );
                }
                current_jumps_len = 0;
            }
            (false, b'L') => {
                return false;
            }
            _ => {
                if current_jumps_len == 0 {
                    current_jumps_start_position = current_position;
                }
                current_jumps_len += 1;
                current_position = new_position;
            }
        }
    }
}

struct Grid<'a> {
    s: &'a [u8],
    width: i32,
}

impl Grid<'_> {
    const fn at(&self, position: (i32, i32)) -> u8 {
        if position.0 < 0 || position.0 >= self.width || position.1 < 0 || position.1 >= self.width
        {
            return b'L';
        }
        self.s[(position.0 + (self.width + 1) * position.1) as usize]
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    test_part_one_no_allocations!(test_input => 41);
    test_part_two_no_allocations!(test_input => 6);

    let real_input = include_str!("day06_input.txt");
    test_part_one_no_allocations!(real_input => 4939);
    test_part_two_no_allocations!(real_input => 1434);
}
