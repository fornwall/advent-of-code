use crate::Input;

struct Screen {
    pixels: [bool; Screen::WIDTH * Screen::HEIGHT],
}

impl Screen {
    const WIDTH: usize = 50;
    const LETTER_WIDTH: usize = 5;
    const HEIGHT: usize = 6;

    const fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[y * Self::WIDTH + x]
    }

    fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
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
        let mut new_col = [false; Self::HEIGHT];
        for y in 0..Self::HEIGHT {
            new_col[(y + amount) % Self::HEIGHT] = self.get_pixel(col, y);
        }

        for (y, &on) in new_col.iter().enumerate() {
            self.set_pixel(col, y, on);
        }
    }

    const fn new() -> Self {
        Self {
            pixels: [false; Self::WIDTH * Self::HEIGHT],
        }
    }
}

pub fn solve(input: &mut Input) -> Result<String, String> {
    let mut screen = Screen::new();
    for line in input.text.lines() {
        if let Some(after) = line.strip_prefix("rect ") {
            let parts = after.split('x').collect::<Vec<_>>();
            if parts.len() != 2 {
                return Err("Invalid input".to_string());
            }
            let width = parts[0].parse::<usize>().map_err(|_| "Invalid input")?;
            let height = parts[1].parse::<usize>().map_err(|_| "Invalid input")?;
            screen.turn_on_rect(width, height);
        } else if let Some(after) = line.strip_prefix("rotate row y=") {
            let parts = after.split(" by ").collect::<Vec<_>>();
            if parts.len() != 2 {
                return Err("Invalid input".to_string());
            }
            let row_to_rotate = parts[0].parse::<usize>().map_err(|_| "Invalid input")?;
            let rotation_amount = parts[1].parse::<usize>().map_err(|_| "Invalid input")?;
            screen.rotate_row(row_to_rotate, rotation_amount);
        } else if let Some(after) = line.strip_prefix("rotate column x=") {
            let parts = after.split(" by ").collect::<Vec<_>>();
            if parts.len() != 2 {
                return Err("Invalid input".to_string());
            }
            let col_to_rotate = parts[0].parse::<usize>().map_err(|_| "Invalid input")?;
            let rotation_amount = parts[1].parse::<usize>().map_err(|_| "Invalid input")?;
            screen.rotate_col(col_to_rotate, rotation_amount);
        } else {
            return Err("Invalid line".to_string());
        }
    }

    if input.is_part_one() {
        Ok(screen.pixels.iter().filter(|&&p| p).count().to_string())
    } else {
        let mut code_on_screen = String::new();

        for letter_idx in 0..(Screen::WIDTH / Screen::LETTER_WIDTH) {
            let mut this_char_string = String::new();
            for y in 0..Screen::HEIGHT {
                for x in 0..Screen::LETTER_WIDTH {
                    this_char_string.push(
                        if screen.get_pixel(letter_idx * Screen::LETTER_WIDTH + x, y) {
                            '█'
                        } else {
                            ' '
                        },
                    );
                }
                this_char_string.push('\n');
            }

            if let Some(c) = recognize(&this_char_string) {
                code_on_screen.push(c);
            }
        }
        Ok(code_on_screen)
    }
}

fn recognize(input: &str) -> Option<char> {
    match input {
        " ██  \n█  █ \n█  █ \n████ \n█  █ \n█  █ \n" => Some('A'),
        "███  \n█  █ \n███  \n█  █ \n█  █ \n███  \n" => Some('B'),
        " ██  \n█  █ \n█    \n█    \n█  █ \n ██  \n" => Some('C'),
        "████ \n█    \n███  \n█    \n█    \n████ \n" => Some('E'),
        "████ \n█    \n███  \n█    \n█    \n█    \n" => Some('F'),
        " ██  \n█  █ \n█    \n█ ██ \n█  █ \n ███ \n" => Some('G'),
        "█  █ \n█  █ \n████ \n█  █ \n█  █ \n█  █ \n" => Some('H'),
        " ███ \n  █  \n  █  \n  █  \n  █  \n ███ \n" => Some('I'),
        "  ██ \n   █ \n   █ \n   █ \n█  █ \n ██  \n" => Some('J'),
        "█  █ \n█ █  \n██   \n█ █  \n█ █  \n█  █ \n" => Some('K'),
        "█    \n█    \n█    \n█    \n█    \n████ \n" => Some('L'),
        " ██  \n█  █ \n█  █ \n█  █ \n█  █ \n ██  \n" => Some('O'),
        "███  \n█  █ \n█  █ \n███  \n█    \n█    \n" => Some('P'),
        "███  \n█  █ \n█  █ \n███  \n█ █  \n█  █ \n" => Some('R'),
        " ███ \n█    \n█    \n ██  \n   █ \n███  \n" => Some('S'),
        "█  █ \n█  █ \n█  █ \n█  █ \n█  █ \n ██  \n" => Some('U'),
        "█   █\n█   █\n █ █ \n  █  \n  █  \n  █  \n" => Some('Y'),
        "████ \n   █ \n  █  \n █   \n█    \n████ \n" => Some('Z'),
        _ => {
            let output = input.replace('\n', "\\n");
            println!(
                "Unrecognized string:\n{}\n\nAdd clause:\n\"{}\" => Some('?'),",
                input, output
            );
            None
        }
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day08_input.txt");
    test_part_one!(real_input => "128".to_string());
    test_part_two!(real_input => "EOARGPHYAO".to_string());
}
