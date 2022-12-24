use std::collections::HashSet;

use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    const DIRECTIONS: [(i16, i16); 8] = [
        // NW, N, NE
        (-1, 1),
        (0, 1),
        (1, 1),
        // E, SE
        (1, 0),
        (1, -1),
        // S, SW
        (0, -1),
        (-1, -1),
        // W
        (-1, 0),
    ];

    const RULES: [(i16, (i16, i16)); 4] = [
        // "If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step."
        (0b0000_0111, (0, 1)),
        // "If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step."
        (0b0111_0000, (0, -1)),
        // "If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step."
        (0b1100_0001, (-1, 0)),
        // "If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step."
        (0b0001_1100, (1, 0)),
    ];

    let mut elves = input
        .text
        .lines()
        .rev()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter_map(move |(x, c)| (c == b'#').then_some((x as i16, y as i16)))
        })
        .collect::<HashSet<_>>();

    for round in 0..input.part_values(10, 10000) {
        let mut new_elves = HashSet::with_capacity(elves.len());
        let mut any_moved = false;
        'elf: for elf in elves.iter() {
            let adjacent_bitmask = DIRECTIONS
                .iter()
                .enumerate()
                .fold(0, |acc, (idx, (dx, dy))| {
                    acc | if elves.contains(&(elf.0 + dx, elf.1 + dy)) {
                        1 << idx
                    } else {
                        0
                    }
                });

            // "During the first half of each round, each Elf considers the eight positions adjacent to themself.
            // If no other Elves are in one of those eight positions, the Elf does not do anything during this round."
            if adjacent_bitmask != 0 {
                'rule: for rule_offset in 0..RULES.len() {
                    let (check_mask, to_move) = RULES[(round + rule_offset) % RULES.len()];

                    if (check_mask & adjacent_bitmask) == 0 {
                        let new_elf = (elf.0 + to_move.0, elf.1 + to_move.1);
                        if new_elves.remove(&new_elf) {
                            // This was occupied - push other elf back (coming from other direction
                            new_elves.insert((new_elf.0 + to_move.0, new_elf.1 + to_move.1));
                            break 'rule;
                        } else {
                            new_elves.insert(new_elf);
                            any_moved = true;
                            continue 'elf;
                        }
                    }
                }
            }

            new_elves.insert(*elf);
            continue 'elf;
        }

        if !any_moved {
            return Ok(round + 1);
        }

        elves = new_elves;
    }

    let (min_x, max_x, min_y, max_y) =
        elves
            .iter()
            .fold((i16::MAX, i16::MIN, i16::MAX, i16::MIN), |acc, e| {
                (
                    acc.0.min(e.0),
                    acc.1.max(e.0),
                    acc.2.min(e.1),
                    acc.3.max(e.1),
                )
            });
    let rectangle_size = ((max_x + 1 - min_x) * (max_y + 1 - min_y)) as usize;
    Ok(rectangle_size - elves.len())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
    test_part_one!(test_input => 110);
    test_part_two!(test_input => 20);

    let real_input = include_str!("day23_input.txt");
    test_part_one!(real_input => 3920);
    test_part_two!(real_input => 889);
}
