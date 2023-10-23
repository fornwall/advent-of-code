/// Solution to Advent of Code, day 23 in 2022.
/// Using portable simd in rust.
/// Based on <https://github.com/Crazytieguy/advent-of-code/blob/master/2022/src/bin/day23/main.rs/>
use std::array;
use std::collections::VecDeque;
use std::simd::u8x32;

use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    let mut grid = ElfGrid::parse(input.text)?;

    if input.is_part_one() {
        grid.run_simulation(10);
        Ok(grid.populated_rect_size() - grid.num_elves())
    } else {
        grid.run_simulation(10000)
            .ok_or_else(|| "No solution found in 10,000 rounds".to_string())
    }
}

/// Each row is represented as a bitset.
/// - Bit 0 in lane 0 is for col 0
/// - [...]
/// - Bit 1 in lane 0 is for col 1
/// - Bit 7 in lane 0 is for col 7
/// - Bit 0 in lane 1 is for col 8
/// - [...]
type ElfGridRow = u8x32;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

struct ElfGrid {
    bit_rows: [ElfGridRow; Self::NUM_ROWS],
}

impl ElfGrid {
    // Values big enough to solve official advent of code inputs:
    const NUM_ROWS: usize = 160;
    const NUM_COLS: usize = 256;
    const ROW_OFFSET: usize = 24;
    const COL_OFFSET: usize = 72;

    fn parse(input: &str) -> Result<Self, String> {
        let mut grid = Self {
            bit_rows: [ElfGridRow::splat(0); Self::NUM_ROWS],
        };
        for (row, line) in input.lines().enumerate() {
            for (col, b) in line.bytes().enumerate() {
                if b == b'#' {
                    let storage_row = row + Self::ROW_OFFSET;
                    let storage_col = col + Self::COL_OFFSET;
                    if storage_row >= Self::NUM_ROWS || storage_col >= Self::NUM_COLS {
                        return Err("Elves does not fit into optimised grid".to_string());
                    }
                    grid.set_elf_at(storage_row, storage_col);
                }
            }
        }
        Ok(grid)
    }

    fn set_elf_at(&mut self, row: usize, col: usize) {
        self.bit_rows[row][col / 8] |= 1 << (col % 8);
    }

    fn is_elf_at(&self, row: usize, col: usize) -> bool {
        self.bit_rows[row][col / 8] & (1 << (col % 8)) != 0
    }

    fn shift_cols_west(&row: &ElfGridRow) -> ElfGridRow {
        // Shift cols to the west/left (to _lower_ values).
        // Start with each lane shifted right (so lowest bit is lost):
        // [abcd, efgh, ijkl] -> [0abc, 0efg, 0ijk]
        (row >> ElfGridRow::splat(1))
            // Bitwise OR with the lowest bit shifted to highest, with rotated lanes.
            // [abcd, efgh, ijkl] -> [h000, l000, d000]
            | (row.rotate_lanes_left::<1>() << ElfGridRow::splat(7))
    }

    fn shift_cols_east(&row: &ElfGridRow) -> ElfGridRow {
        // Shift cols to the east/right (to _higher_ values).
        // Start with each lane shifted left (so highest bit is lost):
        // [abcd, efgh, ijkl] -> [bcd0, fgh0, jkl0]
        (row << ElfGridRow::splat(1))
            // Bitwise OR with the highest bit shifted to lowest, with rotated lanes.
            // [abcd, efgh, ijkl] -> [000l, 000a, 000e]
            | (row.rotate_lanes_right::<1>() >> ElfGridRow::splat(7))
    }

    fn run_simulation(&mut self, max_rounds: usize) -> Option<usize> {
        let mut directions = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        for round in 0..max_rounds {
            if !self.play_round(directions) {
                return Some(round + 1);
            }
            directions.rotate_left(1);
        }
        None
    }

