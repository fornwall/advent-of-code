use crate::common::array_deque::ArrayDeque;
use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<String, String> {
    let mut grid = [[false; 71]; 71];
    for (count, line) in input.text.lines().enumerate() {
        let (x, y) = line.split_once(',').ok_or_else(on_error)?;
        let (x, y) = (
            x.parse::<usize>().map_err(|_| on_error())?,
            y.parse::<usize>().map_err(|_| on_error())?,
        );

        if !(0..71).contains(&x) && !(0..71).contains(&y) {
            return Err(format!("Coordinate out of bounds: {},{}", x, y));
        }
        grid[y][x] = true;

        if input.is_part_one() {
            if count == 1023 {
                return Ok(format!("{}", shortest_path(&grid).ok_or_else(on_error)?));
            }
        } else if count >= 1024 && shortest_path(&grid).is_none() {
            return Ok(format!("{},{}", x, y));
        }
    }

    Err("No solution found".to_string())
}

fn shortest_path(grid: &[[bool; 71]; 71]) -> Option<i32> {
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
            if (0..71).contains(&nx) && (0..71).contains(&ny) && !grid[ny as usize][nx as usize] {
                to_visit.push_back((cost + 1, (nx, ny))).ok()?;
            }
        }
    }

    None
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let real_input = include_str!("day18_input.txt");
    test_part_one_no_allocations!(real_input => "360".to_string());
    test_part_two_no_allocations!(real_input => "58,62".to_string());
}
