use crate::input::Input;
use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Write};
use Amphipod::{Amber, Bronze, Copper, Desert};

pub fn solve(input: &mut Input) -> Result<u64, String> {
    (if input.is_part_one() {
        State::<2>::parse(input.text).least_total_energy_to_organize()
    } else {
        State::<4>::parse(input.text).least_total_energy_to_organize()
    })
    .ok_or_else(|| "No solution found".to_string())
}

const HALLWAY_SPACES: usize = 7;

/// [ H0 H1  H2  H3  H4  H5 H6 ]
///        R0  R1  R2  R3
///        R0  R1  R2  R3
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct State<const SIDE_ROOM_SIZE: usize> {
    hallways: [Option<Amphipod>; HALLWAY_SPACES],
    /// Indexed by amphipod idx
    rooms: [[Option<Amphipod>; SIDE_ROOM_SIZE]; 4],
}

impl<const SIDE_ROOM_SIZE: usize> State<SIDE_ROOM_SIZE> {
    fn parse(text: &str) -> Self {
        let mut rooms = [[Option::None; SIDE_ROOM_SIZE]; 4];
        let mut amphipod_count = 0;
        for b in text.bytes().filter(|b| b'A' <= *b && *b <= b'D') {
            rooms[amphipod_count % 4][amphipod_count / 4] = Some(Amphipod::from_idx(b - b'A'));
            amphipod_count += 1;
            if SIDE_ROOM_SIZE == 4 && amphipod_count == 4 {
                for b in [b'D', b'C', b'B', b'A', b'D', b'B', b'A', b'C'] {
                    rooms[amphipod_count % 4][amphipod_count / 4] =
                        Some(Amphipod::from_idx(b - b'A'));
                    amphipod_count += 1;
                }
            }
        }
        Self {
            hallways: [Option::None; HALLWAY_SPACES],
            rooms,
        }
    }

    fn is_organized(&self) -> bool {
        self.rooms.iter().enumerate().all(|(room_idx, room)| {
            room.iter()
                .all(|occupancy| matches!(occupancy, &Some(a) if (a as usize) == room_idx))
        })
    }

