use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u64, String> {
    const MAX_STEPS_PART_TWO: usize = 100_000;

    let mut board = Board::parse(input.text)?;

    for step in 1..=input.part_values(100, MAX_STEPS_PART_TWO) {
        let before = board.num_flashes;

        board.advance();

        if input.is_part_two() && (board.num_flashes - before) == 100 {
            return Ok(step as u64);
        }
    }

    if input.is_part_two() {
        return Err(format!(
            "No simultaneous flash within {} steps",
            MAX_STEPS_PART_TWO
        ));
    }

    Ok(board.num_flashes)
}

struct Board {
    cells: [u8; Self::WIDTH * Self::WIDTH],
    num_flashes: u64,
}

impl Board {
    const WIDTH: usize = 10;

    fn parse(s: &str) -> Result<Self, String> {
        let mut board = Self {
            cells: [0; Self::WIDTH * Self::WIDTH],
            num_flashes: 0,
        };

        if s.lines().count() != 10 {
            return Err("Board is not 10 rows".to_string());
        }

        for (y, line) in s.lines().enumerate() {
            if line.len() != 10 {
                return Err("Not every row in the board is 10 wide".to_string());
            }
            for (x, b) in line.bytes().enumerate() {
                if !b.is_ascii_digit() {
                    return Err("Not every character is an ASCII digit".to_string());
                }
                board.set(x, y, b - b'0');
            }
        }

        Ok(board)
    }

    const fn at(&self, x: usize, y: usize) -> u8 {
        self.cells[x + (y * Self::WIDTH)]
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        self.cells[x + (y * Self::WIDTH)] = value;
    }

    fn bump(&mut self, x: usize, y: usize) {
        let current_value = self.at(x, y);
        if current_value == 0 {
            return;
        }

        self.set(x, y, current_value + 1);

        if current_value + 1 > 9 {
            self.set(x, y, 0);
            self.num_flashes += 1;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if (dx, dy) != (0, 0) {
                        let n_x = x as i32 + dx;
                        let n_y = y as i32 + dy;
                        if (0..10).contains(&n_x) && (0..10).contains(&n_y) {
                            self.bump(n_x as usize, n_y as usize);
                        }
                    }
                }
            }
        }
    }

    fn advance(&mut self) {
        self.cells.iter_mut().for_each(|c| *c += 1);

        for y in 0..10 {
            for x in 0..10 {
                if self.at(x, y) > 9 {
                    self.bump(x, y);
                }
            }
        }
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    test_part_one!(example => 1656);
    test_part_two!(example => 195);

    let real_input = include_str!("day11_input.txt");
    test_part_one!(real_input => 1546);
    test_part_two!(real_input => 471);
}
