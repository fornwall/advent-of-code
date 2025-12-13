#[cfg(not(feature = "simd"))]
#[cfg(not(feature = "webgpu-compute"))]
#[cfg(not(feature = "visualization"))]
pub fn solve(input: &crate::input::Input) -> Result<usize, String> {
    use crate::common::map_windows::MapWindowsIterator;
    use crate::common::u256::U256;

    /// Each row is represented as a bitset.
    type ElfGridRow = U256;

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
                bit_rows: [ElfGridRow::default(); Self::NUM_ROWS],
            };
            for (row, line) in input.lines().enumerate() {
                for (col, b) in line.bytes().enumerate() {
                    if b == b'#' {
                        let storage_row = row + Self::ROW_OFFSET;
                        let storage_col = col + Self::COL_OFFSET;
                        if storage_row >= Self::NUM_ROWS || storage_col >= Self::NUM_COLS {
                            return Err("Elves does not fit into optimised grid".to_string());
                        }
                        grid.bit_rows[storage_row].set_bit(storage_col);
                    }
                }
            }
            Ok(grid)
        }

        const fn shift_cols_west(&row: &ElfGridRow) -> ElfGridRow {
            // Shift cols to the west/left (to _lower_ values).
            row.shift_right(255)
        }

        const fn shift_cols_east(&row: &ElfGridRow) -> ElfGridRow {
            // Shift cols to the east/right (to _higher_ values).
            row.shift_left(255)
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
                let mut available_to_move = *nw | *n | *ne | *w | *e | *sw | *s | *se;

                for direction in ordered_directions {
                    let direction_occupied = match direction {
                        // "If there is no Elf in the N, NE, or NW adjacent positions,
                        // the Elf proposes moving north one step"
                        Direction::North => *ne | *n | *nw,
                        // "If there is no Elf in the S, SE, or SW adjacent positions,
                        // the Elf proposes moving south one step"
                        Direction::South => *se | *s | *sw,
                        // "If there is no Elf in the W, NW, or SW adjacent positions,
                        // the Elf proposes moving west one step"
                        Direction::West => *nw | *w | *sw,
                        // "If there is no Elf in the E, NE, or SE adjacent positions,
                        // the Elf proposes moving east one step"
                        Direction::East => *ne | *e | *se,
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
                    *north & !*south,
                    *south & !*north,
                    ElfGrid::shift_cols_west(west) & !ElfGrid::shift_cols_east(east),
                    ElfGrid::shift_cols_east(east) & !ElfGrid::shift_cols_west(west),
                ]
            }

            let mut new_bit_rows = self.bit_rows;
            let mut moved = false;

            self.bit_rows
                .iter()
                .map(|row| [Self::shift_cols_east(row), *row, Self::shift_cols_west(row)])
                .map_windows_stable(|[above, cur, below]| {
                    propose_movements(above, cur, below, directions)
                })
                .map_windows_stable(|[above, cur, below]| collide_proposals(above, cur, below))
                .enumerate()
                .for_each(
                    |(row_idx, [from_south, from_north, from_east, from_west])| {
                        // Offset two for the two uses of map_windows() with an array size of 3:
                        let row_idx = row_idx + 2;
                        let destinations = from_north | from_south | from_west | from_east;
                        if destinations != ElfGridRow::default() {
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
                        if self.bit_rows[row].is_bit_set(col) {
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
            self.bit_rows.iter().map(|x| x.count_ones() as usize).sum()
        }
    }

    let mut grid = ElfGrid::parse(input.text)?;

    if input.is_part_one() {
        grid.run_simulation(10);
        Ok(grid.populated_rect_size() - grid.num_elves())
    } else {
        grid.run_simulation(10000)
            .ok_or_else(|| "No solution found in 10,000 rounds".to_string())
    }
}

#[cfg(feature = "simd")]
pub use super::day23_simd::solve;

#[cfg(feature = "webgpu-compute")]
pub use super::day23_webgpu::solve;

#[cfg(feature = "visualization")]
pub use super::day23_renderer::solve;

#[test]
pub fn tests() {
    use crate::input::Input;

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
