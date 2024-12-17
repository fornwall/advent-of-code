use crate::common::array_deque::ArrayDeque;
use crate::common::priority_queueu::PriorityQueue;
use crate::common::u256::U256;
use crate::input::{on_error, Input};

const MAX_GRID_SIZE: usize = 142;
const WORK_QUEUE_MAX_SIZE: usize = 8000;

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
    let mut end_location = (0, 0);
    for y in 0..grid.width {
        for x in 0..grid.width {
            match grid.at((x, y)) {
                b'S' => start_location = (x, y),
                b'E' => end_location = (x, y),
                _ => {}
            }
        }
    }
    if start_location == (0, 0) {
        return Err("No start location".to_string());
    } else if end_location == (0, 0) {
        return Err("No end location".to_string());
    }

    let mut costs = [[u32::MAX; MAX_GRID_SIZE * MAX_GRID_SIZE]; 4];
    let mut to_visit =
        PriorityQueue::<{ WORK_QUEUE_MAX_SIZE }, (u32, (i16, i16), Direction)>::new();

    costs[Direction::East.idx()][(start_location.1 * grid.width + start_location.0) as usize] = 0;
    to_visit.push((0, start_location, Direction::East)).unwrap();

    let mut lowest_end_cost = u32::MAX;
    'outer: while let Some((cost, position, direction)) = to_visit.pop() {
        for (next_cost, next_position, next_direction) in [
            (cost + 1000, position, direction.rotate(true)),
            (cost + 1000, position, direction.rotate(false)),
            (cost + 1, direction.advance(position), direction),
        ] {
            let best_cost = costs[next_direction.idx()]
                [(next_position.1 * grid.width + next_position.0) as usize];
            if grid.at(next_position) != b'#' && next_cost < best_cost {
                costs[next_direction.idx()]
                    [(next_position.1 * grid.width + next_position.0) as usize] = next_cost;
                if next_position == end_location {
                    if input.is_part_one() {
                        return Ok(next_cost);
                    } else if lowest_end_cost == u32::MAX {
                        lowest_end_cost = next_cost;
                    } else if next_cost > lowest_end_cost {
                        break 'outer;
                    }
                }

                to_visit
                    .push((next_cost, next_position, next_direction))
                    .unwrap();
            }
        }
    }

    if input.is_part_one() {
        return Err("No solution found".to_string());
    }

    let mut visited = [U256::default(); MAX_GRID_SIZE];
    let mut to_visit = ArrayDeque::<128, (i32, (i16, i16), Direction)>::new();

    visited[end_location.1 as usize].set_bit(end_location.0 as usize);
    for direction in [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ] {
        if costs[direction.idx()][(end_location.1 * grid.width + end_location.0) as usize]
            != u32::MAX
        {
            to_visit.push_back((lowest_end_cost as i32, end_location, direction))?;
        }
    }

    while let Some((cost, position, direction)) = to_visit.pop_front() {
        for (next_cost, next_position, next_direction) in [
            (cost - 1, direction.reverse().advance(position), direction),
            (cost - 1000, position, direction.rotate(true)),
            (cost - 1000, position, direction.rotate(false)),
        ] {
            let seen_cost = costs[next_direction.idx()]
                [(next_position.1 * grid.width + next_position.0) as usize];
            if seen_cost != u32::MAX && next_cost == seen_cost as i32 {
                visited[next_position.1 as usize].set_bit(next_position.0 as usize);
                to_visit.push_back((next_cost, next_position, next_direction))?;
                costs[next_direction.idx()]
                    [(next_position.1 * grid.width + next_position.0) as usize] = u32::MAX;
            }
        }
    }

    Ok(visited.iter().map(|b| b.count_ones()).sum())
}

struct Grid<'a> {
    s: &'a [u8],
    width: i16,
}

impl Grid<'_> {
    const fn at(&self, position: (i16, i16)) -> u8 {
        if position.0 < 0 || position.0 >= self.width || position.1 < 0 || position.1 >= self.width
        {
            return b'@';
        }
        self.s[(position.0 + (self.width + 1) * position.1) as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Default for Direction {
    fn default() -> Self {
        Self::North
    }
}

impl Direction {
    const fn rotate(self, clockwise: bool) -> Self {
        match (self, clockwise) {
            (Self::North, true) => Self::East,
            (Self::North, false) => Self::West,
            (Self::East, true) => Self::South,
            (Self::East, false) => Self::North,
            (Self::South, true) => Self::West,
            (Self::South, false) => Self::East,
            (Self::West, true) => Self::North,
            (Self::West, false) => Self::South,
        }
    }
    const fn idx(self) -> usize {
        self as usize
    }
    const fn advance(self, position: (i16, i16)) -> (i16, i16) {
        match self {
            Self::North => (position.0, position.1 + 1),
            Self::East => (position.0 + 1, position.1),
            Self::South => (position.0, position.1 - 1),
            Self::West => (position.0 - 1, position.1),
        }
    }
    const fn reverse(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    test_part_one_no_allocations!(test_input => 7036);
    test_part_two_no_allocations!(test_input => 45);
    let test_input = "#######
###..E#
###..##
##....#
##..###
#S.####
#######";
    test_part_two_no_allocations!(test_input => 12);

    let real_input = include_str!("day16_input.txt");
    test_part_one_no_allocations!(real_input => 90_440);
    test_part_two_no_allocations!(real_input => 479);
}
