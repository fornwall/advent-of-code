use super::day10::part2 as knot_hash;
use super::disjoint_set::DisjointSet;
use std::collections::BTreeMap;

fn solution(input_string: &str, part1: bool) -> Result<u32, String> {
    // Mapping from (x,y) coordinate of a used square to an identifier
    // constructed from a zero-based sequence to be used as set identifiers
    // in a disjoint set for part 2.
    let mut location_to_set_identifier = BTreeMap::new();
    let mut used_counter = 0;

    for row in 0..=127 {
        let hash_input = format!("{}-{}", input_string, row);
        let hash = knot_hash(&hash_input)?;
        for (index, digit) in hash.bytes().enumerate() {
            let byte = digit - if digit < b'a' { b'0' } else { b'a' - 10 };
            for bit in 0..4 {
                if byte & (0b1000 >> bit) != 0 {
                    let col = (index * 4 + bit) as i32;
                    location_to_set_identifier.insert((col, row), used_counter);
                    used_counter += 1;
                }
            }
        }
    }

    Ok(if part1 {
        used_counter as u32
    } else {
        let mut disjoint_set = DisjointSet::new(used_counter);
        for ((x, y), &this_set) in location_to_set_identifier.iter() {
            // Since coordinates are stored in an ordered set we only need to consider
            // neighbors to the right and below:
            for (dx, dy) in &[(1, 0), (0, 1)] {
                let next = (x + dx, y + dy);
                if let Some(&other_set) = location_to_set_identifier.get(&next) {
                    disjoint_set.join(this_set, other_set);
                }
            }
        }
        disjoint_set.num_groups() as u32
    })
}

pub fn part1(input_string: &str) -> Result<u32, String> {
    solution(input_string, true)
}

pub fn part2(input_string: &str) -> Result<u32, String> {
    solution(input_string, false)
}

#[test]
fn test_part1() {
    assert_eq!(Ok(8108), part1("flqrgnkx"));
    assert_eq!(Ok(8222), part1(include_str!("day14_input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(Ok(1242), part2("flqrgnkx"));
    assert_eq!(Ok(1086), part2(include_str!("day14_input.txt")));
}
