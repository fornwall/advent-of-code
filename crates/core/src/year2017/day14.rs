use super::day10::solve as knot_hash;
use super::disjoint_set::DisjointSet;
#[cfg(feature = "visualization")]
use crate::input::Visualization;
use crate::input::{Input, Part};
#[cfg(feature = "visualization")]
use std::cell::RefCell;
use std::collections::BTreeMap;

pub fn solve(input: &Input) -> Result<u32, String> {
    // Mapping from (x,y) coordinate of a used square to an identifier
    // constructed from a zero-based sequence to be used as set identifiers
    // in a disjoint set for part 2.
    let mut location_to_set_identifier = BTreeMap::new();
    let mut used_counter = 0;

    if input.text.len() != 8 {
        return Err("Invalid input - should contain 8 characters".to_string());
    }

    for row in 0..=127 {
        let hash_input = format!("{}-{}", input.text, row);
        let hash = knot_hash(&Input {
            text: &hash_input,
            part: Part::Two,
            #[cfg(feature = "visualization")]
            visualization: RefCell::new(Visualization::Svg("".to_string())),
        })?;
        for (index, digit) in hash.bytes().enumerate() {
            let byte = digit - if digit < b'a' { b'0' } else { b'a' - 10 };
            for bit in 0..4 {
                let bit_is_set = byte & (0b1000 >> bit) != 0;
                if bit_is_set {
                    let col = (index * 4 + bit) as i32;
                    location_to_set_identifier.insert((col, row), used_counter);
                    used_counter += 1;
                }
            }
        }
    }

    Ok(if input.is_part_one() {
        used_counter as u32
    } else {
        let mut disjoint_set = DisjointSet::new(used_counter);
        for ((x, y), &this_set) in location_to_set_identifier.iter() {
            // Since coordinates are stored in an ordered set we only need to consider
            // neighbors to the right and below:
            for (dx, dy) in [(1, 0), (0, 1)] {
                let next = (x + dx, y + dy);
                if let Some(&other_set) = location_to_set_identifier.get(&next) {
                    disjoint_set.join(this_set, other_set);
                }
            }
        }
        disjoint_set.num_groups() as u32
    })
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("flqrgnkx" => 8108);
    test_part_two!("flqrgnkx" => 1242);

    let real_input = include_str!("day14_input.txt");
    test_part_one!(real_input => 8222);
    test_part_two!(real_input => 1086);
}
