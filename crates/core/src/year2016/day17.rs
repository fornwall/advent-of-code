use crate::Input;
use md5::digest::generic_array::arr;
use md5::digest::FixedOutput;
use md5::{Digest, Md5};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Hash, Clone, Eq, Ord, PartialOrd, PartialEq)]
struct State {
    position: (i32, i32),
    path_so_far: Vec<u8>,
    doors: [bool; 4],
}

fn check_doors(passcode: &[u8], path_so_far: &[u8]) -> [bool; 4] {
    let mut hasher = Md5::new();
    let mut output = arr![u8; 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 , 0, 0];
    hasher.update(passcode);
    hasher.update(path_so_far);
    hasher.finalize_into_reset(&mut output);
    //"Only the first four characters of the hash are used; they represent, respectively,
    // the doors up, down, left, and right from your current position.
    // Any b, c, d, e, or f means that the corresponding door is open; any other character
    // (any number or a) means that the corresponding door is closed and locked."
    let is_open = |byte: u8| (11..=16).contains(&byte);
    [
        is_open((output[0] & 0xF0) >> 4),
        is_open(output[0] & 0x0F),
        is_open((output[1] & 0xF0) >> 4),
        is_open(output[1] & 0x0F),
    ]
}

pub fn solve(input: &mut Input) -> Result<String, String> {
    let passcode = input.text.as_bytes();

    let mut to_visit = BinaryHeap::new();
    let mut cost_of_states = HashMap::new();

    let initial_state = State {
        position: (0, 0),
        path_so_far: Vec::new(),
        doors: check_doors(passcode, &[]),
    };

    to_visit.push(Reverse((0, initial_state.clone())));
    cost_of_states.insert(initial_state, 0);

    let mut desired_path_length = None;

    while let Some(Reverse((visited_state_cost, visited_state))) = to_visit.pop() {
        if visited_state.position == (3, 3) {
            if input.is_part_one() {
                return Ok(visited_state
                    .path_so_far
                    .iter()
                    .map(|&byte| byte as char)
                    .collect::<String>());
            } else {
                desired_path_length = Some(visited_state_cost);
                continue;
            }
        }

        for (idx, direction) in [(0, -1), (0, 1), (-1, 0), (1, 0)].iter().enumerate() {
            if visited_state.doors[idx] {
                let new_position = (
                    visited_state.position.0 + direction.0,
                    visited_state.position.1 + direction.1,
                );

                if new_position.0 < 0
                    || new_position.0 > 3
                    || new_position.1 < 0
                    || new_position.1 > 3
                {
                    continue;
                }

                let direction_char = [b'U', b'D', b'L', b'R'];
                let mut new_path = visited_state.path_so_far.clone();
                new_path.push(direction_char[idx]);
                let doors = check_doors(passcode, &new_path);

                let new_state = State {
                    position: new_position,
                    path_so_far: new_path,
                    doors,
                };

                let new_cost = visited_state_cost + 1;
                to_visit.push(Reverse((new_cost, new_state)));
            }
        }
    }

    desired_path_length
        .map(|length| length.to_string())
        .ok_or_else(|| "No path found".to_string())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day17_input.txt");
    test_part_one!(real_input => "RDRDUDLRDR".to_string());
    test_part_two!(real_input => "386".to_string());
}
