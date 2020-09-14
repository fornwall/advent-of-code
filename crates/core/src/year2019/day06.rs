use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part1(string: &str) -> Result<u32, String> {
    let mut map = HashMap::new();

    for line in string.lines() {
        let mut parts = line.split(')');
        let part = parts.next().ok_or(format!("Invalid line: {}", line))?;
        let orbited_by = map.entry(part).or_insert_with(Vec::new);
        let part = parts.next().ok_or(format!("Invalid line: {}", line))?;
        orbited_by.push(part);
    }

    fn checksum(map: &HashMap<&str, Vec<&str>>, name: &str, depth: u32) -> u32 {
        depth
            + map.get(name).map_or(0, |list| {
                list.iter()
                    .map(|entry| checksum(map, entry, depth + 1))
                    .sum::<u32>()
            })
    }

    Ok(checksum(&map, "COM", 0))
}

pub fn part2(string: &str) -> Result<u32, String> {
    let mut map = HashMap::new();
    let mut target: &str = "";

    for line in string.lines() {
        let mut parts = line.split(')');
        let orbited_name = parts.next().ok_or(format!("Invalid line: {}", line))?;
        let orbits_name = parts.next().ok_or(format!("Invalid line: {}", line))?;

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
    to_visit.push_back((0u32, "YOU"));

    while !to_visit.is_empty() {
        let (distance, name) = to_visit.pop_front().unwrap();

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
pub fn tests_part1() {
    assert_eq!(
        part1(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"
        ),
        Ok(42)
    );

    assert_eq!(part1(include_str!("day06_input.txt")), Ok(273985));
}

#[test]
fn tests_part2() {
    assert_eq!(
        part2(
            "COM)B
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
I)SAN"
        ),
        Ok(4)
    );

    assert_eq!(part2(include_str!("day06_input.txt")), Ok(460));
}
