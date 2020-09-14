use super::int_code::Program;
use std::collections::{HashSet, VecDeque};

const DIRECTIONS: &[(i32, i32); 4] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];

/// The intcode instruction for moving the robot in the specified direction.
fn instruction_for_direction(direction: (i32, i32)) -> i64 {
    match direction {
        (0, 1) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        (1, 0) => 4,
        _ => panic!("Invalid direction ({},{})", direction.0, direction.1),
    }
}

/// Search the space ship using the given intcode program.
/// The on_visit is called with ((pos_x, pos_y), is_oxygen, distance).
fn search_space_ship<F>(input_string: &str, mut on_visit: F)
where
    F: FnMut((i32, i32), bool, i32),
{
    let initial_program = Program::parse(input_string);
    let initial_position = (0, 0);

    // Contains (pos_x, pos_y):
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    // Contains ((pos_x, pos_y), distance, program):
    let mut to_visit = VecDeque::new();

    visited.insert(initial_position);
    to_visit.push_back(((initial_position), 0, initial_program));
    on_visit(initial_position, false, 0);

    while let Some((position, distance, program)) = to_visit.pop_front() {
        for &direction in DIRECTIONS.iter() {
            let new_position = (position.0 + direction.0, position.1 + direction.1);
            if !visited.insert(new_position) {
                continue;
            }
            let new_distance = distance + 1;

            let mut updated_program = program.clone();
            updated_program.input(instruction_for_direction(direction));

            match updated_program.run_for_output()[0] {
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
                other => panic!("Invalid output: {}", other),
            }
        }
    }
}

pub fn part1(input_string: &str) -> Result<i32, String> {
    let mut distance_to_oxygen = -1;
    search_space_ship(input_string, |_, is_oxygen, distance| {
        if is_oxygen {
            distance_to_oxygen = distance;
        }
    });
    Ok(distance_to_oxygen)
}

pub fn part2(input_string: &str) -> Result<i32, String> {
    // Contains (pos_x, pos_y).
    let mut locations_without_oxygen = HashSet::new();
    // Contains ((pos_x, pos_y), distance_from_oxygen).
    let mut to_visit = VecDeque::new();

    search_space_ship(input_string, |position, is_oxygen, _| {
        if is_oxygen {
            to_visit.push_back((position, 0));
        } else {
            locations_without_oxygen.insert(position);
        }
    });

    let mut furthest_distance = -1;
    while let Some((position, distance)) = to_visit.pop_front() {
        for &direction in DIRECTIONS.iter() {
            let new_position = (position.0 + direction.0, position.1 + direction.1);
            if locations_without_oxygen.remove(&new_position) {
                furthest_distance = distance + 1;
                to_visit.push_back((new_position, furthest_distance));
            }
        }
    }

    Ok(furthest_distance)
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day15_input.txt")), Ok(208));
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day15_input.txt")), Ok(306));
}
