use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Write};

use Amphipod::{Amber, Bronze, Copper, Desert};

use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u64, String> {
    (if input.is_part_one() {
        SearchState::<2>::parse(input.text).least_total_energy_to_organize()
    } else {
        SearchState::<4>::parse(input.text).least_total_energy_to_organize()
    })
    .ok_or_else(|| "No solution found".to_string())
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
struct SearchState<const SIDE_ROOM_SIZE: usize> {
    // First NUM_ROOMS*(2+SIDE_ROOM_SIZE*2) bits are for rooms. 40 bits max.
    // Then 7*3=21 bits for hallway.
    // So for biggest SIDE_ROOM_SIZE we fit in 61 bits.
    storage: u64,
}

impl<const SIDE_ROOM_SIZE: usize> SearchState<SIDE_ROOM_SIZE> {
    /// A room is represented by 10 bits:
    /// 2 bits counting the occupancy (starting from bottom of room)
    /// This can only hold 4 values, while the occupancy can range from
    /// 0 to 4 - that is 5 distinct values. We use:
    /// 0b11 = 4 amphipods
    /// 0b10 = 3 amphipods
    /// 0b01 = 2 amphipods
    /// 0b00 = 1 OR 0 amphipods
    ///        To determine, check if all second amphipod (0b0000_XX00) are stored with 0:s.
    ///        As we normally set unoccupied to 1:s when popping
    ///        that means it is empty.
    pub const NUM_ROOMS: usize = 4;
    pub const BITS_PER_ROOM: usize = 2 + SIDE_ROOM_SIZE * 2;
    pub const ALL_ROOM_BITS: usize = Self::NUM_ROOMS * Self::BITS_PER_ROOM;
    pub const HALLWAY_SPACES: usize = 7;
    /// First a bit if occupied, then amphipod (if occupied)
    pub const BITS_PER_HALLWAY: usize = 3;

    fn parse(text: &str) -> Self {
        let mut result = Self { storage: 0 };
        let mut amphipod_count = 0;
        for b in text.bytes().filter(|b| b'A' <= *b && *b <= b'D').rev() {
            result.push_to_room(3 - amphipod_count % 4, Amphipod::from_idx(b - b'A'));
            amphipod_count += 1;
            if SIDE_ROOM_SIZE == 4 && amphipod_count == 4 {
                for b in [b'D', b'C', b'B', b'A', b'D', b'B', b'A', b'C']
                    .iter()
                    .rev()
                {
                    result.push_to_room(3 - amphipod_count % 4, Amphipod::from_idx(b - b'A'));
                    amphipod_count += 1;
                }
            }
        }
        result
    }

    fn get_at_hallway(self, hallway_idx: usize) -> Option<Amphipod> {
        debug_assert!(
            hallway_idx < Self::HALLWAY_SPACES,
            "Hallway idx={}",
            hallway_idx
        );
        let bit_idx = Self::ALL_ROOM_BITS + Self::BITS_PER_HALLWAY * hallway_idx;
        if self.storage & (1 << (bit_idx + 2)) == 0 {
            return None;
        }
        Some(Amphipod::from_idx(((self.storage >> bit_idx) & 0b11) as u8))
    }

    fn set_at_hallway(&mut self, hallway_idx: usize, amphipod: Option<Amphipod>) {
        debug_assert!(
            hallway_idx <= 6,
            "Max hallway_idx is 6 (was {})",
            hallway_idx
        );
        let bit_idx = Self::ALL_ROOM_BITS + Self::BITS_PER_HALLWAY * hallway_idx;
        match amphipod {
            None => {
                // Need to clear all bits, for is_hallway_empty() to work fast.
                self.storage &= !(0b111 << bit_idx);
            }
            Some(a) => {
                self.storage |= 1 << (bit_idx + 2);
                self.storage &= !(0b11 << bit_idx);
                self.storage |= (a as u64) << bit_idx;
            }
        }
    }

    const fn occupancy_in_room(self, room_idx: u8) -> u8 {
        let room_bit_idx = Self::BITS_PER_ROOM * (room_idx as usize);
        let room_len_bit_idx = room_bit_idx + SIDE_ROOM_SIZE * 2;

        let amphipods_in_room_storage = ((self.storage >> room_len_bit_idx) & 0b11) as u8;
        if amphipods_in_room_storage == 0 && ((self.storage >> room_bit_idx) & 0b0000_1100) == 0 {
            // If we set all bits to 0:s, that means it is empty
            0
        } else {
            amphipods_in_room_storage + 1
        }
    }

    const fn get_at_room(self, room_idx: u8, room_depth: u8) -> Option<Amphipod> {
        let amphipods_in_room = self.occupancy_in_room(room_idx);
        if room_depth >= amphipods_in_room {
            None
        } else {
            let bit_idx = Self::BITS_PER_ROOM as u8 * room_idx;
            Some(Amphipod::from_idx(
                (self.storage >> (bit_idx + room_depth * 2) & 0b11) as u8,
            ))
        }
    }

    /// Returns offset of pushed amphipod.
    fn push_to_room(&mut self, room_idx: u8, amphipod: Amphipod) -> u8 {
        let room_bit_idx = Self::BITS_PER_ROOM * (room_idx as usize);
        let room_len_bit_idx = room_bit_idx + SIDE_ROOM_SIZE * 2;

        let amphipods_initially_in_room = self.occupancy_in_room(room_idx);
        debug_assert!(
            amphipods_initially_in_room < SIDE_ROOM_SIZE as u8,
            "Cannot push to full side room"
        );
        let new_amphipods_in_room = amphipods_initially_in_room + 1;

        // Clear and set storage bits:
        let amphipods_in_room_storage = if new_amphipods_in_room == 0 {
            0
        } else {
            new_amphipods_in_room - 1
        };
        self.storage &= !(0b11 << room_len_bit_idx);
        self.storage |= u64::from(amphipods_in_room_storage) << room_len_bit_idx;

        // Set amphipod bits
        let amphipod_bit_idx = room_bit_idx + 2 * (amphipods_initially_in_room as usize);
        self.storage &= !(0b11 << amphipod_bit_idx);
        self.storage |= (amphipod as u64) << amphipod_bit_idx;

        // Special hack to differentiate 0 and 1 as room occupancy:
        if new_amphipods_in_room == 1 {
            self.storage |= 0b11 << (amphipod_bit_idx + 2);
        }

        debug_assert_eq!(self.occupancy_in_room(room_idx), new_amphipods_in_room);
        SIDE_ROOM_SIZE as u8 - new_amphipods_in_room
    }

    // Returns (popped_depth, amphipod) if there is an amphipod.
    fn pop_room(&mut self, room_idx: u8) -> Option<(u8, Amphipod)> {
        let room_bit_idx = Self::BITS_PER_ROOM * (room_idx as usize);
        let room_len_bit_idx = room_bit_idx + SIDE_ROOM_SIZE * 2;
        let amphipods_in_room = self.occupancy_in_room(room_idx);

        if amphipods_in_room == 0 {
            return None;
        }
        let new_amphipods_in_room = amphipods_in_room - 1;

        // Clear and set storage bits:
        let amphipods_in_room_storage = if new_amphipods_in_room == 0 {
            0
        } else {
            new_amphipods_in_room - 1
        };
        self.storage &= !(0b11 << room_len_bit_idx);
        self.storage |= u64::from(amphipods_in_room_storage) << room_len_bit_idx;

        // Inspect amphipod to pop:
        let amphipod_bit_idx = room_bit_idx + 2 * (new_amphipods_in_room as usize);
        let result = Some((
            SIDE_ROOM_SIZE as u8 - amphipods_in_room,
            Amphipod::from_idx(((self.storage >> amphipod_bit_idx) & 0b11) as u8),
        ));

        // Special hack to differentiate 0 and 1 as room occupancy:
        if new_amphipods_in_room == 0 {
            self.storage &= !(0b11 << (amphipod_bit_idx + 2));
        } else if new_amphipods_in_room == 1 {
            self.storage |= 0b11 << (amphipod_bit_idx);
        }

        debug_assert_eq!(self.occupancy_in_room(room_idx), new_amphipods_in_room);

        result
    }

    const fn is_hallway_empty(self) -> bool {
        self.storage >> Self::ALL_ROOM_BITS == 0
    }

    fn can_amphipod_go_home(self, amphipod: Amphipod) -> bool {
        let room_idx = amphipod as u8;
        let occupancy = self.occupancy_in_room(room_idx);
        if occupancy == 0 {
            return true;
        }
        for i in 0..occupancy {
            if let Some(amphipod_in_room) = self.get_at_room(amphipod as u8, i) {
                if amphipod != amphipod_in_room {
                    return false;
                }
            }
        }
        true
    }

    fn enumerate_possible_moves(self, moves: &mut Vec<(u64, Self)>) {
        'hallway_loop: for hallway_idx in 0..Self::HALLWAY_SPACES {
            if let Some(amphipod) = self.get_at_hallway(hallway_idx) {
                if !self.can_amphipod_go_home(amphipod) {
                    continue;
                }
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
                    if !matches!(self.get_at_hallway(current_hallway_idx), None) {
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

                let mut new_state = self;
                new_state.set_at_hallway(hallway_idx, None);
                let offset_in_room = new_state.push_to_room(amphipod as u8, amphipod);
                let total_travel_cost = ((offset_in_room as usize + 1 + hallway_travel_distance)
                    * amphipod.consumption() as usize)
                    as u64;
                moves.push((total_travel_cost, new_state));
                // No need to consider more moves:
                return;
            }
        }

        for room_idx in 0..Self::NUM_ROOMS {
            let mut new_state_from_leaving_room = self;
            if let Some((offset_in_room, amphipod)) =
                new_state_from_leaving_room.pop_room(room_idx as u8)
            {
                if amphipod as usize == room_idx && self.can_amphipod_go_home(amphipod) {
                    // No point in leaving last of owns room. TODO: Generalise, avoid leaving if e.g. two A furthest in?
                    continue;
                }
                for horizontal_direction in [-1, 1] {
                    // [ H0 H1  H2  H3  H4  H5 H6 ]
                    //        R0  R1  R2  R3
                    // When going left from room 0, coming to 1+0+0 = 1
                    // When going right from room 1, coming to 1+0+1 = 2
                    let mut hallway_end_idx = 1 + room_idx + usize::from(horizontal_direction == 1);
                    let mut hallway_travel_distance = 1;

                    while self.get_at_hallway(hallway_end_idx).is_none() {
                        let mut new_state = new_state_from_leaving_room;
                        new_state.set_at_hallway(hallway_end_idx, Some(amphipod));
                        let total_travel_cost =
                            (1 + u64::from(offset_in_room) + hallway_travel_distance as u64)
                                * u64::from(amphipod.consumption());
                        moves.push((total_travel_cost, new_state));

                        let from_hallway_idx = hallway_end_idx;
                        let hallway_end_idx_signed = hallway_end_idx as i32 + horizontal_direction;
                        if hallway_end_idx_signed < 0
                            || hallway_end_idx_signed >= Self::HALLWAY_SPACES as i32
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
            }
        }
    }

    fn least_total_energy_to_organize(self) -> Option<u64> {
        let mut to_visit = BinaryHeap::from([(Reverse(0), self)]);
        let mut lowest_cost: HashMap<Self, u64> = HashMap::from([(self, 0)]);
        let mut new_states = Vec::new();

        while let Some((Reverse(cost), state)) = to_visit.pop() {
            if cost > 0 && state.is_hallway_empty() {
                // If the hallway is empty after we have left the initial position we are done.
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
                    to_visit.push((Reverse(new_total_cost), new_state));
                }
            }
        }

        None
    }
}

const fn to_char(a: Option<Amphipod>) -> char {
    match a {
        Some(amphipod) => ((amphipod as u8) + b'A') as char,
        None => '.',
    }
}

impl<const SIDE_ROOM_SIZE: usize> Debug for SearchState<SIDE_ROOM_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("#############\n")?;
        writeln!(
            f,
            "#{}{}.{}.{}.{}.{}{}#",
            to_char(self.get_at_hallway(0)),
            to_char(self.get_at_hallway(1)),
            to_char(self.get_at_hallway(2)),
            to_char(self.get_at_hallway(3)),
            to_char(self.get_at_hallway(4)),
            to_char(self.get_at_hallway(5)),
            to_char(self.get_at_hallway(6)),
        )?;
        for side_room_offset in 0..SIDE_ROOM_SIZE {
            let start_and_end = if side_room_offset == 0 { "##" } else { "  " };
            writeln!(
                f,
                "{}#{}#{}#{}#{}#{}",
                start_and_end,
                to_char(self.get_at_room(0, SIDE_ROOM_SIZE as u8 - 1 - side_room_offset as u8)),
                to_char(self.get_at_room(1, SIDE_ROOM_SIZE as u8 - 1 - side_room_offset as u8)),
                to_char(self.get_at_room(2, SIDE_ROOM_SIZE as u8 - 1 - side_room_offset as u8)),
                to_char(self.get_at_room(3, SIDE_ROOM_SIZE as u8 - 1 - side_room_offset as u8)),
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

#[test]
pub fn test_search_state() {
    let mut s = SearchState::<4>::default();
    assert!(s.is_hallway_empty());
    for i in 0..7 {
        assert_eq!(s.get_at_hallway(i), None);
    }
    for i in 0..7 {
        s.set_at_hallway(i, Some(Amphipod::from_idx((i % 4) as u8)));
        assert!(!s.is_hallway_empty());
        for j in 0..7 {
            if j <= i {
                assert_eq!(s.get_at_hallway(j), Some(Amphipod::from_idx((j % 4) as u8)));
            } else {
                assert_eq!(s.get_at_hallway(j), None);
            }
        }
    }

    for i in 0..7 {
        s.set_at_hallway(i, None);
        assert_eq!(None, s.get_at_hallway(i));
        assert_eq!(i == 6, s.is_hallway_empty());
    }

    for room_idx in 0..=3 {
        assert_eq!(s.occupancy_in_room(room_idx), 0);
        for room_depth in 0..=3 {
            assert_eq!(s.get_at_room(room_idx, room_depth), None);
        }
    }

    for room_idx in 0..=3 {
        assert_eq!(s.occupancy_in_room(room_idx), 0);
        for room_depth in 0..=3 {
            assert_eq!(s.get_at_room(room_idx, room_depth), None);
        }
    }

    for room_idx in 0..4 {
        for amphipod_num in 0..4 {
            let pushed_amphipod = Amphipod::from_idx(amphipod_num % 4);
            s.push_to_room(room_idx, pushed_amphipod);
            assert_eq!(Some(pushed_amphipod), s.get_at_room(room_idx, amphipod_num));
            assert_eq!(s.occupancy_in_room(room_idx), amphipod_num + 1);
        }
    }
}

#[test]
pub fn test_search_state_push_pop() {
    let mut s = SearchState::<4>::default();
    for room_idx in 0..4 {
        for amphipod_num in 0..4 {
            let pushed_amphipod = Amphipod::from_idx(amphipod_num % 4);
            assert_eq!(3 - amphipod_num, s.push_to_room(room_idx, pushed_amphipod));
        }
        for amphipod_num in 0..4 {
            let pushed_amphipod = Amphipod::from_idx(amphipod_num % 4);
            assert_eq!(Some(pushed_amphipod), s.get_at_room(room_idx, amphipod_num));
        }
        for amphipod_num in (0..4).rev() {
            let expected = Amphipod::from_idx(amphipod_num % 4);
            let actual = s.pop_room(room_idx);
            assert_eq!(Some((3 - amphipod_num, expected)), actual);
            assert_eq!(amphipod_num, s.occupancy_in_room(room_idx));
        }
    }
}
