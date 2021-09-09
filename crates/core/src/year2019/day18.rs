use crate::input::Input;
#[cfg(feature = "visualization")]
use crate::painter::PainterRef;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

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
    #[cfg(feature = "visualization")]
    x: i32,
    #[cfg(feature = "visualization")]
    y: i32,
}

pub fn steps_to_gather_all_keys(
    input_string: &str,
    #[cfg(feature = "visualization")] mut painter: &mut PainterRef,
    #[cfg(feature = "visualization")] map_x_offset: usize,
    #[cfg(feature = "visualization")] map_y_offset: usize,
    #[cfg(feature = "visualization")] mut global_cols: usize,
    #[cfg(feature = "visualization")] mut global_rows: usize,
) -> Result<usize, String> {
    #[cfg(feature = "visualization")]
    painter.fill_style_rgb(255, 0, 0);

    let rows = input_string.lines().count();
    let cols = input_string.lines().next().ok_or("Empty input")?.len();
    let mut map = vec![b'#'; rows * cols];
    let mut found_keys = HashMap::new();
    let mut all_keys_bitset = 0_u32;

    let index_of = |x, y| x + y * cols;

    #[cfg(feature = "visualization")]
    {
        if global_rows == 0 {
            global_rows = rows;
            global_cols = cols;
        }
    }

    for (y, line) in input_string.lines().enumerate() {
        if line.len() != cols {
            return Err("Not all rows have same width".to_string());
        }
        line.chars().enumerate().for_each(|(x, c)| {
            let byte = c as u8;
            let current_position = (x as i32, y as i32);

            #[cfg(feature = "visualization")]
            let canvas_x = (x + map_x_offset) as f64 / global_cols as f64;
            #[cfg(feature = "visualization")]
            let canvas_y = (y + map_y_offset) as f64 / global_rows as f64;
            #[cfg(feature = "visualization")]
            let draw_width = 0.95 / global_cols as f64;
            #[cfg(feature = "visualization")]
            let draw_height = 0.95 / global_rows as f64;
            #[cfg(feature = "visualization")]
            let draw = |drawer: &mut PainterRef| {
                drawer.fill_rect(canvas_x, canvas_y, draw_width, draw_height);
            };

            let char_to_insert = match c {
                '@' => {
                    // The single entrance.
                    found_keys.insert(Key::new(b'@'), current_position);
                    #[cfg(feature = "visualization")]
                    {
                        painter.fill_style_rgb(0, 0, 255);
                        draw(&mut painter);
                    }
                    b'.'
                }
                'a'..='z' => {
                    // A key.
                    let found_key = Key::new(byte);
                    all_keys_bitset |= found_key.bit_mask();
                    found_keys.insert(found_key, current_position);
                    #[cfg(feature = "visualization")]
                    {
                        painter.fill_style_rgb(0, 255, 0);
                        draw(&mut painter);
                    }
                    byte
                }
                '#' => {
                    #[cfg(feature = "visualization")]
                    {
                        painter.fill_style_rgb(255, 0, 0);
                        draw(&mut painter);
                    }
                    // Stone wall.
                    return;
                }
                _ => {
                    #[cfg(feature = "visualization")]
                    {
                        if ('A'..='Z').contains(&c) {
                            painter.fill_style_rgb(0, 255, 255);
                            draw(&mut painter);
                        }
                    }
                    byte
                }
            };
            map[index_of(x, y)] = char_to_insert;
        });
    }

    #[cfg(feature = "visualization")]
    painter.end_frame();

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
                let mut found_key = None;

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
                        found_key = Some(Key::new(char_at_position));
                    }
                    Some(b'.') => {
                        // Free to enter.
                    }
                    _ => {
                        continue 'key_direction_loop;
                    }
                }

                let new_steps = steps + 1;
                if let Some(target_key) = found_key {
                    adjacency_list
                        .entry(this_key)
                        .or_insert_with(Vec::new)
                        .push(Edge {
                            steps: new_steps as usize,
                            needed_keys: new_needed_keys,
                            target_key,
                            #[cfg(feature = "visualization")]
                            x: new_position.0,
                            #[cfg(feature = "visualization")]
                            y: new_position.1,
                        });
                } else if visited_positions.insert(new_position) {
                    let new_state = (new_position, new_needed_keys, new_steps);
                    to_visit.push_back(new_state);
                }
            }
        }
    }

    shortest_path(
        &adjacency_list,
        all_keys_bitset,
        #[cfg(feature = "visualization")]
        &mut painter,
        #[cfg(feature = "visualization")]
        map_x_offset,
        #[cfg(feature = "visualization")]
        map_y_offset,
        #[cfg(feature = "visualization")]
        global_cols,
        #[cfg(feature = "visualization")]
        global_rows,
    )
    .ok_or_else(|| "Not possible to gather all keys".to_string())
}

