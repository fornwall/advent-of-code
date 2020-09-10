use std::collections::hash_map::{DefaultHasher, Entry};
use std::collections::HashMap;
use std::env;
use std::hash::{Hash, Hasher};
use std::mem::swap;

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<u8>,
    next_gen_cells: Vec<u8>,
}

impl Grid {
    fn parse(input_string: &str) -> Grid {
        let mut height = 0;
        let mut width = 0;
        let mut cells = Vec::new();
        for line in input_string.lines() {
            line.chars().for_each(|c| cells.push(c as u8));
            width = line.len();
            height += 1;
        }

        let next_gen_cells = vec![0; height * width];

        Grid {
            width,
            height,
            cells,
            next_gen_cells,
        }
    }

    fn count_around(&self, x: usize, y: usize, needle: u8) -> u8 {
        let mut sum = 0;
        for &dy in &[-1i32, 0, 1] {
            for &dx in &[-1i32, 0, 1] {
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

    fn advance_minute(&mut self) {
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
                        panic!("Unhandled cell value: {}", cell_value);
                    }
                }
            }
        }
        swap(&mut self.cells, &mut self.next_gen_cells);
    }

    fn resource_value(&self) -> usize {
        self.cells.iter().fold(0, |n, c| n + (*c == b'|') as usize)
            * self.cells.iter().fold(0, |n, c| n + (*c == b'#') as usize)
    }

    fn print(&self) {
        if env::var("ADVENT_DEBUG").is_err() {
            return;
        }
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.cells[self.width * y + x] as char);
            }
            println!();
        }
        println!();
    }
}

pub fn part1(input_string: &str) -> String {
    let mut grid = Grid::parse(input_string);
    grid.print();
    for _ in 0..10 {
        grid.advance_minute();
        grid.print();
    }
    grid.resource_value().to_string()
}

pub fn part2(input_string: &str) -> String {
    let mut grid = Grid::parse(input_string);
    grid.print();

    let mut seen = HashMap::new();

    for i in 1..1_000_000_000 {
        grid.advance_minute();

        let mut hasher = DefaultHasher::new();
        grid.cells.hash(&mut hasher);
        let hash_value = hasher.finish();

        match seen.entry(hash_value) {
            Entry::Occupied(entry) => {
                let cycle_length = i - entry.get();
                let remaining_hashes = (1_000_000_000 - i) % cycle_length;
                for _ in 0..remaining_hashes {
                    grid.advance_minute();
                }
                return grid.resource_value().to_string();
            }
            Entry::Vacant(entry) => {
                entry.insert(i);
            }
        }
    }
    "?".to_string()
}

#[test]
fn tests_part1() {
    assert_eq!(
        "1147",
        part1(
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
        )
    );

    assert_eq!("531417", part1(include_str!("day18_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!("205296", part2(include_str!("day18_input.txt")));
}
