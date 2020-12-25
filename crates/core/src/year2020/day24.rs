use crate::input::Input;
use std::collections::HashSet;

/// Using double-height coordinates - see https://www.redblobgames.com/grids/hexagons/
pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut black_tiles = HashSet::new();

    for line_str in input.text.lines() {
        let mut location = (0_i32, 0_i32);
        let mut string_position = 0;
        let line = line_str.as_bytes();
        while string_position < line.len() {
            let first_char = line[string_position];
            let diff = match first_char {
                b'e' => (2, 0),
                b'w' => (-2, 0),
                b's' | b'n' => {
                    string_position += 1;
                    match (first_char, line.get(string_position)) {
                        (b'n', Some(b'e')) => (1, 1),
                        (b'n', Some(b'w')) => (-1, 1),
                        (b's', Some(b'e')) => (1, -1),
                        (b's', Some(b'w')) => (-1, -1),
                        _ => {
                            return Err("Invalid input".to_string());
                        }
                    }
                }
                _ => {
                    return Err("Invalid input".to_string());
                }
            };

            location = (location.0 + diff.0, location.1 + diff.1);

            string_position += 1;
        }

        if !black_tiles.insert(location) {
            black_tiles.remove(&location);
        }
    }

    Ok(black_tiles.len() as u64)
}
#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!("esew" => 1);
    test_part_one!("esew\nesew" => 0);
    test_part_one!("esew\nnwwswee" => 2);
    //klet example_part_two = "";
    //test_part_two!(example_part_two => 0);

    let real_input = include_str!("day24_input.txt");
    test_part_one!(real_input => 549);
    // test_part_two!(real_input => 0);
}
