use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

const DIRECTIONS: &[(i32, i32); 4] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    steps: usize,
    gathered_keys: u32,
    position: u8,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then_with(|| self.gathered_keys.cmp(&other.gathered_keys))
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    steps: usize,
    needed_keys: u32,
    other_key: u8,
}

pub fn part1(input_string: &str) -> String {
    part1_usize(input_string).to_string()
}

pub fn part1_usize(input_string: &str) -> usize {
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut key_positions: HashMap<u8, (i32, i32)> = HashMap::new();
    let mut found_keys = HashSet::new();

    input_string.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let char_to_insert = match c {
                '@' => {
                    key_positions.insert(c as u8, (x as i32, y as i32));
                    '.'
                }
                'a'..='z' => {
                    found_keys.insert(c as u8);
                    key_positions.insert(c as u8, (x as i32, y as i32));
                    c
                }
                '#' => {
                    return;
                }
                _ => c,
            };
            map.insert((x as i32, y as i32), char_to_insert);
        });
    });

    let mut all_keys_bitset = 0 as u32;
    for &c in found_keys.iter() {
        all_keys_bitset |= 1 << (c as usize - 'a' as usize);
    }

    // Mapping to (other_key, needed_keys_to_reach, steps):
    let mut key_edges: HashMap<u8, Vec<Edge>> = HashMap::new();
    found_keys.insert(b'@');
    for &this_key in &found_keys {
        // Find path from this key to all other keys.
        let this_key_position = *key_positions.get(&this_key).unwrap();

        let mut to_visit = VecDeque::new();
        let mut visited = HashSet::new();
        // (position, bitset_of_needed_keys, steps):
        to_visit.push_back((this_key_position, 0 as u32, 0 as u32));
        visited.insert(this_key_position);

        while let Some((position, needed_keys, steps)) = to_visit.pop_front() {
            'key_direction_loop: for direction in DIRECTIONS.iter() {
                let new_position = (position.0 + direction.0, position.1 + direction.1);
                let mut new_needed_keys = needed_keys;
                let mut found_key = None;
                match map.get(&new_position) {
                    Some(&char_at_position @ 'A'..='Z') => {
                        let needed_key = char_at_position.to_ascii_lowercase();
                        if found_keys.contains(&(needed_key as u8)) {
                            // Only consider door as necessary if key is in quadrant.
                            // Needed by part 4, where we can wait until key is picked
                            // up in other quadrant.
                            let bit_value = 1 << (char_at_position as u8 - b'A');
                            new_needed_keys |= bit_value;
                        }
                    }
                    Some(&char_at_position @ 'a'..='z') => {
                        if char_at_position as u8 != this_key {
                            found_key = Some(char_at_position as u8);
                        }
                    }
                    Some('.') | Some('@') => {
                        // Free to enter.
                    }
                    Some('#') | None => {
                        continue 'key_direction_loop;
                    }
                    Some(c) => {
                        panic!("Invalid map entry: {}", c);
                    }
                }

                let new_steps = steps + 1;
                let new_state = (new_position, new_needed_keys, new_steps);
                if visited.insert(new_position) {
                    to_visit.push_back(new_state);

                    if let Some(other_key) = found_key {
                        key_edges
                            .entry(this_key)
                            .or_insert_with(Vec::new)
                            .push(Edge {
                                steps: new_steps as usize,
                                needed_keys: new_needed_keys,
                                other_key,
                            });
                    }
                }
            }
        }
    }

    shortest_path(&key_edges, b'@', all_keys_bitset).unwrap()
}

fn shortest_path(adj_list: &HashMap<u8, Vec<Edge>>, start: u8, all_keys: u32) -> Option<usize> {
    // From (key_at_position, gathered_keys) to steps required to reach here.
    let mut cost_for_state: HashMap<(u8, u32), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    heap.push(State {
        steps: 0,
        position: start,
        gathered_keys: 0,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        steps,
        gathered_keys,
        position,
    }) = heap.pop()
    {
        // Alternatively we could have continued to find all shortest paths
        if gathered_keys == all_keys {
            return Some(steps);
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        if let Some(edges) = adj_list.get(&position) {
            for edge in edges {
                let next = State {
                    steps: steps + edge.steps,
                    position: edge.other_key,
                    gathered_keys: gathered_keys | (1 << ((edge.other_key - b'a') as u32)),
                };

                if let Some(&existing_cost) =
                    cost_for_state.get(&(edge.other_key, next.gathered_keys))
                {
                    if existing_cost <= next.steps {
                        continue;
                    }
                }
                if edge.needed_keys & gathered_keys == edge.needed_keys {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    cost_for_state.insert((edge.other_key, next.gathered_keys), next.steps);
                }
            }
        }
    }

    // Goal not reachable
    None
}

pub fn part2(input_string: &str) -> String {
    let mut map_top_left = String::new();
    let mut map_top_right = String::new();
    let mut map_bottom_left = String::new();
    let mut map_bottom_right = String::new();

    let num_rows = input_string.lines().count();
    let num_columns = input_string.lines().next().unwrap().len();
    let center_y = num_rows / 2;
    let center_x = num_columns / 2;

    input_string.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let replaced_char = match (center_x as i32 - x as i32, center_y as i32 - y as i32) {
                (0, 0) | (1, 0) | (-1, 0) | (0, 1) | (0, -1) => '#',
                (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => '@',
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

    let result = part1_usize(&map_top_left)
        + part1_usize(&map_top_right)
        + part1_usize(&map_bottom_left)
        + part1_usize(&map_bottom_right);
    result.to_string()
}

#[test]
pub fn tests_part1() {
    assert_eq!(
        part1(
            "#########
    #b.A.@.a#
    #########"
        ),
        "8"
    );

    assert_eq!(
        part1(
            "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
        ),
        "86"
    );

    assert_eq!(part1(include_str!("day18_input.txt")), "4248");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day18_input.txt")), "1878");
}
