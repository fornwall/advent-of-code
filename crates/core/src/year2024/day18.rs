use crate::common::array_deque::ArrayDeque;
use crate::common::priority_queueu::PriorityQueue;
use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<String, String> {
    let mut grid = [[u16::MAX; 71]; 71];
    for (count, line) in input.text.lines().enumerate() {
        let (x, y) = line.split_once(',').ok_or_else(on_error)?;
        let (x, y) = (
            x.parse::<usize>().map_err(|_| on_error())?,
            y.parse::<usize>().map_err(|_| on_error())?,
        );

        if !(0..71).contains(&x) && !(0..71).contains(&y) {
            return Err(format!("Coordinate out of bounds: {},{}", x, y));
        }
        grid[y][x] = count as u16;

        if input.is_part_one() && count == 1023 {
            return Ok(format!("{}", shortest_path(&grid).ok_or_else(on_error)?));
        }
    }

    find_first_blocker_byte(&grid)
}

fn shortest_path(grid: &[[u16; 71]; 71]) -> Option<i32> {
    let mut visited = [[false; 71]; 71];
    let mut to_visit = ArrayDeque::<1024, (i32, (i8, i8))>::new();
    to_visit.push_back((0, (0, 0))).ok()?;

    while let Some((cost, (x, y))) = to_visit.pop_front() {
        if (x, y) == (70, 70) {
            return Some(cost);
        }
        if visited[y as usize][x as usize] {
            continue;
        }
        visited[y as usize][x as usize] = true;
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let (nx, ny) = (x + dx, y + dy);
            if (0..71).contains(&nx)
                && (0..71).contains(&ny)
                && grid[ny as usize][nx as usize] == u16::MAX
            {
                to_visit.push_back((cost + 1, (nx, ny))).ok()?;
            }
        }
    }

    None
}

fn find_first_blocker_byte(grid: &[[u16; 71]; 71]) -> Result<String, String> {
    let mut visited = [[false; 71]; 71];
    let mut to_visit = ArrayDeque::<1024, (i8, i8)>::new();
    let mut found_blockers = PriorityQueue::<5000, (i32, (i8, i8))>::new();
    found_blockers.push((-(u16::MAX as i32), (0, 0)))?;
    while let Some((time, (block_x, block_y))) = found_blockers.pop() {
        let time = (-time) as u16;
        to_visit.push_back((block_x, block_y))?;
        while let Some((x, y)) = to_visit.pop_front() {
            if (x, y) == (70, 70) {
                return Ok(format!("{block_x},{block_y}"));
            }
            if visited[y as usize][x as usize] {
                continue;
            }
            visited[y as usize][x as usize] = true;
            for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy);
                if (0..71).contains(&nx) && (0..71).contains(&ny) {
                    let grid_time = grid[ny as usize][nx as usize];
                    if grid_time < time {
                        found_blockers.push((-(grid_time as i32), (nx, ny)))?;
                    } else {
                        to_visit.push_back((nx, ny))?;
                    }
                }
            }
        }
    }

    Err("No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day18_input.txt");
    test_part_one!(real_input => "360".to_string());
    test_part_two!(real_input => "58,62".to_string());
}