    // TODO: Stack allocated moves (not many possible, return slice)?
    fn enumerate_possible_moves(&self, moves: &mut Vec<(u64, Self)>) {
        for room_idx in 0..self.rooms.len() {
            for (offset_in_room, occupancy) in self.rooms[room_idx].iter().enumerate() {
                match occupancy {
                    &Some(amphipod) => {
                        for horizontal_direction in [-1, 1] {
                            // [ H0 H1  H2  H3  H4  H5 H6 ]
                            //        R0  R1  R2  R3
                            // When going left from room 0, coming to 1+0+0 = 1
                            // When going right from room 1, coming to 1+0+1 = 2
                            let mut hallway_end_idx =
                                1 + room_idx + if horizontal_direction == 1 { 1 } else { 0 };
                            let mut hallway_travel_distance = 1;

                            while matches!(self.hallways[hallway_end_idx], None) {
                                let mut new_hallways = self.hallways;
                                new_hallways[hallway_end_idx] = Some(amphipod);
                                let mut new_rooms = self.rooms;
                                new_rooms[room_idx][offset_in_room] = None;
                                let total_travel_cost =
                                    (1 + offset_in_room as u64 + hallway_travel_distance as u64)
                                        * u64::from(amphipod.consumption());
                                moves.push((
                                    total_travel_cost,
                                    Self {
                                        hallways: new_hallways,
                                        rooms: new_rooms,
                                    },
                                ));

                                let from_hallway_idx = hallway_end_idx;
                                let hallway_end_idx_signed =
                                    hallway_end_idx as i32 + horizontal_direction;
                                if hallway_end_idx_signed < 0
                                    || hallway_end_idx_signed >= HALLWAY_SPACES as i32
                                {
                                    break;
                                }
                                hallway_end_idx = hallway_end_idx_signed as usize;
                                hallway_travel_distance += 1;
                                if !matches!(
                                    (from_hallway_idx, hallway_end_idx),
                                    (0, 1) | (1, 0) | (5, 6) | (6, 5)
                                ) {
                                    hallway_travel_distance += 1;
                                }
                            }
                        }
                        break;
                    }
                    None => {}
                }
            }
        }
        'hallway_loop: for hallway_idx in 0..self.hallways.len() {
            if let Some(amphipod) = self.hallways[hallway_idx] {
                let can_go_to_room =
                    self.rooms[amphipod as usize]
                        .iter()
                        .all(|&occupancy| match occupancy {
                            None => true,
                            Some(a) if a == amphipod => true,
                            _ => false,
                        });
                if can_go_to_room {
                    let (end_idx, direction) = if (amphipod as usize) + 1 < hallway_idx {
                        ((amphipod as usize) + 2, -1)
                    } else {
                        ((amphipod as usize) + 1, 1)
                    };
                    let mut current_hallway_idx = hallway_idx;
                    let mut hallway_travel_distance = 1;
                    while current_hallway_idx != end_idx {
                        let from_hallway_idx = current_hallway_idx;
                        current_hallway_idx = ((current_hallway_idx as i32) + direction) as usize;
                        if !matches!(self.hallways[current_hallway_idx], None) {
                            continue 'hallway_loop;
                        }

                        hallway_travel_distance += 1;
                        if !matches!(
                            (from_hallway_idx, current_hallway_idx),
                            (0, 1) | (1, 0) | (5, 6) | (6, 5)
                        ) {
                            hallway_travel_distance += 1;
                        }
                    }

                    let mut new_hallways = self.hallways;
                    new_hallways[hallway_idx] = None;
                    let mut new_rooms = self.rooms;
                    let offset_in_room = self.rooms[amphipod as usize]
                        .iter()
                        .enumerate()
                        .rev()
                        .find_map(|(room_offset, &occupancy)| match occupancy {
                            None => Some(room_offset),
                            _ => None,
                        })
                        .unwrap_or_default();
                    new_rooms[amphipod as usize][offset_in_room] = Some(amphipod);
                    let total_travel_cost = ((offset_in_room + 1 + hallway_travel_distance)
                        * amphipod.consumption() as usize)
                        as u64;
                    moves.push((
                        total_travel_cost,
                        Self {
                            hallways: new_hallways,
                            rooms: new_rooms,
                        },
                    ));
                }
            }
        }
    }

    fn heuristic_cost(&self) -> u64 {
        let mut heuristic = 0;

        for room_idx in 0..self.rooms.len() {
            for (offset_in_room, occupancy) in self.rooms[room_idx].iter().enumerate() {
                if let Some(amphipod) = *occupancy {
                    if amphipod as usize != room_idx {
                        let hallway_distance_from_own_room =
                            ((room_idx as i32 - (amphipod as i32)).abs() * 2 + 1) as u64;
                        heuristic += (offset_in_room as u64 + hallway_distance_from_own_room)
                            * u64::from(amphipod.consumption());
                    }
                }
            }
        }

        heuristic / 1000
    }

    fn least_total_energy_to_organize(self) -> Option<u64> {
        let mut to_visit = BinaryHeap::from([(Reverse(0), 0, self)]);
        let mut lowest_cost: HashMap<Self, u64> = HashMap::from([(self, 0)]);
        let mut new_states = Vec::new();

        while let Some((_cost_and_heuristic, cost, state)) = to_visit.pop() {
            if cost > *lowest_cost.get(&state).unwrap_or(&u64::MAX) {
                continue;
            }

            if state.is_organized() {
                return Some(cost);
            }

            new_states.clear();
            state.enumerate_possible_moves(&mut new_states);
            for &(new_state_cost_diff, new_state) in new_states.iter() {
                let new_total_cost = cost + new_state_cost_diff;

                let visit_this = match lowest_cost.entry(new_state) {
                    Entry::Vacant(entry) => {
                        entry.insert(new_total_cost);
                        true
                    }
                    Entry::Occupied(mut entry) if new_total_cost < *entry.get() => {
                        entry.insert(new_total_cost);
                        true
                    }
                    _ => false,
                };
                if visit_this {
                    to_visit.push((
                        Reverse(new_total_cost + state.heuristic_cost()),
                        new_total_cost,
                        new_state,
                    ));
                }
            }
        }

        None
    }
}

impl<const SIDE_ROOM_SIZE: usize> Debug for State<SIDE_ROOM_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("#############\n")?;
        writeln!(
            f,
            "#{:?}{:?}.{:?}.{:?}.{:?}.{:?}{:?}#",
            self.hallways[0],
            self.hallways[1],
            self.hallways[2],
            self.hallways[3],
            self.hallways[4],
            self.hallways[5],
            self.hallways[6]
        )?;
        for side_room_offset in 0..SIDE_ROOM_SIZE {
            let start_and_end = if side_room_offset == 0 { "##" } else { "  " };
            writeln!(
                f,
                "{}#{:?}#{:?}#{:?}#{:?}#{}",
                start_and_end,
                self.rooms[0][side_room_offset],
                self.rooms[1][side_room_offset],
                self.rooms[2][side_room_offset],
                self.rooms[3][side_room_offset],
                start_and_end
            )?;
        }
        write!(f, "  #########  ")
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Amphipod {
    Amber = 0,
    Bronze,
    Copper,
    Desert,
}

impl Debug for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let c = (*self as u8) + b'A';
        f.write_char(c as char)
    }
}

impl Amphipod {
    const fn from_idx(idx: u8) -> Self {
        match idx {
            0 => Amber,
            1 => Bronze,
            2 => Copper,
            _ => Desert,
        }
    }
}

impl Amphipod {
    const fn consumption(self) -> u16 {
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
        }
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
    test_part_one!(example => 12521);
    test_part_two!(example => 44169);

    let real_input = include_str!("day23_input.txt");
    test_part_one!(real_input => 14_460);
    test_part_two!(real_input => 41_366);
}
