use std::cmp::{max, min};
use std::env;

fn parse_point_interval(s: &str) -> Result<(u16, u16), String> {
    if s.contains("..") {
        let parts: Vec<&str> = s.split("..").collect();
        if parts.len() != 2 {
            return Err("Invalid input".to_string());
        }
        Ok((
            parts[0].parse::<u16>().map_err(|_| "Invalid input")?,
            parts[1].parse::<u16>().map_err(|_| "Invalid input")?,
        ))
    } else {
        let point = s.parse::<u16>().map_err(|_| "Invalid input")?;
        Ok((point, point))
    }
}

struct Grid {
    cells: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from(input_string: &str) -> Result<Self, String> {
        let mut points: Vec<(u16, u16)> = Vec::new();
        let mut x_range = (std::u16::MAX, std::u16::MIN);
        let mut y_range = (std::u16::MAX, std::u16::MIN);

        for line in input_string.lines() {
            let mut parts: Vec<&str> = line.split(", ").collect();
            if parts.len() != 2 {
                return Err("Invalid input".to_string());
            }
            parts.sort_unstable();
            let (x_start, x_end) = parse_point_interval(&parts[0][2..])?;
            let (y_start, y_end) = parse_point_interval(&parts[1][2..])?;

            x_range = (min(x_range.0, x_start), max(x_range.1, x_end));
            y_range = (min(y_range.0, y_start), max(y_range.1, y_end));

            for x in x_start..=x_end {
                for y in y_start..=y_end {
                    points.push((x, y));
                }
            }
        }

        // Water pouring down at sides:
        x_range = (x_range.0 - 1, x_range.1 + 1);

        // +1 since ranges are inclusive:
        let width = ((x_range.1 - x_range.0) + 1) as usize;
        let height = ((y_range.1 - y_range.0) + 1) as usize;

        let mut cells = vec![b'.'; width as usize * height as usize];
        for point in points {
            let x = point.0 - x_range.0;
            let y = point.1 - y_range.0;
            cells[y as usize * width + x as usize] = b'#';
        }

        let water_x = 500 - x_range.0;
        // Place water on top (may exist above as well coming from the spring, but ignore as that's
        // above the minimum y value).
        let water_y = 0;
        cells[(water_y * width + water_x as usize) as usize] = b'|';

        Ok(Self {
            cells,
            width,
            height,
        })
    }

    fn print(&self, name: &str) {
        if env::var("ADVENT_DEBUG").is_err() {
            return;
        }
        println!("### {}", name);
        for y in 0..self.height {
            println!(
                "{}",
                String::from_utf8(self.cells[y * self.width..(y + 1) * self.width].to_vec())
                    .unwrap_or_else(|_| "Invalid utf-8".to_string())
                    .replace('.', " ")
                    .replace('#', "█")
            );
        }
        println!();
    }

    fn at(&self, x: u16, y: u16) -> u8 {
        self.cells[y as usize * self.width + x as usize]
    }

    fn set_water_at(&mut self, x: u16, y: u16, solid: bool) {
        self.cells[y as usize * self.width + x as usize] = if solid { b'w' } else { b'|' };
    }

    fn dry_at(&mut self, x: u16, y: u16) {
        self.cells[y as usize * self.width + x as usize] = b'.';
    }

    fn wall_in_direction(&self, x_start: u16, y: u16, x_direction: i32) -> bool {
        let mut x = (i32::from(x_start) + x_direction) as u16;
        loop {
            let cell = self.at(x, y);
            if !(cell == b'.' || cell == b'w' || cell == b'|') {
                break;
            }
            let below = self.at(x, y + 1);
            if !(below == b'#' || below == b'w') {
                return false;
            }
            x = (i32::from(x) + x_direction) as u16;
        }
        self.at(x, y) == b'#'
    }

