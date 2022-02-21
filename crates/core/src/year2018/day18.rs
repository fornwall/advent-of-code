use crate::input::Input;
use std::collections::hash_map::{DefaultHasher, Entry};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem::swap;

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<u8>,
    next_gen_cells: Vec<u8>,
}

impl Grid {
    fn parse(input_string: &str) -> Result<Self, String> {
        let mut height = 0;
        let mut width = 0;
        let mut cells = Vec::new();
        for (line_idx, line) in input_string.lines().enumerate() {
            let new_width = line.len();
            if line_idx == 0 {
                width = new_width;
            } else if new_width != width {
                return Err("Not all lines have equal length".into());
            }
            line.chars().for_each(|c| cells.push(c as u8));
            height += 1;
        }

        if width == 0 {
            return Err("Empty input".into());
        }

        let next_gen_cells = vec![0; height * width];
        Ok(Self {
            width,
            height,
            cells,
            next_gen_cells,
        })
    }

    fn count_around(&self, x: usize, y: usize, needle: u8) -> u8 {
        let mut sum = 0;
        for dy in [-1_i32, 0, 1] {
            for dx in [-1_i32, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0
                    && (nx as usize) < self.width
                    && ny >= 0
                    && (ny as usize) < self.height
                    && self.cells[(self.width as i32 * ny + nx) as usize] == needle
                {
                    sum += 1;
                }
            }
        }
        sum
    }

    fn advance_minute(&mut self) -> Result<(), String> {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell_value = self.cells[self.width * y + x];
                self.next_gen_cells[self.width * y + x] = match cell_value {
                    b'.' => {
                        if self.count_around(x, y, b'|') >= 3 {
                            b'|'
                        } else {
                            b'.'
                        }
                    }
                    b'|' => {
                        if self.count_around(x, y, b'#') >= 3 {
                            b'#'
                        } else {
                            b'|'
                        }
                    }
                    b'#' => {
                        if self.count_around(x, y, b'#') >= 1 && self.count_around(x, y, b'|') >= 1
                        {
                            b'#'
                        } else {
                            b'.'
                        }
                    }
                    _ => {
                        return Err(format!("Unhandled cell value: {}", cell_value));
                    }
                }
            }
        }
        swap(&mut self.cells, &mut self.next_gen_cells);
        Ok(())
    }

    fn resource_value(&self) -> usize {
        self.cells
            .iter()
            .fold(0, |n, c| n + usize::from(*c == b'|'))
            * self
                .cells
                .iter()
                .fold(0, |n, c| n + usize::from(*c == b'#'))
    }
}

pub fn solve(input: &mut Input) -> Result<usize, String> {
    let mut grid = Grid::parse(input.text)?;

    if input.is_part_one() {
        for _ in 0..10 {
            grid.advance_minute()?;
        }
        Ok(grid.resource_value())
    } else {
        let mut seen = HashMap::new();

        for i in 1..1_000_000_000 {
            grid.advance_minute()?;

            let mut hasher = DefaultHasher::new();
            grid.cells.hash(&mut hasher);
            let hash_value = hasher.finish();

            match seen.entry(hash_value) {
                Entry::Occupied(entry) => {
                    let cycle_length = i - entry.get();
                    let remaining_hashes = (1_000_000_000 - i) % cycle_length;
                    for _ in 0..remaining_hashes {
                        grid.advance_minute()?;
                    }
                    return Ok(grid.resource_value());
                }
                Entry::Vacant(entry) => {
                    entry.insert(i);
                }
            }
        }
        Err("No solution found".to_string())
    }
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!(
            ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."
        => 1147);

    let input = include_str!("day18_input.txt");
    test_part_one!(input => 531_417);
    test_part_two!(input => 205_296);
}
