use super::int_code::{Program, Word};
use crate::Input;
use std::collections::{HashSet, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

/// The intcode instruction for moving the robot in the specified direction.
fn instruction_for_direction(direction: (i32, i32)) -> Result<Word, String> {
    Ok(match direction {
        (0, 1) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        (1, 0) => 4,
        _ => {
            return Err(format!(
                "Invalid direction ({},{})",
                direction.0, direction.1
            ))
        }
    })
}

/// Search the space ship using the given intcode program.
/// The on_visit is called with ((pos_x, pos_y), is_oxygen, distance).
fn search_space_ship<F>(input_string: &str, mut on_visit: F) -> Result<(), String>
where
    F: FnMut((i32, i32), bool, i32),
{
    let initial_program = Program::parse(input_string)?;
    let initial_position = (0, 0);

    // Contains (pos_x, pos_y):
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    // Contains ((pos_x, pos_y), distance, program):
    let mut to_visit = VecDeque::new();

    visited.insert(initial_position);
    to_visit.push_back(((initial_position), 0, initial_program));
    on_visit(initial_position, false, 0);

    while let Some((position, distance, program)) = to_visit.pop_front() {
        for direction in DIRECTIONS {
            let new_position = (position.0 + direction.0, position.1 + direction.1);
            if !visited.insert(new_position) {
                continue;
            }
            let new_distance = distance + 1;

            let mut updated_program = program.clone();
            let instruction_input = instruction_for_direction(direction)?;
            updated_program.input(instruction_input);

            let output = updated_program.run_for_output()?;
            if output.is_empty() {
                return Err("No output produced".to_string());
            }
            match output[0] {
                // 0: The repair droid hit a wall. Its position has not changed.
                0 => {
                    // Do nothing.
                }
                // 1: The repair droid has moved one step in the requested direction.
                // 2: The repair droid has moved one step in the requested direction;
                // its new position is the location of the oxygen system.
                val @ 1..=2 => {
                    on_visit(new_position, val == 2, new_distance);
                    to_visit.push_back((new_position, new_distance, updated_program.clone()));
                }
                other => {
                    return Err(format!("Invalid output: {}", other));
                }
            }
        }
    }

    Ok(())
}

pub fn solve(input: &mut Input) -> Result<i32, String> {
    if input.is_part_one() {
        let mut distance_to_oxygen = -1;
        search_space_ship(input.text, |_, is_oxygen, distance| {
            if is_oxygen {
                distance_to_oxygen = distance;
            }
        })?;
        Ok(distance_to_oxygen)
    } else {
        // Contains (pos_x, pos_y).
        let mut locations_without_oxygen = HashSet::new();
        // Contains ((pos_x, pos_y), distance_from_oxygen).
        let mut to_visit = VecDeque::new();

        search_space_ship(input.text, |position, is_oxygen, _| {
            if is_oxygen {
                to_visit.push_back((position, 0));
            } else {
                locations_without_oxygen.insert(position);
            }
        })?;

        let mut furthest_distance = -1;
        while let Some((position, distance)) = to_visit.pop_front() {
            for direction in DIRECTIONS {
                let new_position = (position.0 + direction.0, position.1 + direction.1);
                if locations_without_oxygen.remove(&new_position) {
                    furthest_distance = distance + 1;
                    to_visit.push_back((new_position, furthest_distance));
                }
            }
        }

        Ok(furthest_distance)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};
    let input = include_str!("day15_input.txt");
    test_part_one!(input => 208);
    test_part_two!(input => 306);
}
