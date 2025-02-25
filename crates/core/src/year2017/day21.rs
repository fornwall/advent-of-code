use crate::input::Input;

/// A 2x2 tile represented as bits. Example: "../.#" is stored as `0b_10_00`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Tile2 {
    bits: u8,
}

impl Tile2 {
    fn from(input: &str) -> Self {
        assert_eq!(5, input.len());
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

    const fn rotate(self) -> Self {
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

    const fn flip(self) -> Self {
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

/// A 3x3 tile represented as bits. Example: .#./..#/###" is stored as `0b_111_100_010`
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Tile3 {
    bits: u16,
}

impl Tile3 {
    fn from(input: &str) -> Self {
        assert_eq!(11, input.len());
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

    const fn rotate(self) -> Self {
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

    const fn flip(self) -> Self {
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

/// A 4x4 tile represented as bits.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Tile4 {
    bits: u16,
}

impl Tile4 {
    fn from(input: &str) -> Self {
        assert_eq!(19, input.len());
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

    const fn divided_up(self) -> [Tile2; 4] {
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

pub fn solve(input: &Input) -> Result<u32, String> {
    #![allow(clippy::unusual_byte_groupings, clippy::unreadable_literal)]
    let mut from_2_to_3 = [Tile3 { bits: 0 }; 16];
    let mut from_3_to_4 = [Tile4 { bits: 0 }; 512];

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
                from_2_to_3[usize::from(from.bits)] = to;
                from_2_to_3[usize::from(from.rotate().bits)] = to;
                from_2_to_3[usize::from(from.rotate().rotate().bits)] = to;
                from_2_to_3[usize::from(from.rotate().rotate().rotate().bits)] = to;
                from_2_to_3[usize::from(from.flip().bits)] = to;
                from_2_to_3[usize::from(from.flip().rotate().bits)] = to;
                from_2_to_3[usize::from(from.flip().rotate().rotate().bits)] = to;
                from_2_to_3[usize::from(from.flip().rotate().rotate().rotate().bits)] = to;
            }
            (11, 19) => {
                // .../.../... => ##../#.../..../..#.
                let from = Tile3::from(from);
                let to = Tile4::from(to);
                from_3_to_4[usize::from(from.bits)] = to;
                from_3_to_4[usize::from(from.rotate().bits)] = to;
                from_3_to_4[usize::from(from.rotate().rotate().bits)] = to;
                from_3_to_4[usize::from(from.rotate().rotate().rotate().bits)] = to;
                from_3_to_4[usize::from(from.flip().bits)] = to;
                from_3_to_4[usize::from(from.flip().rotate().bits)] = to;
                from_3_to_4[usize::from(from.flip().rotate().rotate().bits)] = to;
                from_3_to_4[usize::from(from.flip().rotate().rotate().rotate().bits)] = to;
            }
            _ => {
                return Err(on_error());
            }
        }
    }

    let initial_tile = Tile3::from(".#./..#/###");
    let mut current_2_tiles: Vec<Tile2> = Vec::new();
    let mut current_3_tiles = vec![initial_tile];
    let mut current_4_tiles: Vec<Tile4> = Vec::new();

    let num_iterations = input.part_values(5, 18);

    for iteration in 0..num_iterations {
        match iteration % 3 {
            0 => {
                // Map each 3x3 matrix to a 4x4 one:
                current_4_tiles = current_3_tiles
                    .iter()
                    .map(|&tile| from_3_to_4[usize::from(tile.bits)])
                    .collect();
            }
            1 => {
                current_2_tiles.clear();

                for tile4 in &current_4_tiles {
                    // Split up a 4x4 matrix into four 2x2 ones:
                    let tile2_parts = tile4.divided_up();
                    // Map each 2x2 matrix to the resulting 3x3 matrix:
                    let tile3_parts: Vec<Tile3> = tile2_parts
                        .iter()
                        .map(|&tile| from_2_to_3[usize::from(tile.bits)])
                        .collect();

                    // From the four 3x3 matrices, build the resulting nine 2x2 ones:
                    //
                    // AAABBB
                    // AAABBB
                    // AAABBB
                    // CCCDDD
                    // CCCDDD
                    // CCCDDD

                    // Upper row of 2x2:
                    current_2_tiles.push(Tile2 {
                        bits: ((tile3_parts[0].bits & 0b11)
                            | ((tile3_parts[0].bits & 0b11000) >> 1))
                            as u8,
                    });
                    current_2_tiles.push(Tile2 {
                        bits: (((tile3_parts[0].bits & 0b100) >> 2)
                            | ((tile3_parts[1].bits & 0b1) << 1)
                            | ((tile3_parts[0].bits & 0b100000) >> 3)
                            | (tile3_parts[1].bits & 0b1000)) as u8,
                    });
                    current_2_tiles.push(Tile2 {
                        bits: (((tile3_parts[1].bits & 0b110) >> 1)
                            | ((tile3_parts[1].bits & 0b110000) >> 2))
                            as u8,
                    });

                    // Middle row of 2x2:
                    current_2_tiles.push(Tile2 {
                        bits: (((tile3_parts[0].bits & 0b11000000) >> 6)
                            | ((tile3_parts[2].bits & 0b11) << 2))
                            as u8,
                    });
                    current_2_tiles.push(Tile2 {
                        bits: (((tile3_parts[0].bits & 0b100000000) >> 8)
                            | ((tile3_parts[1].bits & 0b1000000) >> 5)
                            | (tile3_parts[2].bits & 0b100)
                            | ((tile3_parts[3].bits & 0b1) << 3))
                            as u8,
                    });
                    current_2_tiles.push(Tile2 {
                        bits: (((tile3_parts[1].bits & 0b110000000) >> 7)
                            | ((tile3_parts[3].bits & 0b110) << 1))
                            as u8,
                    });

                    // Bottom row of 2x2:
                    current_2_tiles.push(Tile2 {
                        bits: (((tile3_parts[2].bits & 0b11000) >> 3)
                            | ((tile3_parts[2].bits & 0b11000000) >> 4))
                            as u8,
                    });
                    current_2_tiles.push(Tile2 {
                        bits: (((tile3_parts[2].bits & 0b100000) >> 5)
                            | ((tile3_parts[3].bits & 0b1000) >> 2)
                            | ((tile3_parts[2].bits & 0b100000000) >> 6)
                            | ((tile3_parts[3].bits & 0b1000000) >> 3))
                            as u8,
                    });
                    current_2_tiles.push(Tile2 {
                        bits: (((tile3_parts[3].bits & 0b110000) >> 4)
                            | ((tile3_parts[3].bits & 0b110000000) >> 5))
                            as u8,
                    });
                }
            }
            2 => {
                // Map each 2x2 matrix to a 3x3 one:
                current_3_tiles = current_2_tiles
                    .iter()
                    .map(|tile| from_2_to_3[usize::from(tile.bits)])
                    .collect();
            }
            _ => {
                return Err("Internal error".to_string());
            }
        }
    }

    Ok(if input.is_part_one() {
        current_2_tiles
            .iter()
            .map(|tile| tile.bits.count_ones())
            .sum()
    } else {
        current_3_tiles
            .iter()
            .map(|tile| tile.bits.count_ones())
            .sum()
    })
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

    let tile = Tile2::from("##/##");
    assert_eq!(0b_11_11, tile.bits);
    assert_eq!(tile.bits, tile.rotate().rotate().rotate().rotate().bits);
    assert_eq!(tile.bits, tile.flip().flip().bits);

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
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day21_input.txt");
    test_part_one!(real_input => 142);
    test_part_two!(real_input => 1_879_071);
}
