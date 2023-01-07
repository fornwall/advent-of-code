use crate::input::Input;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(input: &Input) -> Result<u32, String> {
    fn checksum(map: &HashMap<&str, Vec<&str>>, name: &str, depth: u32) -> u32 {
        depth
            + map.get(name).map_or(0, |list| {
                list.iter()
                    .map(|entry| checksum(map, entry, depth + 1))
                    .sum::<u32>()
            })
    }

    if input.is_part_one() {
        let mut map = HashMap::new();

        for line in input.text.lines() {
            let mut parts = line.split(')');
            let part = parts.next().ok_or("Invalid input")?;
            let orbited_by = map.entry(part).or_insert_with(Vec::new);
            let part = parts.next().ok_or("Invalid input")?;
            orbited_by.push(part);
        }

        Ok(checksum(&map, "COM", 0))
    } else {
        part2(input.text)
    }
}

fn part2(string: &str) -> Result<u32, String> {
    let mut map = HashMap::new();
    let mut target: &str = "";

    for line in string.lines() {
        let mut parts = line.split(')');
        let orbited_name = parts.next().ok_or("Invalid input")?;
        let orbits_name = parts.next().ok_or("Invalid input")?;

        let orbits = map.entry(orbits_name).or_insert_with(Vec::new);
        orbits.push(orbited_name);

        let orbited = map.entry(orbited_name).or_insert_with(Vec::new);
        orbited.push(orbits_name);

        if orbits_name == "SAN" {
            target = orbited_name;
        }
    }

    let mut visited: HashSet<&str> = HashSet::new();
    let mut to_visit = VecDeque::new();

    visited.insert("YOU");
    to_visit.push_back((0_u32, "YOU"));

    while let Some((distance, name)) = to_visit.pop_front() {
        if let Some(list) = map.get(name) {
            for entry in list.iter() {
                if visited.insert(entry) {
                    if *entry == target {
                        return Ok(distance);
                    }
                    let new_distance = distance + 1;
                    to_visit.push_back((new_distance, entry));
                }
            }
        }
    }

    Err("Unable to find path".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L" => 42);

    test_part_two!("COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN" => 4);

    let input = include_str!("day06_input.txt");
    test_part_one!(input => 273_985);
    test_part_two!(input => 460);
}
