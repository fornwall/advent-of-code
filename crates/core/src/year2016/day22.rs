use crate::input::Input;
use std::collections::VecDeque;

struct Node {
    position: (u8, u8),
    size: u16,
    used: u16,
}

impl Node {
    const fn available(&self) -> u16 {
        self.size - self.used
    }
}

fn dist(
    dimensions: (u8, u8),
    mut grid: Vec<bool>,
    start: (u8, u8),
    destination: (u8, u8),
) -> Result<usize, String> {
    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    while let Some((path_length, position)) = queue.pop_front() {
        let grid_idx =
            usize::from(position.1) * usize::from(dimensions.0) + usize::from(position.0);

        if grid[grid_idx] {
            continue;
        }
        if position == destination {
            return Ok(path_length);
        }

        grid[grid_idx] = true;

        if position.0 != 0 {
            queue.push_back((path_length + 1, (position.0 - 1, position.1)));
        }
        if position.0 != dimensions.0 - 1 {
            queue.push_back((path_length + 1, (position.0 + 1, position.1)));
        }
        if position.1 != 0 {
            queue.push_back((path_length + 1, (position.0, position.1 - 1)));
        }
        if position.1 != dimensions.1 - 1 {
            queue.push_back((path_length + 1, (position.0, position.1 + 1)));
        }
    }

    Err(format!("No path found from {start:?} to {destination:?}"))
}

pub fn solve(input: &Input) -> Result<u32, String> {
    let error_mapper = |_| "Invalid input";

    let mut nodes = Vec::new();

    for line in input.text.lines() {
        if let Some(remainder) = line.strip_prefix("/dev/grid/node-") {
            let words = remainder.split_whitespace().collect::<Vec<_>>();

            let name_parts = words[0].split('-').collect::<Vec<_>>();
            let x = name_parts[0][1..].parse::<u8>().map_err(error_mapper)?;
            let y = name_parts[1][1..].parse::<u8>().map_err(error_mapper)?;

            let size = words[1][..words[1].len() - 1]
                .parse::<u16>()
                .map_err(error_mapper)?;
            let used = words[2][..words[2].len() - 1]
                .parse::<u16>()
                .map_err(error_mapper)?;
            nodes.push(Node {
                position: (x, y),
                used,
                size,
            });
        }
    }

    if input.is_part_one() {
        Ok(nodes
            .iter()
            .flat_map(|n1| {
                nodes.iter().filter(move |n2| {
                    n1.position != n2.position && n1.used > 0 && n1.used < n2.available()
                })
            })
            .count() as u32)
    } else {
        // TODO: From https://github.com/galenelias/AdventOfCode_2016/blob/master/src/Day22/mod.rs
        let bottom_right = nodes
            .iter()
            .map(|node| node.position)
            .max()
            .ok_or("No bottom right node")?;
        let dimensions = (bottom_right.0 + 1, bottom_right.1 + 1);
        let wall_threshold = nodes
            .iter()
            .filter(|node| node.position.1 == 0)
            .map(|node| node.size)
            .max()
            .ok_or("No wall threshold")?;
        let mut grid = vec![true; usize::from(dimensions.0) * usize::from(dimensions.1)];

        for node in nodes.iter() {
            grid[usize::from(node.position.1) * usize::from(dimensions.0)
                + usize::from(node.position.0)] = node.size > wall_threshold;
        }

        let empty_pos = nodes
            .iter()
            .find(|node| node.used == 0 && node.size > 0)
            .ok_or("No empty node")?
            .position;
        let payload_pos = (dimensions.0 - 1, 0);
        let dist_to_payload = dist(dimensions, grid.clone(), empty_pos, payload_pos)?;
        let dist_to_home = dist(dimensions, grid, (payload_pos.0 - 1, payload_pos.1), (0, 0))?;
        Ok((dist_to_payload + 5 * dist_to_home) as u32)
    }
}

#[test]
pub fn tests() {
    let real_input = include_str!("day22_input.txt");
    test_part_one!(real_input => 901);
    test_part_two!(real_input => 238);
}
