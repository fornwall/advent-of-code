use crate::Input;

struct Grid {
    data: [bool; 10_000],
    stuck_corners: bool,
}

impl Grid {
    fn is_stuck_corner(&self, x: i32, y: i32) -> bool {
        self.stuck_corners
            && ((x, y) == (0, 0) || (x, y) == (99, 0) || (x, y) == (0, 99) || (x, y) == (99, 99))
    }

    fn is_on(&self, x: i32, y: i32) -> bool {
        if self.is_stuck_corner(x, y) {
            true
        } else if (0..100).contains(&x) && (0..100).contains(&y) {
            self.data[(x + y * 100) as usize]
        } else {
            // "Lights on the edge of the grid might have fewer than eight neighbors; the missing ones always count as 'off'."
            false
        }
    }

    fn evolve(&self) -> Self {
        let mut new_data = [false; 10_000];
        for x in 0..100 {
            for y in 0..100 {
                let mut on_neighbors = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if !(dx == 0 && dy == 0) && self.is_on(x + dx, y + dy) {
                            on_neighbors += 1;
                        }
                    }
                }
                new_data[(x + y * 100) as usize] = if self.is_stuck_corner(x, y) {
                    true
                } else if self.is_on(x, y) {
                    on_neighbors == 2 || on_neighbors == 3
                } else {
                    on_neighbors == 3
                };
            }
        }
        Self {
            data: new_data,
            stuck_corners: self.stuck_corners,
        }
    }

    fn parse(input: &str, stuck_corners: bool) -> Result<Self, String> {
        let mut data = [false; 10_000];
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if y >= 100 || x >= 100 {
                    return Err("Invalid grid (not 100x100)".into());
                }
                data[x + y * 100] = char == '#';
            }
        }
        Ok(Self {
            data,
            stuck_corners,
        })
    }

    fn count_lights(&self) -> u32 {
        self.data.iter().filter(|&&b| b).count() as u32
    }
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut grid = Grid::parse(input.text, input.is_part_two())?;
    for _step in 0..100 {
        grid = grid.evolve();
    }
    Ok(grid.count_lights())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day18_input.txt");
    test_part_one!(real_input => 814);
    test_part_two!(real_input => 924);
}
