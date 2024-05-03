use crate::input::Input;
use std::cmp::max;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn visit_rooms<F>(input_string: &str, mut callback: F) -> Result<(), String>
where
    F: FnMut(i32),
{
    if input_string.len() == 1 {
        return Err("Invalid one character input".to_string());
    }
    let input_string = &input_string[1..input_string.len() - 1];

    let apply_direction = |direction, position: &mut (i32, i32)| match direction {
        'N' => {
            position.1 -= 1;
        }
        'E' => {
            position.0 += 1;
        }
        'S' => {
            position.1 += 1;
        }
        'W' => {
            position.0 -= 1;
        }
        _ => {}
    };

    let mut room_doors = HashMap::new();

    let mut current_positions = HashSet::new();
    let mut positions_at_start_of_branch = Vec::new();
    let mut new_possibilities = Vec::new();

    current_positions.insert((0, 0));
    for char in input_string.chars() {
        match char {
            'N' | 'E' | 'S' | 'W' => {
                let old_positions = current_positions;
                current_positions = HashSet::new();
                for possibility in old_positions {
                    let mut possibility = possibility;
                    room_doors
                        .entry(possibility)
                        .or_insert_with(Vec::new)
                        .push(char);
                    apply_direction(char, &mut possibility);
                    let reverse_direction = match char {
                        'N' => 'S',
                        'E' => 'W',
                        'S' => 'N',
                        'W' => 'E',
                        _ => '?',
                    };
                    room_doors
                        .entry(possibility)
                        .or_insert_with(Vec::new)
                        .push(reverse_direction);
                    current_positions.insert(possibility);
                }
            }
            '(' => {
                positions_at_start_of_branch.push(current_positions.clone());
                new_possibilities.push(Vec::new());
            }
            '|' => {
                new_possibilities
                    .last_mut()
                    .ok_or("No possibility to push to for |")?
                    .push(current_positions.clone());
                current_positions.clone_from(
                    positions_at_start_of_branch
                        .last_mut()
                        .ok_or("No position at start of branch for |")?,
                );
            }
            ')' => {
                new_possibilities
                    .last_mut()
                    .ok_or("No possibility to push to for )")?
                    .push(current_positions);
                current_positions = HashSet::new();
                let nn: Vec<HashSet<(i32, i32)>> =
                    new_possibilities.pop().ok_or("No new possibility for )")?;
                for n in nn {
                    for e in n {
                        current_positions.insert(e);
                    }
                }

                positions_at_start_of_branch.pop();
            }
            _ => {
                return Err(format!("Invalid map tile: {char}"));
            }
        }
    }

    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((0_i32, 0, 0));

    while let Some(visiting) = to_visit.pop_front() {
        callback(visiting.0);

        if let Entry::Occupied(doors) = room_doors.entry((visiting.1, visiting.2)) {
            for char in doors.get() {
                let mut adjacent_room = (visiting.1, visiting.2);
                apply_direction(*char, &mut adjacent_room);
                if visited.insert(adjacent_room) {
                    to_visit.push_back((visiting.0 + 1, adjacent_room.0, adjacent_room.1));
                }
            }
        };
    }
    Ok(())
}

pub fn solve(input: &Input) -> Result<i32, String> {
    let mut result = 0;
    visit_rooms(input.text, |cost| {
        if input.is_part_one() {
            result = max(result, cost);
        } else if cost >= 1000 {
            result += 1;
        }
    })?;
    Ok(result)
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("^WNE$" => 3);
    test_part_one!("^ENWWW(NEEE|SSE(EE|N))$" => 10);
    test_part_one!("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$" => 18);
    test_part_one!("^(SSS|EEESSSWWW)ENNES$" => 8);
    test_part_one!("^(E|SSEENNW)S$" => 4);
    test_part_one!("^(E|SEN)$" => 2);
    test_part_one!("^NNNNN(EEEEE|NNN)NNNNN$" => 15);

    let input = include_str!("day20_input.txt");
    test_part_one!(input => 3151);
    test_part_two!(input => 8784);
}
