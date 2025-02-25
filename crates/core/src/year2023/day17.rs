use crate::common::array_stack::ArrayStack;
use crate::common::priority_queueu::PriorityQueue;
use crate::common::u256::U256;
use crate::input::{Input, on_error};

const WORK_QUEUE_MAX_SIZE: usize = 40_000;
const MAX_GRID_SIZE: usize = 192;
// 2 bits (0b11) for direction (north/east/south/west)
// PART1:
// Amount in the range [1,3], encoded with:
// 1 => 0b0, 2 => 0b1, 3 => 0b10
// PART2:
// Amount in the range [4,10], encoded with:
// 4 => 0b0, 5 => 0b1, 6 => 0b10, 7 => 0b11, 8 => 0b100, 9 => 0b101, 10 => 0b110
const DIRECTION_AND_AMOUNT_SIZE: usize = 0b11_110;
const VISITED_SIZE: usize = MAX_GRID_SIZE * DIRECTION_AND_AMOUNT_SIZE;

#[allow(clippy::similar_names)]
pub fn solve(input: &Input) -> Result<u16, String> {
    let part2 = input.is_part_two();

    let map = Map::parse(input.text.as_bytes())?;
    if map.num_cols < 4 || map.num_rows < 4 {
        return Err("Too small map".to_string());
    }

    let mut visited = ArrayStack::<{ VISITED_SIZE }, U256>::with_len(VISITED_SIZE);
    let mut to_visit =
        PriorityQueue::<{ WORK_QUEUE_MAX_SIZE }, (u16, u16, u8, u8, StepsInDirection)>::new();

    let initial = if part2 { 4_usize } else { 1 };
    for (x, y, direction) in [
        (initial, 0, Direction::Right),
        (0, initial, Direction::Down),
    ] {
        let steps_in_direction = StepsInDirection::new(initial as u8, direction, part2);
        let cost = u16::from(map.get(x, y))
            + u16::from(if part2 {
                if x == 0 {
                    map.get(0, y - 1) + map.get(0, y - 2) + map.get(0, y - 3)
                } else {
                    map.get(x - 1, 0) + map.get(x - 2, 0) + map.get(x - 3, 0)
                }
            } else {
                0
            });
        let cost_plus_heuristic = cost + map.heuristic(x, y);
        to_visit.push((
            cost_plus_heuristic,
            cost,
            x as u8,
            y as u8,
            steps_in_direction,
        ))?;
    }

    while let Some((_, cost, x, y, steps)) = to_visit.pop() {
        let array_offset = steps.array_offset(i16::from(y));
        if visited.elements[array_offset].is_bit_set(x as usize) {
            continue;
        }

        if x as usize == map.num_cols - 1 && y as usize == map.num_rows - 1 {
            return Ok(cost);
        }

        visited.elements[array_offset].set_bit(x as usize);

        for turn in [0, 1, 2] {
            let multiplier = if turn != 0 && part2 { 4 } else { 1 };
            let new_num_steps = if turn == 0 {
                let existing_steps = steps.num_steps(part2);
                if existing_steps == if part2 { 10 } else { 3 } {
                    continue;
                } else {
                    existing_steps + 1
                }
            } else {
                multiplier
            };
            let direction = steps.direction();
            let new_direction = match turn {
                0 => direction,
                1 => direction.turn_left(),
                _ => direction.turn_right(),
            };
            let (dx, dy) = new_direction.xy();
            let (x, y) = (i16::from(x), i16::from(y));
            let nx = x + dx * i16::from(multiplier);
            let ny = y + dy * i16::from(multiplier);
            if nx < 0 || nx as usize >= map.num_cols || ny < 0 || ny as usize >= map.num_rows {
                continue;
            }
            let nx_usize = nx as usize;
            let ny_usize = ny as usize;

            let new_cost = cost
                + u16::from(map.get(nx_usize, ny_usize))
                + if multiplier == 4 {
                    (1..=3)
                        .map(|s| u16::from(map.get((x + dx * s) as usize, (y + dy * s) as usize)))
                        .sum()
                } else {
                    0
                };

            let new_steps = StepsInDirection::new(new_num_steps, new_direction, part2);
            if !visited.elements[new_steps.array_offset(ny)].is_bit_set(nx_usize) {
                let new_cost_plus_heuristic = new_cost + map.heuristic(nx_usize, ny_usize);
                to_visit.push((
                    new_cost_plus_heuristic,
                    new_cost,
                    nx as u8,
                    ny as u8,
                    new_steps,
                ))?;
            }
        }
    }

    Err("No solution found".to_string())
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}

