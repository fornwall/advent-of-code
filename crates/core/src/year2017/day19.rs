use crate::input::Input;
use std::collections::HashMap;

pub fn solve(input: &Input) -> Result<String, String> {
    let mut map = HashMap::new();

    let mut entrance_at_top_x = None;
    for (line_idx, line) in input.text.lines().enumerate() {
        for (char_idx, char) in line.chars().enumerate() {
            if line_idx == 0 && char == '|' {
                entrance_at_top_x = Some(char_idx);
            }
            map.insert((char_idx as i16, line_idx as i16), char as u8);
        }
    }

    let mut current_position = match entrance_at_top_x {
        None => {
            return Err("No | at top row".to_string());
        }
        Some(x) => (x as i16, 0_i16),
    };

    let mut direction = (0, 1);
    let mut seen_letters = String::new();
    let mut packet_steps = 0;

    'outer_loop: loop {
        packet_steps += 1;
        if packet_steps >= 100_000 {
            return Err(format!("Aborting after {} steps", packet_steps));
        }
        current_position = (
            current_position.0 + direction.0,
            current_position.1 + direction.1,
        );

        match map.get(&current_position) {
            Some(b'+') => {
                for new_direction in [(0_i16, 1_i16), (1, 0), (-1, 0), (0, -1)] {
                    if new_direction == (-direction.0, -direction.1) {
                        // Do not go back.
                    } else {
                        let adjacent = (
                            current_position.0 + new_direction.0,
                            current_position.1 + new_direction.1,
                        );
                        if let Some(&c) = map.get(&adjacent) {
                            if c == b'|' || c == b'-' {
                                direction = new_direction;
                                continue 'outer_loop;
                            }
                        }
                    }
                }
            }
            Some(c) if (b'A'..=b'Z').contains(c) => {
                seen_letters.push(*c as char);
            }
            Some(&c) if c == b'-' || c == b'|' => {
                // Ok
            }
            _ => {
                break Ok(input.part_values(seen_letters, packet_steps.to_string()));
            }
        }
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day19_input.txt");
    test_part_one!(real_input => "KGPTMEJVS".to_string());
    test_part_two!(real_input => "16328".to_string());
}
