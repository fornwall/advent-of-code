use crate::input::Input;
use std::collections::HashMap;
use std::fmt;

/// A 2x2 tile represented as bits. Example: "../.#" is stored as 0b_10_00.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Tile2 {
    bits: u8,
}

impl Tile2 {
    fn from(input: &str) -> Self {
        let bytes = input.as_bytes();
        let bits = (0..4).fold(0_u8, |acc, bit_offset| {
            let byte_idx = bit_offset + bit_offset / 2;
            acc + if bytes[byte_idx] == b'#' {
                1 << bit_offset
            } else {
                0
            }
        });
        Self { bits }
    }

    fn rotate(self) -> Self {
        // AB   =>   CA
        // CD        DB
        // First bit  (A): Shift higher 1
        // Second bit (B): Shift higher 2
        // Third bit  (C): Shift lower 2
        // Fourth bit (D): Shift lower 1
        Self {
            bits: ((self.bits & 0b_00_01) << 1)
                + ((self.bits & 0b_00_10) << 2)
                + ((self.bits & 0b_01_00) >> 2)
                + ((self.bits & 0b_10_00) >> 1),
        }
    }

    fn flip(self) -> Self {
        // AB   =>   BA
        // CD        DC
        // First bit  (A): Shift higher 1
        // Second bit (B): Shift lower 1
        // Third bit  (C): Shift higher 1
        // Fourth bit (D): Shift lower 1
        Self {
            bits: ((self.bits & 0b_00_01) << 1)
                + ((self.bits & 0b_00_10) >> 1)
                + ((self.bits & 0b_01_00) << 1)
                + ((self.bits & 0b_10_00) >> 1),
        }
    }
}

impl fmt::Debug for Tile2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            format!(
                "{}{}/{}{}",
                if (self.bits & 0b_0001) == 0 { '.' } else { '#' },
                if (self.bits & 0b_0010) == 0 { '.' } else { '#' },
                if (self.bits & 0b_0100) == 0 { '.' } else { '#' },
                if (self.bits & 0b_1000) == 0 { '.' } else { '#' }
            )
            .as_str(),
        )
    }
}

/// A 3x3 tile represented as bits. Example: .#./..#/###" is stored as 0b_111_100_010
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Tile3 {
    bits: u16,
}

impl Tile3 {
    fn from(input: &str) -> Self {
        let bytes = input.as_bytes();
        let bits = (0..9).fold(0_u16, |acc, bit_offset| {
            let byte_idx = bit_offset + bit_offset / 3;
            acc + if bytes[byte_idx] == b'#' {
                1 << bit_offset
            } else {
                0
            }
        });
        Self { bits }
    }

    fn rotate(self) -> Self {
        #![allow(clippy::unusual_byte_groupings)]
        // ABC      GDA
        // DEF  =>  HEB
        // GHI      IFC
        // First bit   (A): Shift higher 2
        // Second bit  (B): Shift higher 4
        // Third bit   (C): Shift higher 6
        // Fourth bit  (D): Shift lower 2
        // Fifth bit   (E): Unmodified
        // Sixth bit   (F): Shift higher 2
        // Seventh bit (G): Shift lower 6
        // Eight bit   (H): Shift lower 4
        // Ninth bit   (I): Shift lower 2
        Self {
            bits: ((self.bits & 0b_000_000_001) << 2)  // A
                + ((self.bits & 0b_000_000_010) << 4)  // B
                + ((self.bits & 0b_000_000_100) << 6)  // C
                + ((self.bits & 0b_000_001_000) >> 2)  // D
                + (self.bits & 0b_000_010_000)         // E
                + ((self.bits & 0b_000_100_000) << 2)  // F
                + ((self.bits & 0b_001_000_000) >> 6)  // G
                + ((self.bits & 0b_010_000_000) >> 4)  // H
                + ((self.bits & 0b_100_000_000) >> 2), // I
        }
    }

