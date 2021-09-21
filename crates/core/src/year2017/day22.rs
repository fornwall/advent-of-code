use crate::input::Input;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
enum NodeFlag {
    Weakened,
    Infected,
    Flagged,
}

const fn turn(direction: (i32, i32), right: bool) -> (i32, i32) {
    if right {
        (-direction.1, direction.0)
    } else {
        (direction.1, -direction.0)
    }
}

const fn reverse(direction: (i32, i32)) -> (i32, i32) {
    (-direction.0, -direction.1)
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut map = HashMap::new();

    let mut cols = -1;
    let mut rows = -1;
    for (line_idx, line) in input.text.lines().enumerate() {
        rows += 1;
        for (char_idx, char) in line.chars().enumerate() {
            if rows == 0 {
                cols += 1;
            }
            if char == '#' {
                map.insert((char_idx as i32, line_idx as i32), NodeFlag::Infected);
            }
        }
    }

    let mut carrier_position = (cols / 2 + cols % 2, rows / 2 + rows % 2);
    let mut carrier_direction = (0, -1);
    let mut bursts_causing_infection = 0;
    for _burst in 0..input.part_values(10_000, 10_000_000) {
        match map.entry(carrier_position) {
            Entry::Vacant(entry) => {
                carrier_direction = turn(carrier_direction, false);

                if input.is_part_one() {
                    // Clean nodes become infected.
                    bursts_causing_infection += 1;
                    entry.insert(NodeFlag::Infected);
                } else {
                    // Clean nodes become weakened.
                    entry.insert(NodeFlag::Weakened);
                }
            }
            Entry::Occupied(mut entry) => {
                match entry.get() {
                    NodeFlag::Weakened => {
                        // Weakened nodes become infected.
                        bursts_causing_infection += 1;
                        entry.insert(NodeFlag::Infected);
                    }
                    NodeFlag::Infected => {
                        carrier_direction = turn(carrier_direction, true);

                        if input.is_part_one() {
                            // Infected nodes become cleaned.
                            entry.remove();
                        } else {
                            // Infected nodes become flagged.
                            entry.insert(NodeFlag::Flagged);
                        }
                    }
                    NodeFlag::Flagged => {
                        carrier_direction = reverse(carrier_direction);

                        // Flagged nodes become clean.
                        entry.remove();
                    }
                }
            }
        }

        carrier_position = (
            carrier_position.0 + carrier_direction.0,
            carrier_position.1 + carrier_direction.1,
        );
    }

    Ok(bursts_causing_infection)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day22_input.txt");
    test_part_one!(real_input => 5246);
    test_part_two!(real_input => 2_512_059);
}
