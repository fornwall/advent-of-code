use crate::Input;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Eq, PartialOrd, PartialEq, Hash, Ord)]
enum GeneratorOrMicrochip {
    Generator(u8),
    Microchip(u8),
}

#[derive(Clone, Default, Eq, PartialEq, Hash)]
struct Floor {
    content: Vec<GeneratorOrMicrochip>,
}

impl Floor {
    fn with_item_added(&self, to_add: GeneratorOrMicrochip) -> Self {
        let mut copy = self.clone();
        copy.content.push(to_add);
        copy.content.sort();
        copy
    }

    fn with_item_removed(&self, to_remove: GeneratorOrMicrochip) -> Self {
        Self {
            content: self
                .content
                .iter()
                .filter(|&&item| item != to_remove)
                .copied()
                .collect(),
        }
    }

    fn is_valid(&self) -> bool {
        let contains_generator = self
            .content
            .iter()
            .any(|item| matches!(item, GeneratorOrMicrochip::Generator(_)));

        if contains_generator {
            let contains_unshielded_microchip = self.content.iter().any(|&item| {
                if let GeneratorOrMicrochip::Microchip(value) = item {
                    // Check if unshielded:
                    !self
                        .content
                        .iter()
                        .any(|&item| item == GeneratorOrMicrochip::Generator(value))
                } else {
                    false
                }
            });
            if contains_unshielded_microchip {
                return false;
            }
        }

        true
    }
}

#[derive(Clone)]
struct State {
    current_floor: i8,
    floors: [Floor; 4],
}

impl State {
    fn pairs(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for (floor_idx, floor) in self.floors.iter().enumerate() {
            for item in floor.content.iter() {
                if let GeneratorOrMicrochip::Microchip(value) = item {
                    let matching_generator = GeneratorOrMicrochip::Generator(*value);
                    for (match_floor_idx, match_floor) in self.floors.iter().enumerate() {
                        if match_floor.content.contains(&matching_generator) {
                            result.push((floor_idx, match_floor_idx));
                        }
                    }
                }
            }
        }
        result.sort_unstable();
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

impl Eq for State {}

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

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut name_to_id = HashMap::new();
    let mut current_id = 0_u8;
    let mut initial_floors = [
        Floor::default(),
        Floor::default(),
        Floor::default(),
        Floor::default(),
    ];

    for (floor_idx, line) in input.text.lines().enumerate() {
        // "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
        // The second floor contains a hydrogen generator.
        // The third floor contains a lithium generator.
        // The fourth floor contains nothing relevant."
        let words = line.split(' ').collect::<Vec<_>>();
        for (word_idx, &word) in words.iter().enumerate() {
            let (isotope_name, microchip) = if word.starts_with("microchip") {
                let isotope_name = words[word_idx - 1]
                    .strip_suffix("-compatible")
                    .ok_or("Invalid syntax - not $ISOTYPE-compatible before 'microchip'")?;
                (isotope_name, true)
            } else if word.starts_with("generator") {
                let isotope_name = words[word_idx - 1];
                (isotope_name, false)
            } else {
                continue;
            };

            let isotope_id = *name_to_id
                .entry(isotope_name.to_string())
                .or_insert_with(|| {
                    current_id += 1;
                    current_id
                });

            initial_floors[floor_idx].content.push(if microchip {
                GeneratorOrMicrochip::Microchip(isotope_id)
            } else {
                GeneratorOrMicrochip::Generator(isotope_id)
            });
        }
    }

    if input.is_part_two() {
        let elerium_id = current_id + 1;
        let dilithium_id = current_id + 2;
        initial_floors[0]
            .content
            .push(GeneratorOrMicrochip::Microchip(elerium_id));
        initial_floors[0]
            .content
            .push(GeneratorOrMicrochip::Generator(elerium_id));
        initial_floors[0]
            .content
            .push(GeneratorOrMicrochip::Microchip(dilithium_id));
        initial_floors[0]
            .content
            .push(GeneratorOrMicrochip::Generator(dilithium_id));
    }

    for floor in initial_floors.iter_mut() {
        floor.content.sort();
    }

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
            .all(|floor| floor.content.is_empty())
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
                    .all(|floor| floor.content.is_empty())
            {
                // Do not bring anything down if every floor beneath current is empty.
                continue;
            }

            for (moved_idx, &first_moved_thing) in visited_state.floors
                [visited_state.current_floor as usize]
                .content
                .iter()
                .enumerate()
            {
                for &second_moved_thing in
                    visited_state.floors[visited_state.current_floor as usize].content[moved_idx..]
                        .iter()
                {
                    let mut new_floors = visited_state.floors.clone();

                    new_floors[visited_state.current_floor as usize] = new_floors
                        [visited_state.current_floor as usize]
                        .with_item_removed(first_moved_thing);
                    new_floors[new_floor as usize] =
                        new_floors[new_floor as usize].with_item_added(first_moved_thing);

                    if second_moved_thing != first_moved_thing {
                        new_floors[visited_state.current_floor as usize] = new_floors
                            [visited_state.current_floor as usize]
                            .with_item_removed(second_moved_thing);
                        new_floors[new_floor as usize] =
                            new_floors[new_floor as usize].with_item_added(second_moved_thing);
                    }

                    if !new_floors.iter().all(Floor::is_valid) {
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
                        let heuristic = (new_state.floors[0].content.len() * 3) / 2
                            + new_state.floors[1].content.len()
                            + new_state.floors[2].content.len() / 2;
                        to_visit.push(Reverse((new_cost + heuristic as u32, new_cost, new_state)));
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