    fn flip(self) -> Self {
        #![allow(clippy::unusual_byte_groupings)]
        // ABC      CBA
        // DEF  =>  FED
        // GHI      IHG
        Self {
            bits: ((self.bits & 0b_000_000_001) << 2)
                + (self.bits & 0b_000_000_010)
                + ((self.bits & 0b_000_000_100) >> 2)
                + ((self.bits & 0b_000_001_000) << 2)
                + (self.bits & 0b_000_010_000)
                + ((self.bits & 0b_000_100_000) >> 2)
                + ((self.bits & 0b_001_000_000) << 2)
                + (self.bits & 0b_010_000_000)
                + ((self.bits & 0b_100_000_000) >> 2),
        }
    }
}

impl fmt::Debug for Tile3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            format!(
                "{}{}{}/{}{}{}/{}{}{}",
                if (self.bits & 0b_000000001) == 0 {
                    '.'
                } else {
                    '#'
                },
                if (self.bits & 0b_000000010) == 0 {
                    '.'
                } else {
                    '#'
                },
                if (self.bits & 0b_000000100) == 0 {
                    '.'
                } else {
                    '#'
                },
                if (self.bits & 0b_000001000) == 0 {
                    '.'
                } else {
                    '#'
                },
                if (self.bits & 0b_000010000) == 0 {
                    '.'
                } else {
                    '#'
                },
                if (self.bits & 0b_000100000) == 0 {
                    '.'
                } else {
                    '#'
                },
                if (self.bits & 0b_001000000) == 0 {
                    '.'
                } else {
                    '#'
                },
                if (self.bits & 0b_010000000) == 0 {
                    '.'
                } else {
                    '#'
                },
                if (self.bits & 0b_100000000) == 0 {
                    '.'
                } else {
                    '#'
                }
            )
            .as_str(),
        )
    }
}

/// A 4x4 tile represented as bits.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Tile4 {
    bits: u16,
}

impl Tile4 {
    fn from(input: &str) -> Self {
        let bytes = input.as_bytes();
        let bits = (0..16).fold(0_u16, |acc, bit_offset| {
            let byte_idx = bit_offset + bit_offset / 4;
            acc + if bytes[byte_idx] == b'#' {
                1 << bit_offset
            } else {
                0
            }
        });
        Self { bits }
    }

    fn divided_up(self) -> [Tile2; 4] {
        [
            Tile2 {
                // XX00
                // XX00
                // 0000
                // 0000
                bits: ((self.bits & 0b_0000_0000_0000_0011)
                    + ((self.bits & 0b_0000_0000_0011_0000) >> 2)) as u8,
            },
            Tile2 {
                // 00XX
                // 00XX
                // 0000
                // 0000
                bits: (((self.bits & 0b_0000_0000_0000_1100) >> 2)
                    + ((self.bits & 0b_0000_0000_1100_0000) >> 4)) as u8,
            },
            Tile2 {
                // 0000
                // 0000
                // XX00
                // XX00
                bits: (((self.bits & 0b_0000_0011_0000_0000) >> 8)
                    + ((self.bits & 0b_0011_0000_0000_0000) >> 10)) as u8,
            },
            Tile2 {
                // 0000
                // 0000
                // 00XX
                // 00XX
                bits: (((self.bits & 0b_0000_1100_0000_0000) >> 10)
                    + ((self.bits & 0b_1100_0000_0000_0000) >> 12)) as u8,
            },
        ]
    }
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut from_2_to_3 = HashMap::new();
    let mut from_3_to_4 = HashMap::new();

