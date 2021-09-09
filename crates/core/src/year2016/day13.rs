use crate::Input;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

const fn is_wall(x: i32, y: i32, magic_number: i32) -> bool {
    let sum = x * x + 3 * x + 2 * x * y + y + y * y + magic_number;
    sum.count_ones() % 2 == 1
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    const TARGET: (i32, i32) = (31, 39);

    let magic_number = input
        .text
        .parse::<i32>()
        .map_err(|e| format!("Invalid magic number: {}", e))?;

    let mut to_visit = BinaryHeap::new();
    let mut visited_states = HashSet::new();

    let initial_state = (1, 1);
    let mut visit_count = 0;

    to_visit.push(Reverse((0, 0, initial_state)));
    visited_states.insert(initial_state);

    while let Some(Reverse((_estimate, visited_state_cost, visited_state))) = to_visit.pop() {
        if input.is_part_one() {
            if visited_state == TARGET {
                return Ok(visited_state_cost);
            }
        } else if visited_state_cost > 50 {
            return Ok(visit_count);
        } else {
            visit_count += 1;
        }

        for (diffx, diffy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_x = visited_state.0 + diffx;
            let new_y = visited_state.1 + diffy;
            if new_x < 0 || new_y < 0 || is_wall(new_x, new_y, magic_number) {
                continue;
            }

            let new_cost = visited_state_cost + 1;

            let do_insert = visited_states.insert((new_x, new_y));
            if do_insert {
                let new_estimate = new_cost
                    + if input.is_part_one() {
                        (new_x - TARGET.0).abs() as u32 + (new_y - TARGET.1).abs() as u32
                    } else {
                        0
                    };
                to_visit.push(Reverse((new_estimate, new_cost, (new_x, new_y))));
            }
        }
    }

    Err("No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day13_input.txt");
    test_part_one!(real_input => 82);
    test_part_two!(real_input => 138);
}
