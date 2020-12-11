use crate::input::Input;

struct Grid {
    data: Vec<u8>,
    scratch: Vec<u8>,
    cols: i32,
    rows: i32,
}

impl Grid {
    fn parse(input: &str) -> Result<Self, String> {
        let rows = input.lines().count() as i32;
        let cols = input.lines().next().ok_or("No rows")?.len() as i32;
        if input.lines().any(|line| line.len() != cols as usize) {
            return Err("Not all lines have equal length".to_string());
        }
        let data: Vec<u8> = input.bytes().filter(|&c| c != b'\n').collect();
        if data.iter().any(|c| !matches!(c, b'#' | b'L' | b'.')) {
            return Err("Invalid input - only '#', 'L', '.' and '\n' expected".to_string());
        }
        let scratch = data.clone();
        Ok(Self {
            data,
            scratch,
            cols,
            rows,
        })
    }

    fn at(&self, x: i32, y: i32) -> Option<u8> {
        if x < 0 || y < 0 || x >= self.cols || y >= self.rows {
            None
        } else {
            Some(self.data[(x + self.cols * y) as usize])
        }
    }

    fn evolve(&mut self, part_one: bool) -> bool {
        for y in 0..self.rows {
            'col: for x in 0..self.cols {
                let idx = (x + y * self.cols) as usize;
                let current_value = self.data[(x + y * self.cols) as usize];
                if current_value == b'.' {
                    continue;
                }
                let occupied_adjacent_for_leave = if part_one { 4 } else { 5 };

                let mut adjacent_occupied = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if !(dx == 0 && dy == 0) {
                            let mut new_x = x + dx;
                            let mut new_y = y + dy;
                            loop {
                                match self.at(new_x, new_y) {
                                    Some(b'#') => {
                                        if current_value == b'L' {
                                            self.scratch[idx] = current_value;
                                            continue 'col;
                                        }
                                        adjacent_occupied += 1;
                                        if current_value == b'#'
                                            && adjacent_occupied >= occupied_adjacent_for_leave
                                        {
                                            self.scratch[idx] = b'L';
                                            continue 'col;
                                        }
                                        break;
                                    }
                                    Some(b'L') | None => {
                                        break;
                                    }
                                    _ => {}
                                }
                                new_x += dx;
                                new_y += dy;
                                if part_one {
                                    break;
                                }
                            }
                        }
                    }
                }

                self.scratch[idx] = b'#';
            }
        }

        std::mem::swap(&mut self.scratch, &mut self.data);
        self.scratch != self.data
    }
}

#[allow(clippy::naive_bytecount)]
pub fn solve(input: &mut Input) -> Result<usize, String> {
    const MAX_ITERATIONS: u32 = 10_000;
    let mut grid = Grid::parse(input.text)?;
    let mut iteration = 0;
    while grid.evolve(input.is_part_one()) {
        iteration += 1;
        if iteration >= MAX_ITERATIONS {
            return Err(format!("Aborting after {} iterations", iteration));
        }
    }
    return Ok(grid.data.iter().filter(|&&c| c == b'#').count());
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    test_part_one!(example => 37);
    test_part_two!(example => 26);

    let real_input = include_str!("day11_input.txt");
    test_part_one!(real_input => 2222);
    test_part_two!(real_input => 2032);
}
