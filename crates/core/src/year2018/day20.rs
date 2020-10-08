use std::cmp::max;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn visit_rooms<F>(input_string: &str, mut callback: F) -> Result<(), String>
where
    F: FnMut(i32),
{
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
                    .push(current_positions);
                current_positions = positions_at_start_of_branch
                    .last_mut()
                    .ok_or("No position at start of branch for |")?
                    .clone();
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
                return Err(format!("Invalid map tile: {}", char));
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

pub fn part1(input_string: &str) -> Result<i32, String> {
    let mut highest_cost = 0;
    visit_rooms(input_string, |cost| {
        highest_cost = max(highest_cost, cost);
    })?;
    Ok(highest_cost)
}

pub fn part2(input_string: &str) -> Result<i32, String> {
    let mut room_count = 0;
    visit_rooms(input_string, |cost| {
        if cost >= 1000 {
            room_count += 1;
        }
    })?;
    Ok(room_count)
}

#[test]
fn tests_part1() {
    assert_eq!(Ok(3), part1("^WNE$"));
    assert_eq!(Ok(10), part1("^ENWWW(NEEE|SSE(EE|N))$"));
    assert_eq!(Ok(18), part1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"));
    assert_eq!(Ok(8), part1("^(SSS|EEESSSWWW)ENNES$"));
    assert_eq!(Ok(4), part1("^(E|SSEENNW)S$"));
    assert_eq!(Ok(2), part1("^(E|SEN)$"));
    assert_eq!(Ok(15), part1("^NNNNN(EEEEE|NNN)NNNNN$"));

    assert_eq!(Ok(3151), part1(include_str!("day20_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(8784), part2(include_str!("day20_input.txt")));
}
