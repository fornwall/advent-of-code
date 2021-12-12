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

struct CaveMap {
    connections: [u16; CaveMap::MAX_SIZE],
    big_caves: u16,
    start: u8,
    end: u8,
}

impl CaveMap {
    const MAX_SIZE: usize = 16;

    const fn path_from(&self, from: u8, to: u8) -> bool {
        (self.connections[from as usize] & (1 << to)) > 0
    }

    fn parse(text: &str) -> Result<Self, String> {
        let mut current_id = 0_u8;
        let mut result = Self {
            connections: [0; Self::MAX_SIZE],
            big_caves: 0,
            start: u8::MAX,
            end: u8::MAX,
        };

        let mut map = HashMap::new();
        for line in text.lines() {
            if let Some((from, to)) = line.split_once('-') {
                let from_id = *map.entry(from).or_insert_with(|| {
                    current_id += 1;
                    current_id - 1
                });
                let to_id = *map.entry(to).or_insert_with(|| {
                    current_id += 1;
                    current_id - 1
                });
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
                } else if to == "end" {
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
        for i in 0..16 {
            let i_bitmask = 1 << i;
            let has_visited_once = visited_once & i_bitmask > 0;
            let is_big_cave = self.big_caves & i_bitmask > 0;
            let i_is_cave_start_or_end = i == self.start || i == self.end;
            if self.path_from(start, i)
                && matches!(
                    (
                        has_visited_once,
                        visited_small_twice,
                        is_big_cave,
                        i_is_cave_start_or_end,
                        part2
                    ),
                    // Can always visit a big cave:
                    (_, _, true, _, _)
                // Can always visit an unvisited cave:
                | (false, _, _, _, _)
                // In part 2, can always visit a cave we haven't
                // visited twice (unless it is the cave start or end):
                | (_, false, _, false, true)
                )
            {
                let now_visited_small_twice =
                    visited_small_twice || (has_visited_once && !is_big_cave);
                result += self.search(
                    visited_once | i_bitmask,
                    now_visited_small_twice,
                    i,
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
