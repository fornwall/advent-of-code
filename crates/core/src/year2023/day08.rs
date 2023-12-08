use crate::common::id_assigner::IdAssigner;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut id_assigner = IdAssigner::<1000, [u8]>::new();

    let (instructions, map_lines) = input.text.split_once("\n\n").unwrap();

    let _ = id_assigner.id_of(&[b'A', b'A', b'A'])?;
    let _ = id_assigner.id_of(&[b'Z', b'Z', b'Z'])?;
    let mut map = vec![(0, 0), (0, 0)];
    let mut starting_nodes = Vec::new();
    let mut end_nodes = Vec::new();

    for line in map_lines.lines() {
        let mut start_idx = usize::MAX;
        let mut str_count = 0;
        let mut ids = [0_u16; 3];
        let bytes = line.as_bytes();
        for (idx, c) in bytes.iter().enumerate() {
            if c.is_ascii_alphanumeric() {
                if start_idx == usize::MAX {
                    start_idx = idx;
                }
            } else if start_idx != usize::MAX {
                if str_count == 3 {
                    return Err("Invalid input".to_string());
                }
                ids[str_count] = id_assigner.id_of(&bytes[start_idx..idx])?;
                if str_count == 0 && bytes[2] == b'A' {
                    starting_nodes.push(ids[str_count]);
                } else if str_count == 0 && bytes[2] == b'Z' {
                    end_nodes.push(ids[str_count]);
                }
                str_count += 1;
                start_idx = usize::MAX;
            }
        }
        if str_count != 3 {
            return Err("Invalid input".to_string());
        }
        let from_id = ids[0] as usize;
        let to = (ids[1], ids[2]);
        if from_id < map.len() {
            map[from_id] = to
        } else {
            map.resize(from_id + 1, (0, 0));
            map[from_id] = to;
        }
    }

    let starting_nodes = if input.is_part_one() {
        vec![0]
    } else {
        starting_nodes
    };

    let mut result = 1;
    'outer: for starting_node in starting_nodes {
        let mut current_pos = starting_node as usize;
        let mut steps = 0;
        for i in instructions.bytes().cycle().take(100000) {
            steps += 1;
            let entry = map[current_pos];
            current_pos = if i == b'L' {
                entry.0 as usize
            } else {
                entry.1 as usize
            };
            if (input.is_part_one() && current_pos == 1) || (input.is_part_two() && end_nodes.contains(&(current_pos as u16))) {
                result = lcm(result, steps);
                continue 'outer;
            }
        }
        return Err("Cycle in input".to_string());
    }

    Ok(result)
}

const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

const fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    test_part_one_no_allocations!(test_input => 2);

    assert_eq!(lcm(4, 6), 12);
    assert_eq!(lcm(8, 9), 72);
    assert_eq!(lcm(1, 72), 72);

    let test_input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    test_part_two_no_allocations!(test_input => 6);

    let real_input = include_str!("day08_input.txt");
    test_part_one_no_allocations!(real_input => 20221);
    test_part_two_no_allocations!(real_input => 14616363770447);
}