    #[allow(unstable_name_collisions)]
    fn play_round(&mut self, directions: [Direction; 4]) -> bool {
        // Given 9 rows:
        // - The above row:   [shifted-east, unmodified, shifted-west]
        // - The current row: [shifted-east, unmodified, shifted-west]
        // - The below row:   [shifted-east, unmodified, shifted-west]
        //
        // This function computes an array, indexed by direction (N=1, S=2, W=3, E=4)
        // mapping to a bitset of those elves who will be moving in the specified direction
        // (not taking collisions into account).
        //
        // Note that e.g. "the above row, shifted-east" is actually "nw" in the function signature.
        // That is since it contains the bits shifted east, so shows the population to the west.
        fn propose_movements(
            [nw, n, ne]: &[ElfGridRow; 3],
            [w, cur, e]: &[ElfGridRow; 3],
            [sw, s, se]: &[ElfGridRow; 3],
            ordered_directions: [Direction; 4],
        ) -> [ElfGridRow; 4] {
            let mut propositions = [*cur; 4];

            // Keep track of elves who can still move. Start by require adjacent elf:
            // "During the first half of each round, each Elf considers the eight
            // positions adjacent to themself. If no other Elves are in one of those
            // eight positions, the Elf does not do anything during this round":
            let mut available_to_move = nw | n | ne | w | e | sw | s | se;

            for direction in ordered_directions {
                let direction_occupied = match direction {
                    // "If there is no Elf in the N, NE, or NW adjacent positions,
                    // the Elf proposes moving north one step"
                    Direction::North => ne | n | nw,
                    // "If there is no Elf in the S, SE, or SW adjacent positions,
                    // the Elf proposes moving south one step"
                    Direction::South => se | s | sw,
                    // "If there is no Elf in the W, NW, or SW adjacent positions,
                    // the Elf proposes moving west one step"
                    Direction::West => nw | w | sw,
                    // "If there is no Elf in the E, NE, or SE adjacent positions,
                    // the Elf proposes moving east one step"
                    Direction::East => ne | e | se,
                };

                // Move the elf if the three adjacent positions in the direction
                // are unoccupied, and elf have not already moved in another direction:
                propositions[direction as usize] &= !direction_occupied & available_to_move;

                // Clear elves who have already moved:
                available_to_move &= direction_occupied;
            }
            propositions
        }

        // Given 3 results from bitset_per_direction_excluding_collisions() above:
        // - How the above row would move   [N, S, W, E]
        // - How the current row would move [N, S, W, E]
        // - How the below row would move   [N, S, W, E],
        // if there were no collisions, this function computes how the current row
        // will be populated by elves moving from the north, south, weast and east.
        //
        // Output format is an array, indexed by direction (N=1, S=2, W=3, E=4)
        // mapping to a bitset of those elves who will be coming from the specified
        // direction - now taking collisions into account.
        fn collide_proposals(
            [_, south, _, _]: &[ElfGridRow; 4],
            [_, _, west, east]: &[ElfGridRow; 4],
            [north, _, _, _]: &[ElfGridRow; 4],
        ) -> [ElfGridRow; 4] {
            [
                north & !*south,
                south & !*north,
                ElfGrid::shift_cols_west(west) & !ElfGrid::shift_cols_east(east),
                ElfGrid::shift_cols_east(east) & !ElfGrid::shift_cols_west(west),
            ]
        }

        let mut new_bit_rows = self.bit_rows;
        let mut moved = false;

        self.bit_rows
            .iter()
            .map(|row| [Self::shift_cols_east(row), *row, Self::shift_cols_west(row)])
            .map_windows(|[above, cur, below]| propose_movements(above, cur, below, directions))
            .map_windows(|[above, cur, below]| collide_proposals(above, cur, below))
            .enumerate()
            .for_each(
                |(row_idx, [from_south, from_north, from_east, from_west])| {
                    // Offset two for the two uses of map_windows() with an array size of 3:
                    let row_idx = row_idx + 2;
                    let destinations = from_north | from_south | from_west | from_east;
                    if destinations != ElfGridRow::splat(0) {
                        moved = true;
                        new_bit_rows[row_idx + 1] &= !from_south;
                        new_bit_rows[row_idx - 1] &= !from_north;
                        new_bit_rows[row_idx] &= !Self::shift_cols_west(&from_west);
                        new_bit_rows[row_idx] &= !Self::shift_cols_east(&from_east);
                        new_bit_rows[row_idx] |= destinations;
                    }
                },
            );

        self.bit_rows = new_bit_rows;
        moved
    }

    fn populated_rect_size(&self) -> usize {
        let bounds = (0..self.bit_rows.len())
            .flat_map(|row| (0..Self::NUM_COLS).map(move |col| (row, col)))
            .fold(
                (usize::MAX, usize::MIN, usize::MAX, usize::MIN),
                |acc, (row, col)| {
                    if self.is_elf_at(row, col) {
                        (
                            acc.0.min(row),
                            acc.1.max(row),
                            acc.2.min(col),
                            acc.3.max(col),
                        )
                    } else {
                        acc
                    }
                },
            );
        (bounds.1 + 1 - bounds.0) * (bounds.3 + 1 - bounds.2)
    }

    fn num_elves(&self) -> usize {
        self.bit_rows
            .iter()
            .flat_map(std::simd::Simd::as_array)
            .map(|x| x.count_ones() as usize)
            .sum()
    }
}

struct MapWindows<I: Iterator, F, T, const N: usize>
where
    F: FnMut([&I::Item; N]) -> T,
{
    iter: I,
    f: F,
    buf: VecDeque<I::Item>,
}

impl<I: Iterator, F, T, const N: usize> MapWindows<I, F, T, N>
where
    F: FnMut([&I::Item; N]) -> T,
{
    fn new(mut iter: I, f: F) -> Self {
        let buf: VecDeque<_> = iter.by_ref().take(N - 1).collect();
        Self { iter, f, buf }
    }
}

impl<I: Iterator, F, T, const N: usize> Iterator for MapWindows<I, F, T, N>
where
    F: FnMut([&I::Item; N]) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|next| {
            self.buf.push_back(next);
            let res = (self.f)(array::from_fn(|i| &self.buf[i]));
            self.buf.pop_front();
            res
        })
    }
}

trait MapWindowsIterator: Iterator {
    fn map_windows<T, F, const N: usize>(self, f: F) -> MapWindows<Self, F, T, N>
    where
        Self: Sized,
        F: FnMut([&Self::Item; N]) -> T,
    {
        MapWindows::new(self, f)
    }
}

impl<I: Iterator> MapWindowsIterator for I {}

#[test]
#[allow(unstable_name_collisions)]
fn test_iterator() {
    let v = [1, 2, 3, 4]
        .iter()
        .map_windows(|[a, b]| (**a, **b))
        .collect::<Vec<_>>();
    assert_eq!(vec![(1, 2), (2, 3), (3, 4)], v);

    let v = [1, 2, 3, 4]
        .iter()
        .map_windows(|[a, b, c]| (**a, **b, **c))
        .collect::<Vec<_>>();
    assert_eq!(vec![(1, 2, 3), (2, 3, 4)], v);
    let v = [1, 2, 3, 4]
        .iter()
        .map_windows(|[a, b, c, d, e]| (**a, **b, **c, **d, **e))
        .next();
    assert_eq!(None, v);
    let v = [1, 2, 3]
        .iter()
        .map_windows(|[a, b, c, d, e]| (**a, **b, **c, **d, **e))
        .next();
    assert_eq!(None, v);
}
