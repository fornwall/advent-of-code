use crate::input::Input;

#[derive(Clone)]
struct Grid {
    /// True if occupied, false if empty.
    seats: Vec<bool>,
    scratch: Vec<bool>,
    /// For a seat at seats[seat_idx], visibility_map[seat_idx] contains the indices into seats that are visible.
    visibility_map: Vec<Vec<u16>>,
    cols: i32,
    rows: i32,
}

impl Grid {
    fn parse(input: &str, part_one: bool) -> Result<Self, String> {
        let rows = input.lines().count() as i32;
        let cols = input.lines().next().ok_or("No lines")?.len() as i32;
        if input.lines().any(|line| line.len() != cols as usize) {
            return Err("Not all lines have equal length".to_string());
        }
        let data: Vec<u8> = input.bytes().filter(|&c| c != b'\n').collect();
        if data.iter().any(|c| !matches!(c, b'#' | b'L' | b'.')) {
            return Err("Invalid input - only '#', 'L', '.' and '\n' expected".to_string());
        }

        let mut data_pos_to_seat_idx = vec![0; data.len()];
        let mut seats_counter = 0;
        for (idx, _) in data.iter().enumerate().filter(|(_, &c)| c != b'.') {
            data_pos_to_seat_idx[idx] = seats_counter as u16;
            seats_counter += 1;
        }

        // Build up visibility map:
        let mut visibility_map = vec![Vec::new(); seats_counter];
        for (idx, _) in data.iter().enumerate().filter(|(_, &c)| c != b'.') {
            let x = (idx as i32) % cols;
            let y = (idx as i32) / cols;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let mut new_x = x + dx;
                    let mut new_y = y + dy;
                    loop {
                        if new_x < 0 || new_y < 0 || new_x >= cols || new_y >= rows {
                            break;
                        }

                        let visited_idx = (new_x + cols * new_y) as usize;
                        if matches!(data[visited_idx], b'#' | b'L') {
                            visibility_map[data_pos_to_seat_idx[idx] as usize]
                                .push(data_pos_to_seat_idx[visited_idx]);
                            break;
                        }

                        if part_one {
                            break;
                        }

                        new_x += dx;
                        new_y += dy;
                    }
                }
            }
        }

        let mut seats = vec![false; seats_counter];
        for (idx, &char) in data.iter().enumerate().filter(|(_, &c)| c != b'.') {
            seats[data_pos_to_seat_idx[idx] as usize] = char == b'#';
        }

        let scratch = seats.clone();

        Ok(Self {
            seats,
            scratch,
            visibility_map,
            cols,
            rows,
        })
    }

    fn evolve(&mut self, leave_when_seeing: usize) -> bool {
        for (idx, visible) in self.visibility_map.iter().enumerate() {
            let seen_from_here_count = visible
                .iter()
                .filter(|&&idx| self.seats[idx as usize])
                .count();

            self.scratch[idx] = if !self.seats[idx] && seen_from_here_count == 0 {
                // Free seat that is now taken
                true
            } else if self.seats[idx] && seen_from_here_count >= leave_when_seeing {
                // Occupied seat that is now left.
                false
            } else {
                self.seats[idx]
            };
        }

        std::mem::swap(&mut self.scratch, &mut self.seats);
        self.scratch != self.seats
    }
}

pub fn solve(input: &mut Input) -> Result<usize, String> {
    const MAX_ITERATIONS: u32 = 10_000;
    let mut iteration = 0;

    let mut grid = Grid::parse(input.text, input.is_part_one())?;
    while grid.evolve(input.part_values(4, 5)) {
        iteration += 1;
        if iteration >= MAX_ITERATIONS {
            return Err(format!("Aborting after {} iterations", iteration));
        }
    }

    return Ok(grid.seats.iter().filter(|&&occupied| occupied).count());
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
#[cfg(feature = "count-allocations")]
#[test]
pub fn no_memory_allocations() {
    use crate::{test_part_one, test_part_two};
    let real_input = include_str!("day11_input.txt");
    let allocations = allocation_counter::count(|| {
        test_part_one!(real_input => 2222);
        test_part_two!(real_input => 2032);
    });
    assert_eq!(allocations, 28);
}
