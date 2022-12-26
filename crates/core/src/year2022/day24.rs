use std::collections::VecDeque;

use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<i32, String> {
    let mut valley = Valley::parse(input.text).ok_or("Invalid input")?;

    let start_pos = (0, -1);
    let goal_post = (valley.width as i32 - 1, valley.height as i32);
    let trip_length = find_shortest(&mut valley, 0, start_pos, goal_post)?;
    if input.is_part_one() {
        Ok(trip_length)
    } else {
        let trip_length = find_shortest(&mut valley, trip_length, goal_post, start_pos)?;
        find_shortest(&mut valley, trip_length, start_pos, goal_post)
    }
}

fn find_shortest(
    valley: &mut Valley,
    start_minute: i32,
    start_pos: (i32, i32),
    end_pos: (i32, i32),
) -> Result<i32, String> {
    let mut to_visit = VecDeque::new();
    let mut last_minute = start_minute;
    to_visit.push_back((start_minute, start_pos));

    while let Some((minute, (x, y))) = to_visit.pop_front() {
        if minute != last_minute {
            last_minute = minute;
            valley.reset_visited();
        }
        for (nx, ny) in [(x, y), (x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)] {
            if valley.can_go_to(nx, ny, (minute + 1) as usize) && valley.mark_visited(nx, ny) {
                if nx == end_pos.0 && ny == end_pos.1 {
                    return Ok(minute + 1);
                }
                to_visit.push_back((minute + 1, (nx, ny)));
            }
        }
    }
    Err("No solution found".to_string())
}

struct Valley {
    width: usize,
    height: usize,
    cells: Vec<u8>,
    visited: Vec<bool>,
}

impl Valley {
    fn parse(input: &str) -> Option<Self> {
        let num_lines = input.lines().count();
        let mut lines = input.lines();
        let line = lines.next()?;

        let width = line.len() - 2;
        let height = num_lines - 2;

        let cells = lines
            .take(height)
            .flat_map(|line| line.bytes().skip(1).take(width))
            .collect::<Vec<u8>>();

        (cells.len() == width * height).then_some(Self {
            width,
            height,
            cells,
            visited: vec![false; width * height],
        })
    }

    fn at(&self, x: i32, y: i32) -> u8 {
        let x = x.rem_euclid(self.width as i32);
        let y = y.rem_euclid(self.height as i32);
        self.cells[y as usize * self.width + x as usize]
    }

    fn mark_visited(&mut self, x: i32, y: i32) -> bool {
        if (x, y) == (0, -1) || (x, y) == (self.width as i32 - 1, self.height as i32) {
            return true;
        }
        if self.visited[y as usize * self.width + x as usize] {
            return false;
        }
        self.visited[y as usize * self.width + x as usize] = true;
        true
    }

    fn reset_visited(&mut self) {
        self.visited.fill(false);
    }

    fn can_go_to(&self, x: i32, y: i32, minute: usize) -> bool {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return (x, y) == (0, -1) || (x, y) == (self.width as i32 - 1, self.height as i32);
        }
        let minute = minute as i32;
        self.at(x - minute, y) != b'>'
            && self.at(x + minute, y) != b'<'
            && self.at(x, y - minute) != b'v'
            && self.at(x, y + minute) != b'^'
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
    test_part_one!(test_input => 18);
    test_part_two!(test_input => 54);

    let real_input = include_str!("day24_input.txt");
    test_part_one!(real_input => 242);
    test_part_two!(real_input => 720);
}
