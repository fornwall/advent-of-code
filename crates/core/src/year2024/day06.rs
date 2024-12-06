use crate::{
    common::u256::U256,
    input::{on_error, Input},
};

pub fn solve(input: &Input) -> Result<u32, String> {
    let width = input
        .text
        .lines()
        .next()
        .map(|line| line.len())
        .ok_or_else(on_error)? as i32;

    let mut grid = Grid {
        s: input.text.as_bytes(),
        width,
        extra_obstacle_position: (-1, -1),
    };

    if grid.s.len() != ((grid.width + 1) * grid.width - 1) as usize {
        return Err("Invalid input - not a rectangle".to_string());
    }

    let mut visited = [U256::default(); 192];
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

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

    loop {
        let dir = directions[current_direction_idx];
        let new_position = (current_position.0 + dir.0, current_position.1 + dir.1);
        match grid.at(new_position) {
            b'L' => {
                if input.is_part_one() {
                    return Ok(visited.iter().map(U256::count_ones).sum());
                }
                break;
            }
            b'#' => {
                current_direction_idx = (current_direction_idx + 1) % 4;
            }
            _ => {
                current_position = new_position;
                visited[current_position.1 as usize].set_bit(current_position.0 as usize);
            }
        }
    }

    let mut num_loops = 0;
    for x in 0..grid.width {
        'inner: for y in 0..grid.width {
            if !visited[y as usize].is_bit_set(x as usize) {
                // The guard never visits this location - no need to simulate placing an obstacle here.
                continue;
            } else if (x, y) == initial_position {
                // "The new obstruction can't be placed at the guard's starting position
                // - the guard is there right now and would notice."
                continue;
            }

            grid.extra_obstacle_position = (x, y);
            current_position = initial_position;
            current_direction_idx = 0;
            // Indexed by direction idx:
            let mut repeated_visit = [[U256::default(); 192]; 4];
            repeated_visit[current_direction_idx][current_position.1 as usize]
                .set_bit(current_position.0 as usize);

            loop {
                let dir = directions[current_direction_idx];
                let new_position = (current_position.0 + dir.0, current_position.1 + dir.1);
                match grid.at(new_position) {
                    b'L' => {
                        continue 'inner;
                    }
                    b'#' => {
                        current_direction_idx = (current_direction_idx + 1) % 4;
                        if repeated_visit[current_direction_idx][current_position.1 as usize]
                            .is_bit_set(current_position.0 as usize)
                        {
                            num_loops += 1;
                            continue 'inner;
                        }
                        repeated_visit[current_direction_idx][current_position.1 as usize]
                            .set_bit(current_position.0 as usize);
                    }
                    _ => {
                        current_position = new_position;
                    }
                }
            }
        }
    }

    Ok(num_loops)
}

struct Grid<'a> {
    s: &'a [u8],
    width: i32,
    extra_obstacle_position: (i32, i32),
}

impl Grid<'_> {
    fn at(&self, position: (i32, i32)) -> u8 {
        if position.0 < 0 || position.0 >= self.width || position.1 < 0 || position.1 >= self.width
        {
            return b'L';
        } else if position == self.extra_obstacle_position {
            return b'#';
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
