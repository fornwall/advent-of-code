use std::collections::HashMap;

fn parse(string: &str) -> u32 {
    let mut map = HashMap::new();

    for line in string.lines() {
        let mut parts = line.split(')');
        let orbited_by = map.entry(parts.next().unwrap()).or_insert_with(Vec::new);
        orbited_by.push(parts.next().unwrap());
    }

    fn checksum(map: &HashMap<&str, Vec<&str>>, name: &str, depth: u32) -> u32 {
        let mut result = depth as u32;
        if let Some(list) = map.get(name) {
            for entry in list.iter() {
                result += checksum(map, entry, depth + 1);
            }
        }
        result
    }

    checksum(&map, "COM", 0)
}

pub fn part1(input_string: &str) -> String {
    parse(input_string).to_string()
}

pub fn part2(_input_string: &str) -> String {
    String::from("")
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
        "42"
    );

    assert_eq!(part1(include_str!("day06_input.txt")), "273985");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(""), "");

    // assert_eq!(part2(include_str!("day06_input.txt")), "");
}
