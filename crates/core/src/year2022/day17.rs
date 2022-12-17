use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    let target_rocks_count = input.part_values(2022, 1_000_000_000_000);
    let mut grid = Grid::new();

    let mut seen = HashMap::new();
    let mut direction_iterator = input.text.bytes().enumerate().cycle();

    for (current_rock_count, &(rock_width, rock_bitmask)) in ROCK_SEQUENCE
        .iter()
        .cycle()
        .enumerate()
        .take(target_rocks_count)
    {
        // "Each rock appears so that its left edge is two units away from the left
        // wall and its bottom edge is three units above the highest rock in the
        // room (or the floor, if there isn't one)."
        let (mut rock_bottom_y, mut rock_left_x) = (grid.highest_rock + 3, 2);

        for (direction_idx, direction) in direction_iterator.by_ref() {
            let pushed_x = rock_left_x + if direction == b'<' { -1 } else { 1 };
            let push_possible = pushed_x >= 0
                && pushed_x + rock_width as i32 <= Grid::WIDTH as i32
                && grid.can_place_rock(rock_bitmask, pushed_x as usize, rock_bottom_y);

            if push_possible {
                rock_left_x = pushed_x;
            }

            if rock_bottom_y != 0
                && grid.can_place_rock(rock_bitmask, rock_left_x as usize, rock_bottom_y - 1)
            {
                rock_bottom_y -= 1;
                continue;
            }

            grid.settle_rock(rock_bitmask, rock_left_x as usize, rock_bottom_y);

            if input.is_part_two() {
                let rock_and_direction_idx =
                    (current_rock_count % ROCK_SEQUENCE.len(), direction_idx);

                match seen.entry(rock_and_direction_idx) {
                    Entry::Occupied(entry) => {
                        let (last_seen_rock_drop_iteration, last_seen_highest_rock) = *entry.get();
                        let rocks_per_cycle = current_rock_count - last_seen_rock_drop_iteration;
                        let remaining_rocks = target_rocks_count - current_rock_count;

                        if remaining_rocks % rocks_per_cycle == 0 {
                            let remaining_cycles = remaining_rocks / rocks_per_cycle;
                            let highest_rock_growth = grid.highest_rock - last_seen_highest_rock;
                            return Ok(
                                grid.highest_rock + remaining_cycles * highest_rock_growth - 1
                            );
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert((current_rock_count, grid.highest_rock));
                    }
                }
            }
            break;
        }
    }

    Ok(grid.highest_rock)
}

type Rock = u16;

// Bitmasks start from lower left corner.
// (width, rock_bitmask)
const ROCK_SEQUENCE: [(usize, Rock); 5] = [
    // "####"
    (4, 0b0000_0000_0000_1111),
    // ".#."
    // "###"
    // ".#."
    (3, 0b0000_0010_0111_0010),
    // "..#"
    // "..#"
    // "###"
    (3, 0b0000_0100_0100_0111),
    // "#"
    // "#"
    // "#"
    // "#"
    (1, 0b0001_0001_0001_0001),
    // "##"
    // "##"
    (2, 0b0000_0000_0011_0011),
];

struct Grid {
    data: [u8; Self::MAX_HEIGHT],
    highest_rock: usize,
}

impl Grid {
    const WIDTH: usize = 7;
    const MAX_HEIGHT: usize = 8192;

    const fn new() -> Self {
        Self {
            data: [0; Self::MAX_HEIGHT],
            highest_rock: 0,
        }
    }

    fn can_place_rock(&self, rock: Rock, left_edge_x: usize, bottom_edge_y: usize) -> bool {
        rock & ((u16::from(self.data[bottom_edge_y] >> left_edge_x) & 0b1111)
            + ((u16::from(self.data[bottom_edge_y + 1] >> left_edge_x) & 0b1111) << 4)
            + ((u16::from(self.data[bottom_edge_y + 2] >> left_edge_x) & 0b1111) << 8)
            + ((u16::from(self.data[bottom_edge_y + 3] >> left_edge_x) & 0b1111) << 12))
            == 0
    }

    fn settle_rock(&mut self, rock: Rock, left_edge_x: usize, bottom_edge_y: usize) {
        self.data[bottom_edge_y] |= ((rock & 0b1111) as u8) << left_edge_x;
        self.data[bottom_edge_y + 1] |= (((rock >> 4) & 0b1111) as u8) << left_edge_x;
        self.data[bottom_edge_y + 2] |= (((rock >> 8) & 0b1111) as u8) << left_edge_x;
        self.data[bottom_edge_y + 3] |= (((rock >> 12) & 0b1111) as u8) << left_edge_x;

        let rock_height = (4 - rock.leading_zeros() / 4) as usize;
        self.highest_rock = self.highest_rock.max(bottom_edge_y + rock_height);
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    test_part_one!(test_input => 3068);
    test_part_two!(test_input => 1_514_285_714_288);

    let real_input = include_str!("day17_input.txt");
    test_part_one!(real_input => 3117);
    test_part_two!(real_input => 1_553_314_121_019);
}
