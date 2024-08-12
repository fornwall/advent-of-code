use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::input::Input;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Key {
    value: u8,
}

/// Keys represented as a bit mask where bit 0 is set for 'a', bit 1 is set for 'b' and so on.
type KeyBitset = u32;

impl Key {
    const fn new(value: u8) -> Self {
        Self { value }
    }

    const fn bit_mask(self) -> KeyBitset {
        1 << (self.value as usize - 'a' as usize)
    }
}

/// Path between keys (or from starting position to a key).
struct Edge {
    /// The key at the other end.
    target_key: Key,
    /// Required steps to reach the target key.
    steps: usize,
    /// The keys needed to traverse this path.
    needed_keys: KeyBitset,
}

pub fn steps_to_gather_all_keys(input_string: &str) -> Result<usize, String> {
    let rows = input_string.lines().count();
    let cols = input_string.lines().next().ok_or("Empty input")?.len();
    let mut map = vec![b'#'; rows * cols];
    let mut found_keys = HashMap::new();
    let mut all_keys_bitset = 0_u32;

    let index_of = |x, y| x + y * cols;

    for (y, line) in input_string.lines().enumerate() {
        if line.len() != cols {
            return Err("Not all rows have same width".to_string());
        }
        line.chars().enumerate().for_each(|(x, c)| {
            let byte = c as u8;
            let current_position = (x as i32, y as i32);

            let char_to_insert = match c {
                '@' => {
                    // The single entrance.
                    found_keys.insert(Key::new(b'@'), current_position);
                    b'.'
                }
                'a'..='z' => {
                    // A key.
                    let found_key = Key::new(byte);
                    all_keys_bitset |= found_key.bit_mask();
                    found_keys.insert(found_key, current_position);
                    byte
                }
                '#' => {
                    // Stone wall.
                    return;
                }
                _ => byte,
            };
            map[index_of(x, y)] = char_to_insert;
        });
    }

    if !found_keys.contains_key(&Key::new(b'@')) {
        return Err("No entrance ('@') found".to_string());
    }

    // Mapping to (other_key, needed_keys_to_reach, steps):
    let mut adjacency_list: HashMap<Key, Vec<Edge>> = HashMap::new();

    for (&this_key, &this_key_position) in found_keys.iter() {
        // Find path from this key to all other keys.

        // (position, bitset_of_needed_keys, steps):
        let mut to_visit = VecDeque::new();
        to_visit.push_back((this_key_position, 0_u32, 0_u32));

        let mut visited_positions = HashSet::new();
        visited_positions.insert(this_key_position);

        while let Some((position, needed_keys, steps)) = to_visit.pop_front() {
            'key_direction_loop: for direction in DIRECTIONS {
                let new_position = (position.0 + direction.0, position.1 + direction.1);
                if new_position.0 < 0 || new_position.1 < 0 {
                    continue 'key_direction_loop;
                }
                let mut new_needed_keys = needed_keys;

                match map.get(index_of(new_position.0 as usize, new_position.1 as usize)) {
                    Some(&char_at_position @ b'A'..=b'Z') => {
                        let needed_key = Key::new(char_at_position.to_ascii_lowercase());
                        if found_keys.contains_key(&needed_key) {
                            // Only consider door as necessary if key is in quadrant.
                            // Needed by part 2, where we can wait until key is picked
                            // up in other quadrant.
                            new_needed_keys |= needed_key.bit_mask();
                        }
                    }
                    Some(&char_at_position @ b'a'..=b'z') => {
                        let target_key = Key::new(char_at_position);
                        adjacency_list.entry(this_key).or_default().push(Edge {
                            steps: (steps + 1) as usize,
                            needed_keys: new_needed_keys,
                            target_key,
                        });
                    }
                    Some(b'.') => {
                        // Free to enter.
                    }
                    _ => {
                        continue 'key_direction_loop;
                    }
                }

                if visited_positions.insert(new_position) {
                    let new_state = (new_position, new_needed_keys, steps + 1);
                    to_visit.push_back(new_state);
                }
            }
        }
    }

    shortest_path(&adjacency_list, all_keys_bitset)
        .ok_or_else(|| "Not possible to gather all keys".to_string())
}

