use crate::common::array_stack::ArrayStack;
use crate::common::id_assigner_copy::IdAssigner;
use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAX_ENTRIES: usize = 1024;
    const MAX_START_END_NODES: usize = 32;

    let mut id_assigner = IdAssigner::<MAX_ENTRIES, u32>::new(0);

    let (instructions, map_lines) = input.text.split_once("\n\n").ok_or_else(on_error)?;

    let a = u32::from(b'A');
    let _ = id_assigner.id_of((a << 16) + (a << 8) + a);
    let z = u32::from(b'Z');
    let _ = id_assigner.id_of((z << 16) + (z << 8) + z);

    let mut map = [(0, 0); MAX_ENTRIES];

    let mut starting_nodes = ArrayStack::<MAX_START_END_NODES, u16>::new();
    let mut end_nodes = ArrayStack::<MAX_START_END_NODES, u16>::new();

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
                if str_count == 3 || (start_idx + 3 != idx) {
                    return Err("Invalid input".to_string());
                }
                let key = (u32::from(bytes[start_idx]) << 16)
                    + (u32::from(bytes[start_idx + 1]) << 8)
                    + u32::from(bytes[start_idx + 2]);
                ids[str_count] = id_assigner.id_of(key)?;
                if str_count == 0 && bytes[2] == b'A' {
                    starting_nodes.push(ids[str_count])?;
                } else if str_count == 0 && bytes[2] == b'Z' {
                    end_nodes.push(ids[str_count])?;
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
        map[from_id] = to;
    }

    let (starting_nodes, end_nodes) = if input.is_part_one() {
        (&[0][..], &[1][..])
    } else {
        (starting_nodes.slice(), end_nodes.slice())
    };

    let mut result = 1;
    'outer: for &starting_node in starting_nodes {
        let mut current_pos = starting_node as usize;
        let mut steps = 0;
        for i in instructions
            .bytes()
            .cycle()
            .take(id_assigner.len() * id_assigner.len())
        {
            steps += 1;
            let entry = map[current_pos];
            current_pos = if i == b'L' {
                entry.0 as usize
            } else {
                entry.1 as usize
            };
            if (input.is_part_one() && current_pos == 1)
                || (input.is_part_two() && end_nodes.contains(&(current_pos as u16)))
            {
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
    test_part_two_no_allocations!(real_input => 14_616_363_770_447);
}
