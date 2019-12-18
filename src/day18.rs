use std::collections::{HashMap, HashSet, VecDeque};

const DIRECTIONS: &[(i32, i32); 4] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];

pub fn part1(input_string: &str) -> String {
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    // (position_x, position_y, bitset_of_keys):
    let mut position: (i32, i32, u32) = (0, 0, 0);
    let mut highest_key = 'a';

    input_string.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let char_to_insert = match c {
                '@' => {
                    position = (x as i32, y as i32, 0);
                    '.'
                }
                'a'..='z' => {
                    highest_key = std::cmp::max(highest_key, c);
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
    for c in b'a'..=(highest_key as u8) {
        all_keys_bitset |= 1 << (c as usize - 'a' as usize);
    }
    println!(
        "highest key={}, all_keys_bitset={:b}",
        highest_key, all_keys_bitset
    );

    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::new();
    to_visit.push_back((position, 0));
    visited.insert(position);

    while let Some(((position_x, position_y, taken_keys_bitset), steps)) = to_visit.pop_front() {
        //println!("Visiting {}, {}, bitset={:b}, steps={}", position_x, position_y, taken_keys_bitset, steps);
        'direction_loop: for &direction in DIRECTIONS.iter() {
            let mut new_taken_keys_bitset = taken_keys_bitset;
            let new_position = (position_x + direction.0, position_y + direction.1);
            match map.get(&new_position) {
                Some(&char_at_position) if char_at_position >= 'A' && char_at_position <= 'Z' => {
                    // Check if door open (=key taken).
                    let bit_value = 1 << (char_at_position as u8 - b'A');
                    if bit_value & taken_keys_bitset == bit_value {
                        // Has key.
                        //println!("Yes - can enter door {} with bitset {:b}, steps={}", char_at_position, taken_keys_bitset, steps + 1);
                    } else {
                        //println!("NO - cannot enter door {} with bitset {:b}", char_at_position, taken_keys_bitset);
                        continue 'direction_loop;
                    }
                }
                Some(&char_at_position) if char_at_position >= 'a' && char_at_position <= 'z' => {
                    // Add key.
                    let bit_value = 1 << (char_at_position as u8 - b'a');
                    new_taken_keys_bitset |= bit_value;
                    if new_taken_keys_bitset == all_keys_bitset {
                        return (steps + 1).to_string();
                    }
                    //println!("After key={}, bitset={:b}, steps={}", char_at_position, taken_keys_bitset, steps+1);
                }
                Some('.') => {
                    // Free to enter.
                }
                Some(c) => {
                    panic!("Invalid map entry: {}", c);
                }
                None => {
                    continue 'direction_loop;
                }
            }

            let new_state = (new_position.0, new_position.1, new_taken_keys_bitset);
            if visited.insert(new_state) {
                to_visit.push_back((new_state, steps + 1));
            }
        }
    }
    String::from("")
}

pub fn part2(_input_string: &str) -> String {
    String::from("")
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
    assert_eq!(part2(""), "");

    // assert_eq!(part2(include_str!("day18_input.txt")), "");
}
