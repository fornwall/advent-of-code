use crate::common::array_deque::ArrayDeque;
use crate::common::u256::U256;
use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAX_GRID_SIZE: usize = 150;
    const MAX_STACK_SIZE: usize = 4;

    let map = Map::parse(input.text.trim().as_bytes())?;
    if map.num_rows > MAX_GRID_SIZE || map.num_cols > MAX_GRID_SIZE {
        return Err(format!("Invalid input - max grid size is {MAX_GRID_SIZE}"));
    }

    let start_idx = map
        .bytes
        .iter()
        .enumerate()
        .find_map(|(idx, &b)| (b == b'S').then_some(idx))
        .ok_or_else(on_error)?;
    let (start_x, start_y) = map.idx_to_xy(start_idx);

    let mut visited_bitmask = [U256::default(); MAX_GRID_SIZE];
    visited_bitmask[start_y].set_bit(start_x);

    let mut to_visit = ArrayDeque::<MAX_STACK_SIZE, (usize, u64)>::new();
    to_visit.push_back((start_idx, 0))?;

    let mut max_distance = 0;

    while let Some((idx, distance)) = to_visit.pop_front() {
        let (x, y) = map.idx_to_xy(idx);
        let from = map.bytes[idx];
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < map.num_cols as i32 && ny >= 0 && ny < map.num_rows as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                let to = map.get(nx, ny);
                if matches!(
                    (from, to, dx, dy),
                    (b'S' | b'-' | b'L' | b'F', b'-' | b'J' | b'7', 1, 0)
                        | (b'S' | b'-' | b'J' | b'7', b'-' | b'L' | b'F', -1, 0)
                        | (b'S' | b'|' | b'F' | b'7', b'|' | b'L' | b'J', 0, 1)
                        | (b'S' | b'|' | b'L' | b'J', b'|' | b'F' | b'7', 0, -1)
                ) && !visited_bitmask[ny].is_bit_set(nx)
                {
                    visited_bitmask[ny].set_bit(nx);
                    max_distance = distance + 1;
                    to_visit.push_back((map.xy_to_idx(nx, ny), max_distance))?;
                }
            }
        }
    }

    if input.is_part_one() {
        return Ok(max_distance);
    }

    let mut inside_loop_count = 0;
    for (y, bitset) in visited_bitmask.iter_mut().enumerate() {
        let mut inside_loop = false;
        for x in 0..map.num_cols {
            if bitset.is_bit_set(x) {
                let b = map.get(x, y);
                if matches!(b, b'|' | b'L' | b'J')
                    || (b == b'S' && (y != 0 && matches!(map.get(x, y - 1), b'|' | b'7' | b'F')))
                {
                    inside_loop = !inside_loop;
                }
            } else if inside_loop {
                inside_loop_count += 1;
            }
        }
    }
    Ok(inside_loop_count)
}

struct Map<'a> {
    bytes: &'a [u8],
    num_rows: usize,
    num_cols: usize,
}

impl<'a> Map<'a> {
    fn parse(bytes: &'a [u8]) -> Result<Self, String> {
        let num_cols = bytes
            .iter()
            .position(|&b| b == b'\n')
            .ok_or_else(on_error)?;
        if !(bytes.len() + 1).is_multiple_of(num_cols + 1) {
            return Err(on_error());
        }
        let num_rows = (bytes.len() + 1) / (num_cols + 1);
        Ok(Self {
            bytes,
            num_rows,
            num_cols,
        })
    }

    const fn idx_to_xy(&self, idx: usize) -> (usize, usize) {
        let y = idx / (self.num_cols + 1);
        let x = idx - y * (self.num_cols + 1);
        (x, y)
    }

    const fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        (self.num_cols + 1) * y + x
    }

    const fn get(&self, x: usize, y: usize) -> u8 {
        self.bytes[self.xy_to_idx(x, y)]
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = ".....
.S-7.
.|.|.
.L-J.
.....";
    test_part_one_no_allocations!(test_input => 4);

    let test_input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    test_part_one_no_allocations!(test_input => 8);

    let test_input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    test_part_two_no_allocations!(test_input => 4);

    let test_input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
    test_part_two_no_allocations!(test_input => 4);

    let real_input = include_str!("day10_input.txt");
    test_part_one_no_allocations!(real_input => 6875);
    test_part_two_no_allocations!(real_input => 471);
}
