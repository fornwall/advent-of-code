use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::input::Input;

type Position = (usize, usize);

struct Graph {
    cells: Vec<u8>,
    visited: Vec<bool>,
    height: usize,
    width: usize,
}

impl Graph {
    fn parse(input: &str) -> Result<(Position, Position, Self), String> {
        let height = input.bytes().filter(|b| *b == b'\n').count() + 1;
        let width = input.lines().next().unwrap_or_default().len();
        let mut start_pos = (0, 0);
        let mut destination_pos = (0, 0);
        let mut cells = vec![0_u8; height * width];
        for (y, line) in input.lines().enumerate() {
            if line.len() != width {
                return Err("Not all rows have equal length".to_string());
            }
            for (x, val) in line.bytes().enumerate() {
                cells[y * width + x] = if val == b'S' {
                    // "Your current position (S) has elevation a"
                    start_pos = (x, y);
                    b'a'
                } else if val == b'E' {
                    // "the location that should get the best signal (E) has elevation z"
                    destination_pos = (x, y);
                    b'z'
                } else {
                    val
                };
            }
        }
        let visited = vec![false; cells.len()];
        Ok((
            start_pos,
            destination_pos,
            Self {
                cells,
                visited,
                height,
                width,
            },
        ))
    }

    fn height_at(&mut self, x: usize, y: usize) -> u8 {
        self.cells[y * self.width + x]
    }

    fn mark_visited(&mut self, x: usize, y: usize) -> bool {
        let old = self.visited[y * self.width + x];
        self.visited[y * self.width + x] = true;
        !old
    }

    fn can_go(&mut self, x: usize, y: usize, dx: i32, dy: i32, part_2: bool) -> Option<Position> {
        if (dx < 0 && x == 0)
            || (dy < 0 && y == 0)
            || (dx > 0 && x + 1 == self.width)
            || (dy > 0 && y + 1 == self.height)
        {
            return None;
        }
        let new_x = (x as i32 + dx) as usize;
        let new_y = (y as i32 + dy) as usize;
        let mut new_height = self.height_at(new_x, new_y);
        let mut old_height = self.height_at(x, y);
        if part_2 {
            std::mem::swap(&mut new_height, &mut old_height);
        }
        (new_height <= old_height + 1 && self.mark_visited(new_x, new_y)).then_some((new_x, new_y))
    }
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let (mut start_pos, destination_pos, mut graph) = Graph::parse(input.text)?;

    if input.is_part_two() {
        start_pos = destination_pos;
    }

    let mut to_visit = BinaryHeap::new();
    graph.mark_visited(start_pos.0, start_pos.1);
    to_visit.push(Reverse((0, start_pos)));

    while let Some(Reverse((cost, pos))) = to_visit.pop() {
        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if let Some(new_pos) = graph.can_go(pos.0, pos.1, dx, dy, input.is_part_two()) {
                let new_cost = cost + 1;
                let at_goal = if input.is_part_one() {
                    new_pos == destination_pos
                } else {
                    graph.height_at(new_pos.0, new_pos.1) == b'a'
                };
                if at_goal {
                    return Ok(new_cost);
                }
                to_visit.push(Reverse((new_cost, new_pos)));
            }
        }
    }
    Err("No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
    test_part_one!(test_input => 31);
    test_part_two!(test_input => 29);

    let real_input = include_str!("day12_input.txt");
    test_part_one!(real_input => 528);
    test_part_two!(real_input => 522);
}