impl Direction {
    const fn from_idx(idx: i8) -> Self {
        match idx.rem_euclid(4) {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            _ => Self::Left,
        }
    }
    const fn turn_right(self) -> Self {
        let idx = self as i8;
        Self::from_idx(idx + 1)
    }
    const fn turn_left(self) -> Self {
        let idx = self as i8;
        Self::from_idx(idx - 1)
    }
    const fn xy(self) -> (i16, i16) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}

#[derive(PartialOrd, PartialEq, Copy, Clone, Default)]
struct StepsInDirection {
    bits: u8,
}

impl StepsInDirection {
    fn new(num_steps: u8, direction: Direction, part2: bool) -> Self {
        if part2 {
            debug_assert!(num_steps > 3);
            debug_assert!(num_steps <= 10);
            Self {
                bits: ((num_steps - 4) << 2) | (direction as u8),
            }
        } else {
            debug_assert!(num_steps > 0);
            debug_assert!(num_steps <= 3);
            Self {
                bits: ((num_steps - 1) << 2) | (direction as u8),
            }
        }
    }
    const fn num_steps(self, part2: bool) -> u8 {
        (self.bits >> 2) + if part2 { 4 } else { 1 }
    }
    const fn direction(self) -> Direction {
        Direction::from_idx((self.bits & 0b11) as i8)
    }
    const fn array_offset(self, y: i16) -> usize {
        (self.bits as usize) * MAX_GRID_SIZE + (y as usize)
    }
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
        if (bytes.len() + 1) % (num_cols + 1) != 0 {
            return Err(on_error());
        }
        let num_rows = (bytes.len() + 1) / (num_cols + 1);
        Ok(Self {
            bytes,
            num_rows,
            num_cols,
        })
    }

    const fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        (self.num_cols + 1) * y + x
    }

    const fn get(&self, x: usize, y: usize) -> u8 {
        self.bytes[self.xy_to_idx(x, y)] - b'0'
    }

    const fn heuristic(&self, x: usize, y: usize) -> u16 {
        (self.num_cols - 1 - x) as u16 + (self.num_rows - 1 - y) as u16
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let steps_in_direction = StepsInDirection::new(2, Direction::Down, false);
    assert_eq!(steps_in_direction.num_steps(false), 2);
    assert!(steps_in_direction.direction() == Direction::Down);
    let steps_in_direction = StepsInDirection::new(4, Direction::Down, true);
    assert_eq!(steps_in_direction.num_steps(true), 4);
    assert!(steps_in_direction.direction() == Direction::Down);
    let steps_in_direction = StepsInDirection::new(10, Direction::Down, true);
    assert_eq!(steps_in_direction.num_steps(true), 10);
    assert!(steps_in_direction.direction() == Direction::Down);

    let test_input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    test_part_one_no_allocations!(test_input => 102);
    test_part_two_no_allocations!(test_input => 94);

    let test_input = "111111111111
999999999991
999999999991
999999999991
999999999991";
    test_part_two_no_allocations!(test_input => 71);

    let real_input = include_str!("day17_input.txt");
    test_part_one_no_allocations!(real_input => 1039);
    test_part_two_no_allocations!(real_input => 1201);
}
