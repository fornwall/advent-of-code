use crate::common::id_assigner::IdAssigner;

use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    let map = CaveMap::parse(input.text)?;

    let visited_once = 1 << map.start_cave_identifier;
    let visited_small_twice = false;
    Ok(map.search(
        visited_once,
        visited_small_twice,
        map.start_cave_identifier,
        map.end_cave_identifier,
        input.is_part_two(),
    ))
}

struct CaveMap {
    /// Indexed by start node id, and mapping to bitset of connected caves.
    connections: [u16; CaveMap::MAX_SIZE],
    /// Bitset of which caves are big.
    big_caves: u16,
    start_cave_identifier: u8,
    end_cave_identifier: u8,
}

impl CaveMap {
    const MAX_SIZE: usize = 16;

    fn parse(text: &str) -> Result<Self, String> {
        let mut result = Self {
            connections: [0; Self::MAX_SIZE],
            big_caves: 0,
            start_cave_identifier: u8::MAX,
            end_cave_identifier: u8::MAX,
        };

        let mut id_assigner = IdAssigner::<{ Self::MAX_SIZE }, str>::new("");

        for line in text.lines() {
            if let Some((from, to)) = line.split_once('-') {
                let from_id = id_assigner.id_of(from)? as u8;
                let to_id = id_assigner.id_of(to)? as u8;

                let from_bitmask = 1 << from_id;
                let to_bitmask = 1 << to_id;

                result.connections[to_id as usize] |= from_bitmask;
                result.connections[from_id as usize] |= to_bitmask;

                if from == "start" {
                    result.start_cave_identifier = from_id;
                } else if from == "end" {
                    result.end_cave_identifier = from_id;
                }
                if to == "start" {
                    result.start_cave_identifier = to_id;
                } else if to == "end" {
                    result.end_cave_identifier = to_id;
                }

                let from_is_big = from.bytes().all(|b| b.is_ascii_uppercase());
                let to_is_big = to.bytes().all(|b| b.is_ascii_uppercase());
                if from_is_big {
                    if to_is_big {
                        return Err("Two big caves cannot be connected".to_string());
                    }
                    result.big_caves |= from_bitmask;
                } else if to_is_big {
                    result.big_caves |= to_bitmask;
                };
            } else {
                return Err("Not all lines have the format '${FROM}-${TO}".to_string());
            }
        }

        if result.start_cave_identifier == u8::MAX {
            return Err("No start cave connected".to_string());
        }
        if result.end_cave_identifier == u8::MAX {
            return Err("No end cave connected".to_string());
        }

        Ok(result)
    }

    fn search(
        &self,
        visited_once_bitset: u16,
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

            let has_visited_destination_once = visited_once_bitset & destination_bitmask > 0;
            let destination_is_big_cave = self.big_caves & destination_bitmask > 0;
            let destination_is_start_or_end_cave = destination_id == self.start_cave_identifier
                || destination_id == self.end_cave_identifier;

            if matches!(
                (
                    has_visited_destination_once,
                    visited_small_twice,
                    destination_is_big_cave,
                    destination_is_start_or_end_cave,
                    part2
                ),
                // Can always visit a big cave:
                (_, _, true, _, _)
                // Can always visit an unvisited cave:
                | (false, _, _, _, _)
                // Can always visit a cave in part 2 if it has not been
                // visited twice (unless it is the start or end cave):
                | (_, false, _, false, true)
            ) {
                let now_visited_small_twice = visited_small_twice
                    || (has_visited_destination_once && !destination_is_big_cave);
                result += self.search(
                    visited_once_bitset | destination_bitmask,
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
