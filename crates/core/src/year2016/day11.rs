use crate::Input;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Clone, Default, Eq, PartialEq, Hash, Copy)]
struct Floor {
    generators: u8,
    microchips: u8,
}

impl Floor {
    fn add_item(&mut self, chip: bool, isotope_id: u8) {
        let bit_mask = 1 << isotope_id;
        if chip {
            self.microchips |= bit_mask;
        } else {
            self.generators |= bit_mask;
        }
    }

    fn remove_item(&mut self, chip: bool, isotope_id: u8) {
        let bit_mask = !(1 << isotope_id);
        if chip {
            self.microchips &= bit_mask;
        } else {
            self.generators &= bit_mask;
        }
    }

    const fn is_valid(self) -> bool {
        let contains_generator = self.generators != 0;
        let contains_unshielded_microchip = self.microchips & self.generators != self.microchips;
        !(contains_generator && contains_unshielded_microchip)
    }

    const fn count_items(self) -> u32 {
        self.generators.count_ones() + self.microchips.count_ones()
    }
}

#[derive(Clone, Eq)]
struct State {
    current_floor: i8,
    floors: [Floor; 4],
}

impl State {
    fn pairs(&self) -> usize {
        let mut result = 0;
        let mut current_idx = 0;
        for (floor_idx, floor) in self.floors.iter().enumerate() {
            for offset in 0..8 {
                let bit_mask = 1 << offset;
                if floor.microchips & bit_mask != 0 {
                    for (match_floor_idx, match_floor) in self.floors.iter().enumerate() {
                        if match_floor.generators & bit_mask != 0 {
                            result |=
                                (floor_idx << current_idx) + (match_floor_idx << (current_idx + 2));
                            current_idx += 4;
                        }
                    }
                }
            }
        }
        result
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.current_floor.hash(hasher);
        self.pairs().hash(hasher);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.current_floor == other.current_floor && self.pairs() == other.pairs()
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.current_floor.cmp(&self.current_floor)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str, part2: bool) -> Result<[Floor; 4], String> {
    let mut name_to_id = HashMap::new();
    let mut current_id = 0_u8;
    let mut initial_floors = [
        Floor::default(),
        Floor::default(),
        Floor::default(),
        Floor::default(),
    ];

    for (floor_idx, line) in input.lines().enumerate() {
        // "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
        // The second floor contains a hydrogen generator.
        // The third floor contains a lithium generator.
        // The fourth floor contains nothing relevant."
        let words = line.split(' ').collect::<Vec<_>>();
        for (word_idx, &word) in words.iter().enumerate() {
            let (isotope_name, microchip) = if word_idx > 0 && word.starts_with("microchip") {
                let isotope_name = words[word_idx - 1]
                    .strip_suffix("-compatible")
                    .ok_or("Invalid syntax - not $ISOTYPE-compatible before 'microchip'")?;
                (isotope_name, true)
            } else if word_idx > 0 && word.starts_with("generator") {
                let isotope_name = words[word_idx - 1];
                (isotope_name, false)
            } else {
                continue;
            };

            let isotope_id = *name_to_id
                .entry(isotope_name.to_string())
                .or_insert_with(|| {
                    current_id += 1;
                    current_id - 1
                });
            if isotope_id == 6 {
                return Err("Too many isotopes - max supported is 5".to_string());
            }
            let bit_mask = 1 << isotope_id;

            if microchip {
                initial_floors[floor_idx].microchips |= bit_mask;
            } else {
                initial_floors[floor_idx].generators |= bit_mask;
            }
        }
    }

    if part2 {
        let elerium_id = current_id + 1;
        let dilithium_id = current_id + 2;
        initial_floors[0].add_item(true, elerium_id);
        initial_floors[0].add_item(false, elerium_id);
        initial_floors[0].add_item(true, dilithium_id);
        initial_floors[0].add_item(false, dilithium_id);
    }

    Ok(initial_floors)
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let initial_floors = parse_input(input.text, input.is_part_two())?;
    let mut to_visit = BinaryHeap::new();
    let mut visited_states = HashSet::new();

    let initial_state = State {
        // "When you enter the containment area, you and the elevator will start on the first floor":
        current_floor: 0,
        floors: initial_floors,
    };

    to_visit.push(Reverse((0, 0, initial_state.clone())));
    visited_states.insert(initial_state);

    while let Some(Reverse((_, visited_state_cost, visited_state))) = to_visit.pop() {
        if visited_state
            .floors
            .iter()
            .take(3)
            .all(|floor| floor.count_items() == 0)
        {
            // If floor 0-3 is empty we're done.
            return Ok(visited_state_cost);
        }

        for direction in &[-1, 1] {
            let new_floor = visited_state.current_floor + direction;
            if !(0..=3).contains(&new_floor) {
                continue;
            }
            if *direction == -1
                && visited_state
                    .floors
                    .iter()
                    .take(visited_state.current_floor as usize)
                    .all(|floor| floor.count_items() == 0)
            {
                // Do not bring anything down if every floor beneath current is empty.
                continue;
            }

            let current_floor = visited_state.floors[visited_state.current_floor as usize];
            for &first_moved_is_chip in &[true, false] {
                for first_offset in 0..8 {
                    let contains_first_item = if first_moved_is_chip {
                        current_floor.microchips
                    } else {
                        current_floor.generators
                    } & (1 << first_offset)
                        != 0;
                    if !contains_first_item {
                        continue;
                    }

                    for &second_moved_is_chip in if first_moved_is_chip {
                        [true, false].iter()
                    } else {
                        [false].iter()
                    } {
                        for second_offset in 0..=(if first_moved_is_chip == second_moved_is_chip {
                            first_offset
                        } else {
                            7
                        }) {
                            let contains_second_item = if second_moved_is_chip {
                                current_floor.microchips
                            } else {
                                current_floor.generators
                            } & (1 << second_offset)
                                != 0;
                            if !contains_second_item {
                                continue;
                            }

                            let mut new_floors = visited_state.floors;

                            new_floors[visited_state.current_floor as usize]
                                .remove_item(first_moved_is_chip, first_offset);
                            new_floors[new_floor as usize]
                                .add_item(first_moved_is_chip, first_offset);

                            if (first_moved_is_chip, first_offset)
                                != (second_moved_is_chip, second_offset)
                            {
                                new_floors[visited_state.current_floor as usize]
                                    .remove_item(second_moved_is_chip, second_offset);
                                new_floors[new_floor as usize]
                                    .add_item(second_moved_is_chip, second_offset);
                            }

                            if !new_floors.iter().all(|&floor| floor.is_valid()) {
                                continue;
                            }

                            let new_cost = visited_state_cost + 1;
                            let new_state = State {
                                current_floor: new_floor,
                                floors: new_floors,
                            };

                            let do_insert = visited_states.insert(new_state.clone());
                            if do_insert {
                                // Encourage moving things up:
                                let heuristic = (new_state.floors[0].count_items() * 3) / 2
                                    + new_state.floors[1].count_items()
                                    + new_state.floors[2].count_items() / 2;
                                to_visit.push(Reverse((
                                    new_cost + heuristic as u32,
                                    new_cost,
                                    new_state,
                                )));
                            }
                        }
                    }
                }
            }
        }
    }

    Err("No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example_input = "The first floor contains a promethium generator and a promethium-compatible microchip.
The second floor contains a cobalt generator, a curium generator, a ruthenium generator, and a plutonium generator.
The third floor contains a cobalt-compatible microchip, a curium-compatible microchip, a ruthenium-compatible microchip, and a plutonium-compatible microchip.
The fourth floor contains nothing relevant.";
    test_part_one!(example_input => 33);
    test_part_two!(example_input => 57);

    let real_input = include_str!("day11_input.txt");
    test_part_one!(real_input => 37);
    test_part_two!(real_input => 61);
}
