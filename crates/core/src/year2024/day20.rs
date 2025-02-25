use crate::{
    common::array_deque::ArrayDeque,
    input::{Input, on_error},
};

const MAX_GRID_SIZE: usize = 150;
const WORK_QUEUE_MAX_SIZE: usize = 1024;

pub fn solve(input: &Input) -> Result<u32, String> {
    let width = input
        .text
        .lines()
        .next()
        .map(|line| line.len())
        .ok_or_else(on_error)? as i16;
    let grid = Grid {
        s: input.text.as_bytes(),
        width,
    };
    if grid.s.len() != ((grid.width + 1) * grid.width - 1) as usize {
        return Err("Invalid input - not a rectangle".to_string());
    } else if grid.width >= MAX_GRID_SIZE as i16 {
        return Err("Invalid input - too big rectangle".to_string());
    }

    let mut start_location = (0, 0);
    for y in 0..grid.width {
        for x in 0..grid.width {
            if grid.at((x, y)) == b'S' {
                start_location = (x, y);
            }
        }
    }
    if start_location == (0, 0) {
        return Err("No start location".to_string());
    }

    let mut costs = [[u16::MAX; MAX_GRID_SIZE]; MAX_GRID_SIZE];
    let mut to_visit = ArrayDeque::<{ WORK_QUEUE_MAX_SIZE }, (u16, (i16, i16))>::new();
    to_visit.push_back((0, start_location))?;

    while let Some((cost, position)) = to_visit.pop_front() {
        if costs[position.1 as usize][position.0 as usize] <= cost {
            continue;
        }
        costs[position.1 as usize][position.0 as usize] = cost;
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let next = (position.0 + dx, position.1 + dy);
            if grid.at(next) != b'#' {
                to_visit.push_back((cost + 1, next))?;
            }
        }
    }

    // Second pass to find cheats:
    let diamond_size: i16 = input.part_values(2, 20);
    let mut num_great_cheats = 0;
    for y in 1..(grid.width - 1) {
        for x in 1..(grid.width - 1) {
            let cost = costs[y as usize][x as usize];
            if cost == u16::MAX {
                continue;
            }
            for dy in -diamond_size..=diamond_size {
                for dx in (dy.abs() - diamond_size)..=(diamond_size - dy.abs()) {
                    let manhattan_distance = dx.abs() + dy.abs();
                    let cheat = (x + dx, y + dy);
                    if grid.at(cheat) != b'#' {
                        if let Some(gain) = costs[cheat.1 as usize][cheat.0 as usize]
                            .checked_sub(cost + manhattan_distance as u16)
                        {
                            if gain >= 100 {
                                num_great_cheats += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(num_great_cheats)
}

struct Grid<'a> {
    s: &'a [u8],
    width: i16,
}

impl Grid<'_> {
    const fn at(&self, position: (i16, i16)) -> u8 {
        if position.0 < 0 || position.0 >= self.width || position.1 < 0 || position.1 >= self.width
        {
            return b'#';
        }
        self.s[(position.0 + (self.width + 1) * position.1) as usize]
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let real_input = include_str!("day20_input.txt");
    test_part_one_no_allocations!(real_input => 1338);
    test_part_two_no_allocations!(real_input => 975_376);
}
