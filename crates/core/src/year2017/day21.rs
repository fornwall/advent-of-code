use crate::input::Input;
use std::collections::HashMap;

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
        Self {
            bits: ((self.bits & 0b1110) >> 1) + if (self.bits & 1) == 0 { 0 } else { 0b_10_00 },
        }
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
        Self {
            bits: (self.bits >> 1)
                + if (self.bits & 1) == 0 {
                    0
                } else {
                    0b_100_000_000
                },
        }
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

    fn rotate(self) -> Self {
        Self {
            bits: (self.bits >> 1)
                + if (self.bits & 1) == 0 {
                    0
                } else {
                    0b_1000_0000_0000
                },
        }
    }
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let initial_tile = Tile3::from(".#./..#/###");

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
            }
            (11, 19) => {
                // .../.../... => ##../#.../..../..#.
                let from = Tile3::from(from);
                let to = Tile4::from(to);
                from_3_to_4.insert(from, to);
            }
            _ => {
                return Err(on_error());
            }
        }
    }
    Ok(0)
}

#[test]
pub fn tile_tests() {
    #![allow(clippy::unusual_byte_groupings)]
    let tile = Tile2::from("##/.#");
    assert_eq!(0b_10_11, tile.bits);
    let rotated_tile = tile.rotate();
    assert_eq!(Tile2::from(".#/##").bits, tile.bits);

    let tile = Tile3::from("##./.#./##.");
    assert_eq!(0b_011_010_011, tile.bits);

    let tile = Tile4::from("####/.###/..../#.#.");
    assert_eq!(0b_0101_0000_1110_1111, tile.bits);
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day21_input.txt");
    test_part_one!(real_input => 0);
    test_part_two!(real_input => 0);
}
