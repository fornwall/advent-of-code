use crate::input::Input;
use std::collections::{HashMap, HashSet};

/// Using double-width coordinates - see https://www.redblobgames.com/grids/hexagons/
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

    if input.is_part_two() {
        for _day in 1..=100 {
            let mut adjacent_blacks_count = HashMap::new();
            let mut new_black_tiles = black_tiles.clone();

            for &black_tile in black_tiles.iter() {
                for diff in [(2, 0), (1, -1), (-1, -1), (-2, 0), (-1, 1), (1, 1)] {
                    let adjacent_location = (black_tile.0 + diff.0, black_tile.1 + diff.1);
                    *adjacent_blacks_count.entry(adjacent_location).or_insert(0) += 1;
                }
            }

            for location in black_tiles.iter() {
                if !adjacent_blacks_count.contains_key(location) {
                    // "Any black tile with zero or more than 2 black tiles immediately
                    // adjacent to it is flipped to white."
                    // We only do the first check here.
                    new_black_tiles.remove(location);
                }
            }

            for (&location, &adjacent_blacks) in adjacent_blacks_count.iter() {
                let is_black = black_tiles.contains(&location);
                if is_black && adjacent_blacks > 2 {
                    // "Any black tile with zero or more than 2 black tiles immediately
                    // adjacent to it is flipped to white."
                    // We only do the second check here.
                    new_black_tiles.remove(&location);
                } else if !is_black && adjacent_blacks == 2 {
                    // "Any white tile with exactly 2 black tiles immediately adjacent
                    // to it is flipped to black."
                    new_black_tiles.insert(location);
                }
            }

            std::mem::swap(&mut black_tiles, &mut new_black_tiles);
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

    let example = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    test_part_one!(example => 10);
    test_part_two!(example => 2208);

    let real_input = include_str!("day24_input.txt");
    test_part_one!(real_input => 549);
    test_part_two!(real_input => 4147);
}
