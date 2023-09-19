use crate::input::Input;

struct Board {
    numbers: [u8; 25],
    drawn_numbers_bitmask: u32,
}

impl Board {
    const fn is_bitmask_set(&self, bitmask: u32) -> bool {
        self.drawn_numbers_bitmask & bitmask == bitmask
    }

    const fn has_won(&self) -> bool {
        #![allow(clippy::unusual_byte_groupings)]
        self.is_bitmask_set(0b00000_00000_00000_00000_11111)
            || self.is_bitmask_set(0b00000_00000_00000_11111_00000)
            || self.is_bitmask_set(0b00000_00000_11111_00000_00000)
            || self.is_bitmask_set(0b00000_11111_00000_00000_00000)
            || self.is_bitmask_set(0b11111_00000_00000_00000_00000)
            || self.is_bitmask_set(0b00001_00001_00001_00001_00001)
            || self.is_bitmask_set(0b00010_00010_00010_00010_00010)
            || self.is_bitmask_set(0b00100_00100_00100_00100_00100)
            || self.is_bitmask_set(0b01000_01000_01000_01000_01000)
            || self.is_bitmask_set(0b10000_10000_10000_10000_10000)
    }

    fn note_drawn_number(&mut self, drawn_number: u8) -> bool {
        if let Some(idx) = self.numbers.iter().position(|&e| e == drawn_number) {
            self.drawn_numbers_bitmask |= 1 << idx;
            self.has_won()
        } else {
            false
        }
    }

    fn unmarked_sum(&self) -> u32 {
        self.numbers
            .iter()
            .enumerate()
            .filter(|(idx, _number)| !self.is_bitmask_set(1 << idx))
            .map(|(_idx, &number)| u32::from(number))
            .sum()
    }
}

pub fn solve(input: &Input) -> Result<u32, String> {
    let drawn_numbers = input
        .text
        .lines()
        .next()
        .unwrap_or_default()
        .split(',')
        .map(|s| {
            s.parse::<u8>()
                .map_err(|_| "Invalid drawn numbers".to_string())
        });

    let mut boards = input
        .text
        .split("\n\n")
        .skip(1)
        .map(|board_str| -> Result<Board, String> {
            let numbers = board_str
                .split_ascii_whitespace()
                .map(|n| {
                    n.parse::<u8>()
                        .map_err(|_| "Invalid board - not 25 numbers".to_string())
                })
                .collect::<Result<Vec<u8>, String>>()?;
            Ok(Board {
                numbers: numbers
                    .try_into()
                    .map_err(|_| "Invalid board - not 25 numbers".to_string())?,
                drawn_numbers_bitmask: 0,
            })
        })
        .collect::<Result<Vec<Board>, _>>()?;

    let wanted_wins = input.part_values(1, boards.len());
    let mut num_wins = 0;

    for number in drawn_numbers {
        let number = number?;
        for board in boards.iter_mut() {
            if !board.has_won() && board.note_drawn_number(number) {
                num_wins += 1;
                if num_wins == wanted_wins {
                    return Ok(board.unmarked_sum() * u32::from(number));
                }
            }
        }
    }

    Err("No board won".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
    test_part_one!(example => 4512);
    test_part_two!(example => 1924);

    let real_input = include_str!("day04_input.txt");
    test_part_one!(real_input => 8136);
    test_part_two!(real_input => 12738);
}