fn shortest_path(adjacency_list: &HashMap<Key, Vec<Edge>>, all_keys: KeyBitset) -> Option<usize> {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct Vertex {
        at_key: Key,
        steps: usize,
        gathered_keys: KeyBitset,
    }

    impl Ord for Vertex {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .steps
                .cmp(&self.steps)
                .then_with(|| self.gathered_keys.cmp(&other.gathered_keys))
                .then_with(|| self.at_key.cmp(&other.at_key))
        }
    }

    impl PartialOrd for Vertex {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    // From (key, gathered_keys) to total steps required to reach there.
    let mut cost_for_keys: HashMap<(Key, KeyBitset), usize> = HashMap::new();
    let mut to_visit = BinaryHeap::new();

    to_visit.push(Vertex {
        at_key: Key::new(b'@'),
        steps: 0,
        gathered_keys: 0,
    });

    while let Some(current) = to_visit.pop() {
        if current.gathered_keys == all_keys {
            return Some(current.steps);
        }

        for edge in adjacency_list.get(&current.at_key)? {
            let all_needed_keys_gathered =
                edge.needed_keys & current.gathered_keys == edge.needed_keys;
            if !all_needed_keys_gathered {
                continue;
            }

            let next = Vertex {
                steps: current.steps + edge.steps,
                at_key: edge.target_key,
                gathered_keys: current.gathered_keys | edge.target_key.bit_mask(),
            };

            let current_cost = cost_for_keys
                .entry((edge.target_key, next.gathered_keys))
                .or_insert(usize::MAX);

            if next.steps < *current_cost {
                to_visit.push(next);
                *current_cost = next.steps;
            }
        }
    }

    None
}

pub fn solve(input: &Input) -> Result<usize, String> {
    if input.is_part_one() {
        return steps_to_gather_all_keys(input.text);
    }

    let mut map_top_left = String::new();
    let mut map_top_right = String::new();
    let mut map_bottom_left = String::new();
    let mut map_bottom_right = String::new();

    let num_rows = input.text.lines().count();
    let num_columns = input
        .text
        .lines()
        .next()
        .ok_or("Invalid input - empty first line")?
        .len();
    let center_y = num_rows / 2;
    let center_x = num_columns / 2;

    input.text.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let replaced_char = match (center_x as i32 - x as i32, center_y as i32 - y as i32) {
                (-1..=1, 0) | (0, 1 | -1) => '#',
                (1 | -1, 1 | -1) => '@',
                _ => c,
            };

            if y <= center_y {
                if x <= center_x {
                    &mut map_top_left
                } else {
                    &mut map_top_right
                }
            } else if x <= center_x {
                &mut map_bottom_left
            } else {
                &mut map_bottom_right
            }
            .push(replaced_char);
        });
        if y <= center_y {
            map_top_left.push('\n');
            map_top_right.push('\n');
        } else {
            map_bottom_left.push('\n');
            map_bottom_right.push('\n');
        }
    });

    if !(map_top_left.starts_with('#')) {
        return Err("Invalid input (not surrounded by '#')".to_string());
    }

    let s1 = steps_to_gather_all_keys(&map_top_left)?;
    let s2 = steps_to_gather_all_keys(&map_top_right)?;
    let s3 = steps_to_gather_all_keys(&map_bottom_left)?;
    let s4 = steps_to_gather_all_keys(&map_bottom_right)?;
    Ok(s1 + s2 + s3 + s4)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("#########
#b.A.@.a#
#########"
        => 8);

    test_part_one!("########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
            => 86);

    let input = include_str!("day18_input.txt");
    test_part_one!(input => 4248);
    test_part_two!(input => 1878);
}