    for (line_idx, line) in input.text.lines().enumerate() {
        let on_error = || format!("Line {}: Invalid format", line_idx + 1);

        let mut parts = line.splitn(2, " => ");
        let from = parts.next().ok_or_else(on_error)?;
        let to = parts.next().ok_or_else(on_error)?;

        match (from.len(), to.len()) {
            (5, 11) => {
                // ../.. => #../##./...
                let from = Tile2::from(from);
                let to = Tile3::from(to);
                from_2_to_3.insert(from, to);
                from_2_to_3.insert(from.rotate(), to);
                from_2_to_3.insert(from.rotate().rotate(), to);
                from_2_to_3.insert(from.rotate().rotate().rotate(), to);
                from_2_to_3.insert(from.flip(), to);
                from_2_to_3.insert(from.flip().rotate(), to);
                from_2_to_3.insert(from.flip().rotate().rotate(), to);
                from_2_to_3.insert(from.flip().rotate().rotate().rotate(), to);
            }
            (11, 19) => {
                // .../.../... => ##../#.../..../..#.
                let from = Tile3::from(from);
                let to = Tile4::from(to);
                from_3_to_4.insert(from, to);
                from_3_to_4.insert(from.rotate(), to);
                from_3_to_4.insert(from.rotate().rotate(), to);
                from_3_to_4.insert(from.rotate().rotate().rotate(), to);
                from_3_to_4.insert(from.flip(), to);
                from_3_to_4.insert(from.flip().rotate(), to);
                from_3_to_4.insert(from.flip().rotate().rotate(), to);
                from_3_to_4.insert(from.flip().rotate().rotate().rotate(), to);
            }
            _ => {
                return Err(on_error());
            }
        }
    }

    let initial_tile = Tile3::from(".#./..#/###");
    let mut current_3_tiles = vec![initial_tile];
    let mut current_2_tiles: Vec<Tile2> = Vec::new();

    for iteration in 0..5 {
        if iteration % 2 == 0 {
            current_2_tiles = current_3_tiles
                .iter()
                .inspect(|tile| println!("Iteration {} - going from {:?}", iteration, tile))
                .map(|tile| from_3_to_4.get(tile).unwrap().divided_up())
                .collect::<Vec<[Tile2; 4]>>()
                .iter()
                .inspect(|tile| println!("... to {:?}", tile))
                .flat_map(|tile| tile.iter().copied())
                .collect();
        } else {
            current_3_tiles = current_2_tiles
                .iter()
                .map(|tile| {
                    let tile_3 = from_2_to_3.get(tile);
                    println!(
                        "iteration: {} - going from {:?} to {:?}",
                        iteration,
                        tile,
                        tile_3.unwrap()
                    );
                    *tile_3.unwrap()
                })
                .collect();
        }
    }

    Ok(current_2_tiles
        .iter()
        .map(|tile| tile.bits.count_ones())
        .sum())
}

#[test]
pub fn tile_tests() {
    #![allow(clippy::unusual_byte_groupings)]
    let tile = Tile2::from("##/.#");
    assert_eq!(0b_10_11, tile.bits);
    assert_eq!(tile.bits, tile.rotate().rotate().rotate().rotate().bits);
    assert_eq!(tile.bits, tile.flip().flip().bits);
    // ##   =>   .#
    // .#        ##
    let rotated_tile = tile.rotate();
    assert_eq!(Tile2::from(".#/##").bits, rotated_tile.bits);

    let flipped_tile = tile.flip();
    assert_eq!(Tile2::from("##/#.").bits, flipped_tile.bits);

    let tile = Tile3::from("##./.#./##.");
    assert_eq!(0b_011_010_011, tile.bits);
    assert_eq!(tile.bits, tile.rotate().rotate().rotate().rotate().bits);
    assert_eq!(tile.bits, tile.flip().flip().bits);
    // ##.        #.#
    // .#.   =>   ###
    // ##.        ...
    let rotated_tile = tile.rotate();
    assert_eq!(Tile3::from("#.#/###/...").bits, rotated_tile.bits);

    let flipped_tile = tile.flip();
    assert_eq!(Tile3::from(".##/.#./.##").bits, flipped_tile.bits);

    let tile = Tile4::from("####/.###/..../#.#.");
    assert_eq!(0b_0101_0000_1110_1111, tile.bits);

    let tile = Tile4::from("#..#/..../..../#..#");
    assert_eq!(0b_1001_0000_0000_1001, tile.bits);
    // #.|.#
    // ..|..
    // --+--
    // ..|..
    // #.|.#
    assert_eq!(
        tile.divided_up(),
        [
            Tile2::from("#./.."),
            Tile2::from(".#/.."),
            Tile2::from("../#."),
            Tile2::from("../.#")
        ]
    );
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day21_input.txt");
    test_part_one!(real_input => 0);
    //test_part_two!(real_input => 0);
}
