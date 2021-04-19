use crate::Input;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

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
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct State {
    current_floor: i8,
    floors: [Floor; 4],
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

                    let new_cost = visited_state_cost + 1;
                    let new_state = State {
                        current_floor: new_floor,
                        floors: new_floors,
                    };

                    let do_insert = visited_states.insert(new_state.clone());
                    if do_insert {
                        // Encourage moving things up:
                        let heuristic = new_state.floors[0].content.len() * 60
                            + new_state.floors[1].content.len() * 40
                            + new_state.floors[2].content.len() * 20;
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

    let real_input = include_str!("day11_input.txt");
    test_part_one!(real_input => 37);
    test_part_two!(real_input => 61);
}