    fn fill_in_direction(&mut self, x_start: u16, y: u16, x_direction: i32, solid: bool) {
        let mut x = (i32::from(x_start) + x_direction) as u16;
        loop {
            let cell = self.at(x, y);
            if !(cell == b'.' || cell == b'w' || cell == b'|') {
                break;
            }
            self.set_water_at(x, y, solid);
            let below = self.at(x, y + 1);
            if !(below == b'#' || below == b'w') {
                return;
            }
            x = (i32::from(x) + x_direction) as u16;
        }
    }

    fn spread_water_at(&mut self, x: u16, y: u16) -> u16 {
        self.set_water_at(x, y, false);

        if (y as usize) < self.height - 1 {
            let below = self.at(x, y + 1);
            if below == b'#' || below == b'w' {
                let left_wall = self.wall_in_direction(x, y, -1);
                let right_wall = self.wall_in_direction(x, y, 1);
                let surrounded_by_walls = left_wall && right_wall;
                self.fill_in_direction(x, y, -1, surrounded_by_walls);
                self.fill_in_direction(x, y, 1, surrounded_by_walls);
                if surrounded_by_walls {
                    self.set_water_at(x, y, true);
                }
                if surrounded_by_walls && y > 0 {
                    return self.spread_water_at(x, y - 1);
                }
            }
        }
        y
    }

    fn pour_water(&mut self) {
        let mut line = 1;
        while line < self.height {
            let mut top_y = line;
            let mut to_fill = Vec::new();
            for x in 0..self.width {
                if self.at(x as u16, line as u16 - 1) == b'|'
                    && self.at(x as u16, line as u16) == b'.'
                {
                    to_fill.push(x);
                }
            }
            for x in to_fill {
                top_y = min(top_y, self.spread_water_at(x as u16, line as u16) as usize);
            }
            line = top_y + 1;
        }
    }

    fn holds_in_direction(&self, x_start: u16, y: u16, x_direction: i32) -> bool {
        let mut x = (i32::from(x_start) + x_direction) as u16;
        loop {
            let cell = self.at(x, y);
            let below = self.at(x, y + 1);
            if !(below == b'#' || below == b'w') {
                return false;
            }
            if cell == b'#' {
                return true;
            }
            x = (i32::from(x) + x_direction) as u16;
        }
    }

    fn dry_up(&mut self) {
        let mut line = self.height as u16;
        while line > 0 {
            line -= 1;
            for x in 0..self.width {
                if self.at(x as u16, line as u16) == b'w' {
                    let below = if line == (self.height as u16 - 1) {
                        b'.'
                    } else {
                        self.at(x as u16, line + 1)
                    };

                    if !((below == b'#' || below == b'w')
                        && self.holds_in_direction(x as u16, line, -1)
                        && self.holds_in_direction(x as u16, line, 1))
                    {
                        self.dry_at(x as u16, line as u16);
                    }
                }
            }
        }
    }

    fn count_water(&self) -> usize {
        self.cells
            .iter()
            .fold(0, |n, c| n + (*c == b'w' || *c == b'|') as usize)
    }

    fn count_drained_water(&self) -> usize {
        self.cells.iter().fold(0, |n, c| n + (*c == b'w') as usize)
    }
}

pub fn part1(input_string: &str) -> Result<usize, String> {
    let mut grid = Grid::from(input_string)?;
    grid.print("Initial");
    grid.pour_water();
    grid.print("After pouring");
    Ok(grid.count_water())
}

pub fn part2(input_string: &str) -> Result<usize, String> {
    let mut grid = Grid::from(input_string)?;
    grid.print("Initial");
    grid.pour_water();
    grid.print("After pouring");
    grid.dry_up();
    grid.print("After drying up");
    Ok(grid.count_drained_water())
}

#[test]
fn tests_part1() {
    assert_eq!(
        Ok(57),
        part1(
            "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"
        )
    );

    assert_eq!(Ok(31949), part1(include_str!("day17_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(
        Ok(29),
        part2(
            "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"
        )
    );

    assert_eq!(Ok(26384), part2(include_str!("day17_input.txt")));
}
