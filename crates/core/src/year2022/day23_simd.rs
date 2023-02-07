/// Solutions to Advent of Code, day 23 in 2022.
/// Using portable simd in rust.
/// Taken from https://github.com/Crazytieguy/advent-of-code/blob/master/2022/src/bin/day23/main.rs
use std::array;
use std::collections::VecDeque;
use std::ops::Range;
use std::simd::u8x32;
use crate::chain;

use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    let mut grid = ElfGrid::new();
    input.text.lines().enumerate().for_each(|(row, line)| {
        for (col, b) in line.bytes().enumerate() {
            if b == b'#' {
                grid.set_elf_at(row + ELF_GRID_ROW_OFFSET, col + ELF_GRID_COL_OFFSET);
            }
        }
    });

    if input.is_part_one() {
        grid.run_simulation(10);
        let (rows, cols) = grid.bounds();
        Ok(rows.len() * cols.len() - grid.num_elves())
    } else {
        grid.run_simulation(10000)
            .ok_or_else(|| "No solution found in 10,000 rounds".to_string())
    }
}

type ElfGridRow = u8x32;
const ELF_GRID_NUM_ROWS: usize = 160;
const ELF_GRID_ROW_OFFSET: usize = 24;
const ELF_GRID_COL_OFFSET: usize = 72;

struct ElfGrid {
    bit_rows: [ElfGridRow; ELF_GRID_NUM_ROWS],
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl ElfGrid {
    fn new() -> Self {
        Self {
            bit_rows: [ElfGridRow::splat(0); ELF_GRID_NUM_ROWS],
        }
    }

    fn set_elf_at(&mut self, row: usize, col: usize) {
        self.bit_rows[row][col / 8] |= 1 << (col % 8);
    }

    fn is_elf_at(&self, row: usize, col: usize) -> bool {
        self.bit_rows[row][col / 8] & (1 << (col % 8)) != 0
    }

    fn shift_west(&row: &ElfGridRow) -> ElfGridRow {
        (row >> ElfGridRow::splat(1)) | (row.rotate_lanes_left::<1>() << ElfGridRow::splat(7))
    }

    fn shift_east(&row: &ElfGridRow) -> ElfGridRow {
        (row << ElfGridRow::splat(1)) | (row.rotate_lanes_right::<1>() >> ElfGridRow::splat(7))
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

    fn play_round(&mut self, priority: [Direction; 4]) -> bool {
        fn propose(
            [nw, n, ne]: &[ElfGridRow; 3],
            [w, cur, e]: &[ElfGridRow; 3],
            [sw, s, se]: &[ElfGridRow; 3],
            directions: [Direction; 4],
        ) -> [ElfGridRow; 4] {
            let mut propositions = [*cur; 4];
            let mut not_chosen = nw | n | ne | w | e | sw | s | se;
            for d in directions {
                let (row, dir_available) = match d {
                    Direction::North => (&mut propositions[0], !(ne | n | nw)),
                    Direction::South => (&mut propositions[1], !(se | s | sw)),
                    Direction::West => (&mut propositions[2], !(nw | w | sw)),
                    Direction::East => (&mut propositions[3], !(ne | e | se)),
                };
                *row &= dir_available & not_chosen;
                not_chosen &= !dir_available;
            }
            propositions
        }

        fn collide_proposals(
            [_, south, _, _]: &[ElfGridRow; 4],
            [_, _, west, east]: &[ElfGridRow; 4],
            [north, _, _, _]: &[ElfGridRow; 4],
        ) -> [ElfGridRow; 4] {
            [
                north & !*south,
                south & !*north,
                ElfGrid::shift_west(west) & !ElfGrid::shift_east(east),
                ElfGrid::shift_east(east) & !ElfGrid::shift_west(west),
            ]
        }

        let mut new_bit_rows = self.bit_rows;
        let mut moved = false;
        let empty_row = [ElfGridRow::splat(0); 2];

        chain!(&empty_row, &self.bit_rows, &empty_row)
            .map(|row| [Self::shift_east(row), *row, Self::shift_west(row)])
            .map_windows(|[above, cur, below]| propose(above, cur, below, priority))
            .map_windows(|[above, cur, below]| collide_proposals(above, cur, below))
            .enumerate()
            .for_each(|(i, [from_south, from_north, from_east, from_west])| {
                let destinations = from_north | from_south | from_west | from_east;
                if destinations != ElfGridRow::splat(0) {
                    moved = true;
                    new_bit_rows[i + 1] &= !from_south;
                    new_bit_rows[i - 1] &= !from_north;
                    new_bit_rows[i] &= !Self::shift_west(&from_west);
                    new_bit_rows[i] &= !Self::shift_east(&from_east);
                    new_bit_rows[i] |= destinations;
                }
            });

        self.bit_rows = new_bit_rows;
        moved
    }

    fn bounds(&self) -> (Range<usize>, Range<usize>) {
        let mut min_row = usize::MAX;
        let mut max_row = usize::MIN;
        let mut min_col = usize::MAX;
        let mut max_col = usize::MIN;
        for row in 0..self.bit_rows.len() {
            for col in 0..256 {
                if self.is_elf_at(row, col) {
                    min_row = min_row.min(row);
                    max_row = max_row.max(row);
                    min_col = min_col.min(col);
                    max_col = max_col.max(col);
                }
            }
        }
        (min_row..max_row + 1, min_col..max_col + 1)
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
        let buf = iter.by_ref().take(N - 1).collect();
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