fn shortest_path(
    adjacency_list: &HashMap<Key, Vec<Edge>>,
    all_keys: KeyBitset,
    #[cfg(feature = "visualization")] drawer: &mut PainterRef,
    #[cfg(feature = "visualization")] map_x_offset: usize,
    #[cfg(feature = "visualization")] map_y_offset: usize,
    #[cfg(feature = "visualization")] global_cols: usize,
    #[cfg(feature = "visualization")] global_rows: usize,
) -> Option<usize> {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct Vertex {
        at_key: Key,
        steps: usize,
        gathered_keys: KeyBitset,
        #[cfg(feature = "visualization")]
        x: i32,
        #[cfg(feature = "visualization")]
        y: i32,
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

    #[cfg(feature = "visualization")]
    let mut visited_locations = HashSet::new();

    to_visit.push(Vertex {
        at_key: Key::new(b'@'),
        steps: 0,
        gathered_keys: 0,
        #[cfg(feature = "visualization")]
        x: 0,
        #[cfg(feature = "visualization")]
        y: 0,
    });

    while let Some(current) = to_visit.pop() {
        if current.gathered_keys == all_keys {
            return Some(current.steps);
        }

        #[cfg(feature = "visualization")]
        {
            if visited_locations.insert((current.x, current.y)) && current.at_key.value != b'@' {
                let canvas_x = (current.x + map_x_offset as i32) as f64 / global_cols as f64;
                let canvas_y = (current.y + map_y_offset as i32) as f64 / global_rows as f64;
                let draw_width = 0.95 / global_cols as f64;
                let draw_height = 0.95 / global_rows as f64;
                drawer.fill_style_rgb(80, 0, 80);
                drawer.fill_rect(canvas_x, canvas_y, draw_width, draw_height);
                drawer.meta_delay(50);
            }
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
                #[cfg(feature = "visualization")]
                x: edge.x,
                #[cfg(feature = "visualization")]
                y: edge.y,
            };

            let current_cost = cost_for_keys
                .entry((edge.target_key, next.gathered_keys))
                .or_insert(usize::max_value());

            if next.steps < *current_cost {
                to_visit.push(next);
                *current_cost = next.steps;
            }
        }
    }

    None
}

pub fn solve(input: &mut Input) -> Result<usize, String> {
    if input.is_part_one() {
        return steps_to_gather_all_keys(
            input.text,
            #[cfg(feature = "visualization")]
            &mut input.painter,
            #[cfg(feature = "visualization")]
            0,
            #[cfg(feature = "visualization")]
            0,
            #[cfg(feature = "visualization")]
            0,
            #[cfg(feature = "visualization")]
            0,
        );
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

    if !(map_top_left.starts_with('#')) {
        return Err("Invalid input (not surrounded by '#')".to_string());
    }

    let s1 = steps_to_gather_all_keys(
        &map_top_left,
        #[cfg(feature = "visualization")]
        &mut input.painter,
        #[cfg(feature = "visualization")]
        0,
        #[cfg(feature = "visualization")]
        0,
        #[cfg(feature = "visualization")]
        num_columns,
        #[cfg(feature = "visualization")]
        num_rows,
    )?;
    let s2 = steps_to_gather_all_keys(
        &map_top_right,
        #[cfg(feature = "visualization")]
        &mut input.painter,
        #[cfg(feature = "visualization")]
        {
            center_x + 1
        },
        #[cfg(feature = "visualization")]
        0,
        #[cfg(feature = "visualization")]
        num_columns,
        #[cfg(feature = "visualization")]
        num_rows,
    )?;
    let s3 = steps_to_gather_all_keys(
        &map_bottom_left,
        #[cfg(feature = "visualization")]
        &mut input.painter,
        #[cfg(feature = "visualization")]
        0,
        #[cfg(feature = "visualization")]
        {
            center_y + 1
        },
        #[cfg(feature = "visualization")]
        num_columns,
        #[cfg(feature = "visualization")]
        num_rows,
    )?;
    let s4 = steps_to_gather_all_keys(
        &map_bottom_right,
        #[cfg(feature = "visualization")]
        &mut input.painter,
        #[cfg(feature = "visualization")]
        {
            center_x + 1
        },
        #[cfg(feature = "visualization")]
        {
            center_y + 1
        },
        #[cfg(feature = "visualization")]
        num_columns,
        #[cfg(feature = "visualization")]
        num_rows,
    )?;
    Ok(s1 + s2 + s3 + s4)
}

#[test]
pub fn tests_part1() {
    assert_eq!(
        solve(&mut Input::part_one(
            "#########
#b.A.@.a#
#########"
        )),
        Ok(8)
    );

    assert_eq!(
        solve(&mut Input::part_one(
            "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
        )),
        Ok(86)
    );

    assert_eq!(
        solve(&mut Input::part_one(include_str!("day18_input.txt"))),
        Ok(4248)
    );
}

#[test]
fn tests_part2() {
    assert_eq!(
        solve(&mut Input::part_two(include_str!("day18_input.txt"))),
        Ok(1878)
    );
}
