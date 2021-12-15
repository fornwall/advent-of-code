use crate::input::Input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Graph {
    risk_levels: Vec<u8>,
    visited: Vec<bool>,
    width: usize,
    height: usize,
}

impl Graph {
    fn parse(text: &str, multiplier: usize) -> Result<Self, String> {
        let original_height = text.lines().count();
        let original_width = text.lines().next().unwrap_or_default().len();
        if original_height < 1 || original_width < 1 {
            return Err("Too small input".to_string());
        }

        let height = original_height * multiplier;
        let width = original_width * multiplier;

        let mut risk_levels = vec![0; width * height];
        let visited = vec![false; width * height];
        for (y, line) in text.lines().enumerate() {
            if line.len() != original_width {
                return Err("Not all lines have equal length".to_string());
            }
            for (x, byte) in line.bytes().enumerate() {
                let risk_level = byte - b'0';
                risk_levels[x + y * width] = risk_level;
                for mx in 0..multiplier {
                    for my in 0..multiplier {
                        let risk_level_unwrapped = risk_level + (mx as u8 + my as u8);
                        let risk_level_wrapped = 1 + (risk_level_unwrapped - 1) % 9;
                        risk_levels[x + (y + my * original_height) * width + mx * original_width] =
                            risk_level_wrapped;
                    }
                }
            }
        }
        Ok(Self {
            risk_levels,
            visited,
            width,
            height,
        })
    }

    fn mark_visited(&mut self, x: usize, y: usize) {
        self.visited[x + y * self.width] = true;
    }

    fn is_visited(&self, x: usize, y: usize) -> bool {
        self.visited[x + y * self.width]
    }

    fn risk_level_at(&self, x: usize, y: usize) -> u8 {
        self.risk_levels[x + y * self.width]
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        (0..(self.width as i32)).contains(&x) && (0..(self.height as i32)).contains(&y)
    }
}

#[derive(Eq, PartialEq, Clone, PartialOrd, Ord)]
struct State {
    estimate: u64,
    risk: u64,
    x: usize,
    y: usize,
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut graph = Graph::parse(input.text, input.part_values(1, 5))?;
    let destination = (graph.width - 1, graph.height - 1);

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(State {
        estimate: 0,
        risk: 0,
        x: 0,
        y: 0,
    }));
    graph.mark_visited(0, 0);

    while let Some(Reverse(state)) = to_visit.pop() {
        if (state.x, state.y) == destination {
            return Ok(state.risk);
        }

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let new_x = state.x as i32 + dx;
            let new_y = state.y as i32 + dy;
            if graph.contains(new_x, new_y) && !graph.is_visited(new_x as usize, new_y as usize) {
                let new_risk =
                    state.risk + u64::from(graph.risk_level_at(new_x as usize, new_y as usize));
                let estimate = new_risk
                    + (destination.0 - new_x as usize) as u64
                    + (destination.1 - new_y as usize) as u64;
                graph.mark_visited(new_x as usize, new_y as usize);
                to_visit.push(Reverse(State {
                    estimate,
                    risk: new_risk,
                    x: new_x as usize,
                    y: new_y as usize,
                }))
            }
        }
    }

    Err("No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};

    let example = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    test_part_one!(example => 40);
    test_part_two!(example => 315);

    test_part_one_error!("" => "Too small input");
    test_part_one_error!("a\nab" => "Not all lines have equal length");

    let real_input = include_str!("day15_input.txt");
    test_part_one!(real_input => 790);
    test_part_two!(real_input => 2998);
}
