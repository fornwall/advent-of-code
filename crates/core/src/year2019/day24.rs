use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Grid {
    value: u32,
}

impl Grid {
    fn zeroed() -> Grid {
        Grid { value: 0 }
    }

    fn parse(input: &str) -> Grid {
        Grid {
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
                .fold(0, |value, (x, y)| value | 1 << ((y * 5) + x)),
        }
    }

    fn at(self, x: i32, y: i32) -> u32 {
        if x < 0 || y < 0 || x >= 5 || y >= 5 {
            0
        } else {
            let bit = 1 << ((y * 5) + x);
            if bit & self.value == bit {
                1
            } else {
                0
            }
        }
    }

    fn advance_minute(&mut self) {
        let mut new_value = 0u32;

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

    fn count_bugs(self) -> u32 {
        self.value.count_ones()
    }

    fn count_bugs_at_edge(self, coming_from: (i32, i32)) -> u32 {
        (self.value
            & match coming_from {
                (2, 1) => 0b00000_00000_00000_00000_11111u32,
                (3, 2) => 0b10000_10000_10000_10000_10000u32,
                (2, 3) => 0b11111_00000_00000_00000_00000u32,
                (1, 2) => 0b00001_00001_00001_00001_00001u32,
                _ => panic!("Unsupported direction"),
            })
        .count_ones()
    }

    fn advance(&mut self, inner_grid: Grid, outer_grid: Grid) -> Grid {
        let mut new_value = 0u32;

        for y in 0..5 {
            for x in 0..5 {
                if (x, y) == (2, 2) {
                    continue;
                }

                let mut adjacent_bugs = 0;

                const DIRECTIONS: &[(i32, i32); 4] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];
                for &direction in DIRECTIONS.iter() {
                    let new_x = x + direction.0;
                    let new_y = y + direction.1;
                    adjacent_bugs += if !((0..5).contains(&new_x) && (0..5).contains(&new_y)) {
                        outer_grid.at(2 + direction.0, 2 + direction.1)
                    } else if (new_x, new_y) == (2, 2) {
                        inner_grid.count_bugs_at_edge((x, y))
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

        Grid { value: new_value }
    }
}

pub fn part1(input_string: &str) -> Result<u32, String> {
    let mut grid = Grid::parse(input_string);
    Ok(grid.advance_until_repeat())
}

pub fn part2(input_string: &str) -> Result<u32, String> {
    const MINUTES: usize = 200;
    const MAX_LEVELS: usize = MINUTES * 2;

    let mut current_generation = vec![Grid::zeroed(); MAX_LEVELS];
    let mut next_generation = vec![Grid::zeroed(); MAX_LEVELS];

    current_generation[MAX_LEVELS / 2] = Grid::parse(input_string);

    for _minute in 0..MINUTES {
        for i in 1..(current_generation.len() - 1) {
            let mut this_grid = current_generation[i];
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
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day24_input.txt")), Ok(11042850));
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day24_input.txt")), Ok(1967));
}
