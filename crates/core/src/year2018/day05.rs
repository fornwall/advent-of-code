use std::cmp;
use std::collections::HashSet;

fn destroys_eachother(a: u8, b: u8) -> bool {
    a != b && (a as char).to_ascii_lowercase() == (b as char).to_ascii_lowercase()
}

pub fn part1(input: &str) -> Result<usize, String> {
    let bytes = input.as_bytes();
    let length = fully_react(bytes);
    Ok(length)
}

fn fully_react(bytes: &[u8]) -> usize {
    if bytes.len() < 2 {
        return bytes.len();
    }

    let mut destroyed = vec![false; bytes.len()];

    let mut start = 0;
    let mut end = 1;

    let end_index = bytes.len() - 1;

    'outer: loop {
        let a = bytes[start];
        let b = bytes[end];

        if destroys_eachother(a, b) {
            destroyed[start] = true;
            destroyed[end] = true;

            while destroyed[start] {
                if start == 0 {
                    break;
                }
                start -= 1;
            }

            while destroyed[end] {
                if end == end_index {
                    break;
                }
                end += 1;
            }

            if destroyed[start] {
                if end == end_index {
                    break 'outer;
                } else {
                    start = end;
                    end += 1;
                }
            }
        } else if end == end_index {
            break 'outer;
        } else {
            start = end;
            end += 1;
            while destroyed[end] {
                if end == end_index {
                    break 'outer;
                } else {
                    end += 1;
                }
            }

            if end == end_index {
                break 'outer;
            }
        }
    }

    destroyed
        .iter()
        .filter(|unit_destroyed| !*unit_destroyed)
        .count()
}

pub fn part2(input: &str) -> Result<usize, String> {
    let bytes = input.as_bytes();

    let mut considered_unit_types_lower = HashSet::new();

    let result = bytes
        .iter()
        .fold(input.len() + 1, |shortest_length, unit_type| {
            let unit_type_lower = unit_type.to_ascii_lowercase();

            if considered_unit_types_lower.insert(unit_type_lower) {
                let filtered: Vec<u8> = bytes
                    .iter()
                    .filter(|b| b.to_ascii_lowercase() != unit_type_lower)
                    .cloned()
                    .collect();

                let length = fully_react(&filtered);
                cmp::min(shortest_length, length)
            } else {
                shortest_length
            }
        });
    Ok(result)
}

#[test]
fn tests_part1() {
    assert_eq!(Ok(0), part1("aA"));
    assert_eq!(Ok(0), part1("abBA"));
    assert_eq!(Ok(4), part1("abAB"));
    assert_eq!(Ok(6), part1("aabAAB"));

    assert_eq!(Ok(11252), part1(include_str!("day05_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(4), part2("dabAcCaCBAcCcaDA"));

    assert_eq!(Ok(6118), part2(include_str!("day05_input.txt")));
}
