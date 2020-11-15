use super::day10::part2 as knot_hash;
use super::disjoint_set::DisjointSet;
use std::collections::BTreeMap;

pub fn part1(input_string: &str) -> Result<u32, String> {
    let mut used_count = 0;
    for row in 0..=127 {
        let hash_input = format!("{}-{}", input_string, row);
        let hash = knot_hash(&hash_input)?;
        used_count += hash
            .chars()
            .map(|b| {
                u32::from_str_radix(&b.to_string(), 16)
                    .expect("ok")
                    .count_ones()
            })
            .sum::<u32>();
    }
    Ok(used_count)
}

pub fn part2(input_string: &str) -> Result<usize, String> {
    let mut hash_map = BTreeMap::new();

    let mut counter = 0;
    for row in 0..=127 {
        let hash_input = format!("{}-{}", input_string, row);
        let hash = knot_hash(&hash_input)?;
        for (index, byte) in hash
            .chars()
            .map(|b| u32::from_str_radix(&b.to_string(), 16).expect("ok"))
            .enumerate()
        {
            for bit in 0..4 {
                if byte & (0b1000 >> bit) != 0 {
                    let col = (index * 4 + bit) as i32;
                    hash_map.insert((col, row), counter);
                    counter += 1;
                }
            }
        }
    }

    let mut disjoint_set = DisjointSet::new(counter);
    for ((x, y), &value) in hash_map.iter() {
        for (dx, dy) in &[(1, 0), (0, 1)] {
            let next = (x + dx, y + dy);
            if let Some(&other) = hash_map.get(&next) {
                disjoint_set.join(value, other);
            }
        }
    }

    Ok(disjoint_set.num_groups())
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
