use crate::input::Input;
use std::collections::HashMap;

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let map = CaveMap::parse(input.text)?;

    let visited_once = 1 << map.start;
    let visited_small_twice = false;
    Ok(map.search(
        visited_once,
        visited_small_twice,
        map.start,
        map.end,
        input.is_part_two(),
    ))
}

struct IdAssigner<'a> {
    id_map: HashMap<&'a str, u8>,
}

impl<'a> IdAssigner<'a> {
    fn new() -> Self {
        Self {
            id_map: HashMap::new(),
        }
    }

    fn id_of(&mut self, name: &'a str) -> u8 {
        let next_id = self.id_map.len() as u8;
        *self.id_map.entry(name).or_insert(next_id)
    }
}

struct CaveMap {
    connections: [u16; CaveMap::MAX_SIZE],
    big_caves: u16,
    start: u8,
    end: u8,
}

impl CaveMap {
    const MAX_SIZE: usize = 16;

    fn parse(text: &str) -> Result<Self, String> {
        let mut result = Self {
            connections: [0; Self::MAX_SIZE],
            big_caves: 0,
            start: u8::MAX,
            end: u8::MAX,
        };

        let mut id_assigner = IdAssigner::new();

        for line in text.lines() {
            if let Some((from, to)) = line.split_once('-') {
                let from_id = id_assigner.id_of(from);
                let to_id = id_assigner.id_of(to);

                if to_id >= Self::MAX_SIZE as u8 {
                    return Err(format!(
                        "Too many distinct nodes - only {} supported",
                        Self::MAX_SIZE
                    ));
                }

                result.connections[from_id as usize] |= 1 << to_id;
                result.connections[to_id as usize] |= 1 << from_id;

                if from == "start" {
                    result.start = from_id;
                };

                if to == "end" {
                    result.end = to_id;
                };

                if from.bytes().all(|b| b.is_ascii_uppercase()) {
                    result.big_caves |= 1 << from_id;
                }
                if to.bytes().all(|b| b.is_ascii_uppercase()) {
                    result.big_caves |= 1 << to_id;
                }
            } else {
                return Err("Not all lines have the format '${FROM}-${TO}".to_string());
            }
        }

        Ok(result)
    }

    fn search(
        &self,
        visited_once: u16,
        visited_small_twice: bool,
        start: u8,
        end: u8,
        part2: bool,
    ) -> u64 {
        if start == end {
            return 1;
        }

        let mut result = 0;
        let mut connections_from_start = self.connections[start as usize];
        while connections_from_start != 0 {
            let destination_id = connections_from_start.trailing_zeros() as u8;
            let destination_bitmask = 1 << destination_id;
            connections_from_start ^= destination_bitmask;
            let has_visited_once = visited_once & destination_bitmask > 0;
            let is_big_cave = self.big_caves & destination_bitmask > 0;
            let is_start_or_end = destination_id == self.start || destination_id == self.end;
            if matches!(
                (
                    has_visited_once,
                    visited_small_twice,
                    is_big_cave,
                    is_start_or_end,
                    part2
                ),
                // Can always visit a big cave:
                (_, _, true, _, _)
                // Can always visit an unvisited cave:
                | (false, _, _, _, _)
                // In part 2, can always visit a cave we haven't
                // visited twice (unless it is the cave start or end):
                | (_, false, _, false, true)
            ) {
                let now_visited_small_twice =
                    visited_small_twice || (has_visited_once && !is_big_cave);
                result += self.search(
                    visited_once | destination_bitmask,
                    now_visited_small_twice,
                    destination_id,
                    end,
                    part2,
                );
            }
        }
        result
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    test_part_one!(example => 10);
    test_part_two!(example => 36);

    let example = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    test_part_one!(example => 19);
    test_part_two!(example => 103);

    let example = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    test_part_one!(example => 226);
    test_part_two!(example => 3509);

    let real_input = include_str!("day12_input.txt");
    test_part_one!(real_input => 4495);
    test_part_two!(real_input => 131_254);
}
