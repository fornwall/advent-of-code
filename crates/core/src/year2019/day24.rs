use crate::input::Input;
use std::collections::HashSet;
use std::slice::Iter;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const fn delta(self) -> (i32, i32) {
        match self {
            Self::Up => (0, 1),
            Self::Right => (1, 0),
            Self::Down => (0, -1),
            Self::Left => (-1, 0),
        }
    }
    fn iterator() -> Iter<'static, Self> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        DIRECTIONS.iter()
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Grid {
    value: u32,
}

impl Grid {
    const fn zeroed() -> Self {
        Self { value: 0 }
    }

    fn parse(input: &str) -> Result<Self, String> {
        if input.chars().filter(|&c| c == '#' || c == '.').count() != 25
            || input.chars().any(|c| !matches!(c, '#' | '.' | '\n'))
            || input.lines().count() != 5
        {
            return Err("Invalid input - expected 5x5 grid of '#' and '.'".to_string());
        }
        Ok(Self {
            value: input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(move |(x, character)| match character {
                            '#' => Some((x, y)),
                            _ => None,
                        })
                })
                .fold(0, |value, (x, y)| value | (1 << ((y * 5) + x))),
        })
    }

    fn at(self, x: i32, y: i32) -> u32 {
        if x < 0 || y < 0 || x >= 5 || y >= 5 {
            0
        } else {
            let bit = 1 << ((y * 5) + x);
            u32::from(bit & self.value == bit)
        }
    }

    fn advance_minute(&mut self) {
        let mut new_value = 0_u32;

        for y in 0..5 {
            for x in 0..5 {
                let adjacent_bugs =
                    self.at(x - 1, y) + self.at(x + 1, y) + self.at(x, y - 1) + self.at(x, y + 1);
                if adjacent_bugs == 1 || (self.at(x, y) == 0 && adjacent_bugs == 2) {
                    let bit = 1 << ((y * 5) + x);
                    new_value |= bit;
                }
            }
        }

        self.value = new_value;
    }

    fn advance_until_repeat(&mut self) -> u32 {
        let mut seen = HashSet::new();

        while seen.insert(self.value) {
            self.advance_minute();
        }

        self.value
    }

    const fn count_bugs(self) -> u32 {
        self.value.count_ones()
    }

    const fn count_bugs_at_edge(self, coming_from: Direction) -> u32 {
        #![allow(clippy::unusual_byte_groupings)]
        (self.value
            & match coming_from {
                Direction::Up => 0b00000_00000_00000_00000_11111_u32,
                Direction::Left => 0b10000_10000_10000_10000_10000_u32,
                Direction::Down => 0b11111_00000_00000_00000_00000_u32,
                Direction::Right => 0b00001_00001_00001_00001_00001_u32,
            })
        .count_ones()
    }

    fn advance(self, inner_grid: Self, outer_grid: Self) -> Self {
        let mut new_value = 0_u32;

        for y in 0..5 {
            for x in 0..5 {
                if (x, y) == (2, 2) {
                    continue;
                }

                let mut adjacent_bugs = 0;

                for &direction in Direction::iterator() {
                    let delta = direction.delta();
                    let new_x = x + delta.0;
                    let new_y = y + delta.1;
                    adjacent_bugs += if !((0..5).contains(&new_x) && (0..5).contains(&new_y)) {
                        outer_grid.at(2 + delta.0, 2 + delta.1)
                    } else if (new_x, new_y) == (2, 2) {
                        inner_grid.count_bugs_at_edge(direction)
                    } else {
                        self.at(new_x, new_y)
                    };
                }

                if adjacent_bugs == 1 || (self.at(x, y) == 0 && adjacent_bugs == 2) {
                    let bit = 1 << ((y * 5) + x);
                    new_value |= bit;
                }
            }
        }

        Self { value: new_value }
    }
}

pub fn solve(input: &Input) -> Result<u32, String> {
    const MINUTES: usize = 200;
    const MAX_LEVELS: usize = MINUTES * 2;

    let mut grid = Grid::parse(input.text)?;

    if input.is_part_one() {
        return Ok(grid.advance_until_repeat());
    }

    let mut current_generation = vec![Grid::zeroed(); MAX_LEVELS];
    let mut next_generation = vec![Grid::zeroed(); MAX_LEVELS];

    current_generation[MAX_LEVELS / 2] = grid;

    for _minute in 0..MINUTES {
        for i in 1..(current_generation.len() - 1) {
            let this_grid = current_generation[i];
            next_generation[i] =
                this_grid.advance(current_generation[i - 1], current_generation[i + 1]);
        }
        std::mem::swap(&mut current_generation, &mut next_generation);
    }

    Ok(current_generation
        .iter()
        .map(|value| value.count_bugs())
        .sum::<u32>())
}

#[test]
pub fn tests() {
    let input = include_str!("day24_input.txt");
    test_part_one!(input => 11_042_850);
    test_part_two!(input => 1967);
}
