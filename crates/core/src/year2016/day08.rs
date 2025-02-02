use crate::common::character_recognition::{recognize, CHAR_HEIGHT};
use crate::input::Input;

struct Screen {
    pixels: [bool; Screen::WIDTH * CHAR_HEIGHT],
}

impl Screen {
    const WIDTH: usize = 50;

    const fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[y * Self::WIDTH + x]
    }

    const fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.pixels[y * Self::WIDTH + x] = on;
    }

    fn turn_on_rect(&mut self, width: usize, height: usize) {
        for x in 0..width {
            for y in 0..height {
                self.set_pixel(x, y, true);
            }
        }
    }

    fn rotate_row(&mut self, row: usize, amount: usize) {
        let mut new_row = [false; Self::WIDTH];
        for x in 0..Self::WIDTH {
            new_row[(x + amount) % Self::WIDTH] = self.get_pixel(x, row);
        }
        for (x, &on) in new_row.iter().enumerate() {
            self.set_pixel(x, row, on);
        }
    }

    fn rotate_col(&mut self, col: usize, amount: usize) {
        let mut new_col = [false; CHAR_HEIGHT];
        for y in 0..CHAR_HEIGHT {
            new_col[(y + amount) % CHAR_HEIGHT] = self.get_pixel(col, y);
        }

        for (y, &on) in new_col.iter().enumerate() {
            self.set_pixel(col, y, on);
        }
    }

    const fn new() -> Self {
        Self {
            pixels: [false; Self::WIDTH * CHAR_HEIGHT],
        }
    }
}

pub fn solve(input: &Input) -> Result<String, String> {
    let mut screen = Screen::new();
    for line in input.text.lines() {
        if let Some(after) = line.strip_prefix("rect ") {
            let (part1, part2) = after
                .split_once('x')
                .ok_or_else(|| "Invalid input".to_string())?;
            let width = part1.parse::<usize>().map_err(|_| "Invalid input")?;
            let height = part2.parse::<usize>().map_err(|_| "Invalid input")?;
            screen.turn_on_rect(width, height);
        } else if let Some(after) = line.strip_prefix("rotate row y=") {
            let (part1, part2) = after
                .split_once(" by ")
                .ok_or_else(|| "invalid input".to_string())?;
            let row_to_rotate = part1.parse::<usize>().map_err(|_| "Invalid input")?;
            let rotation_amount = part2.parse::<usize>().map_err(|_| "Invalid input")?;
            screen.rotate_row(row_to_rotate, rotation_amount);
        } else if let Some(after) = line.strip_prefix("rotate column x=") {
            let (part1, part2) = after
                .split_once(" by ")
                .ok_or_else(|| "invalid input".to_string())?;
            let col_to_rotate = part1.parse::<usize>().map_err(|_| "Invalid input")?;
            let rotation_amount = part2.parse::<usize>().map_err(|_| "Invalid input")?;
            screen.rotate_col(col_to_rotate, rotation_amount);
        } else {
            return Err("Invalid line".to_string());
        }
    }

    if input.is_part_one() {
        Ok(screen.pixels.iter().filter(|&&p| p).count().to_string())
    } else {
        recognize(&screen.pixels)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day08_input.txt");
    test_part_one!(real_input => "128".to_string());
    test_part_two!(real_input => "EOARGPHYAO".to_string());
}
