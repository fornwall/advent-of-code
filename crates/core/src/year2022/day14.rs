use crate::common::tuple_window_iterator::TupleWindowIteratorExt;
use crate::input::Input;

struct Grid {
    data: Vec<bool>,
    highest_y_coordinate: usize,
    sand_count: usize,
}

impl Grid {
    const SQUARE_WIDTH: usize = 1000;

    fn parse(input: &str) -> Result<Self, String> {
        let mut highest_y_coordinate = 0;
        let mut data = vec![false; Self::SQUARE_WIDTH * Self::SQUARE_WIDTH];
        for line in input.lines() {
            for (from_str, to_str) in line.split(" -> ").tuple_windows() {
                let mut from_it = from_str.split(',');
                let mut to_it = to_str.split(',');
                if let (Some(from_x), Some(from_y), Some(to_x), Some(to_y)) = (
                    from_it.next().and_then(|s| s.parse::<u16>().ok()),
                    from_it.next().and_then(|s| s.parse::<u16>().ok()),
                    to_it.next().and_then(|s| s.parse::<u16>().ok()),
                    to_it.next().and_then(|s| s.parse::<u16>().ok()),
                ) {
                    if from_x.max(to_x).max(from_y).max(to_y) >= Self::SQUARE_WIDTH as u16 {
                        return Err(format!(
                            "Grid is not within [{},{})",
                            Self::SQUARE_WIDTH,
                            Self::SQUARE_WIDTH
                        ));
                    }
                    highest_y_coordinate = highest_y_coordinate.max(from_y).max(to_y);
                    if from_x == to_x {
                        for y in from_y.min(to_y)..=from_y.max(to_y) {
                            data[y as usize * Self::SQUARE_WIDTH + from_x as usize] = true;
                        }
                    } else {
                        for x in from_x.min(to_x)..=from_x.max(to_x) {
                            data[from_y as usize * Self::SQUARE_WIDTH + x as usize] = true;
                        }
                    }
                }
            }
        }
        Ok(Self {
            data,
            highest_y_coordinate: highest_y_coordinate as usize,
            sand_count: 0,
        })
    }

    fn is_free(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= Self::SQUARE_WIDTH as i32 || y >= Self::SQUARE_WIDTH as i32 {
            return false;
        }
        !self.data[y as usize * Self::SQUARE_WIDTH + x as usize]
    }

    fn set_sand(&mut self, x: i32, y: i32) {
        if !(x < 0 || y < 0 || x >= Self::SQUARE_WIDTH as i32 || y >= Self::SQUARE_WIDTH as i32) {
            self.data[y as usize * Self::SQUARE_WIDTH + x as usize] = true;
            self.sand_count += 1;
        }
    }

    fn fill_part_1(&mut self) {
        let (mut x, mut y) = (500, 0);
        'outer: while y != self.highest_y_coordinate as i32 {
            for dx in [0, -1, 1] {
                if self.is_free(x + dx, y + 1) {
                    y += 1;
                    x += dx;
                    continue 'outer;
                }
            }
            self.set_sand(x, y);
            (x, y) = (500, 0);
        }
    }

    fn fill_part_2(&mut self, x: i32, y: i32) {
        if self.is_free(x, y) && y < self.highest_y_coordinate as i32 + 2 {
            self.set_sand(x, y);
            for dx in [0, -1, 1] {
                self.fill_part_2(x + dx, y + 1);
            }
        }
    }
}

pub fn solve(input: &Input) -> Result<usize, String> {
    let mut grid = Grid::parse(input.text)?;
    if input.is_part_one() {
        grid.fill_part_1();
    } else {
        grid.fill_part_2(500, 0);
    }
    Ok(grid.sand_count)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
    test_part_one!(test_input => 24);
    test_part_two!(test_input => 93);

    let real_input = include_str!("day14_input.txt");
    test_part_one!(real_input => 793);
    test_part_two!(real_input => 24166);
}
