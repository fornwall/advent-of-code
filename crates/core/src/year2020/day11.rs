#[cfg(feature = "visualization")]
use super::day11_renderer::Renderer;
use crate::input::Input;

#[derive(Copy, Clone)]
pub struct VisibilityEntry {
    start: u16,
    end: u16,
}

#[derive(Clone)]
pub struct Grid {
    /// 1 if occupied, 0 if empty.
    pub seats: Vec<u8>,
    pub visibility_map: Vec<VisibilityEntry>,
    pub visibility_array: Vec<u16>,
    pub to_visit: Vec<u16>,
    pub changed: Vec<u16>,
}

impl Grid {
    fn parse(
        input: &str,
        part_one: bool,
        #[cfg(feature = "visualization")] renderer: &mut Renderer,
    ) -> Result<Self, String> {
        let rows = input.lines().count() as i32;
        let cols = input.lines().next().ok_or("No lines")?.len() as i32;
        if input.lines().any(|line| line.len() != cols as usize) {
            return Err("Not all lines have equal length".to_string());
        }

        if rows * cols > i32::from(u16::MAX) {
            return Err(format!(
                "Too big input ({}x{}) - max supported seats is {}",
                cols,
                rows,
                u16::MAX
            ));
        }

        let data: Vec<u8> = input.bytes().filter(|&c| c != b'\n').collect();

        let mut data_pos_to_seat_idx = vec![0; data.len()];
        let mut seats_counter = 0;
        for (idx, &c) in data.iter().enumerate() {
            if c == b'.' || c == b'\n' {
                // Ok.
            } else if c == b'L' {
                // Free seat.
                data_pos_to_seat_idx[idx] = seats_counter as u16;

                #[cfg(feature = "visualization")]
                renderer.add_idx_mapping(seats_counter, idx as i32 % cols, idx as i32 / cols);

                seats_counter += 1;
            } else {
                return Err("Invalid input - only 'L', '.' and '\n' expected".to_string());
            }
        }

        let mut visibility_map = vec![VisibilityEntry { start: 0, end: 0 }; seats_counter];
        let mut visibility_array = Vec::with_capacity(seats_counter * 8);
        for (idx, _) in data.iter().enumerate().filter(|(_, &c)| c != b'.') {
            let x = (idx as i32) % cols;
            let y = (idx as i32) / cols;
            let mut visibility_entry = VisibilityEntry {
                start: visibility_array.len() as u16,
                end: visibility_array.len() as u16,
            };
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
                        if data[visited_idx] == b'L' {
                            visibility_entry.end += 1;
                            visibility_array.push(data_pos_to_seat_idx[visited_idx] as u16);
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

            visibility_map[data_pos_to_seat_idx[idx] as usize] = visibility_entry;
        }

        let seats = vec![0; seats_counter];

        let mut to_visit = vec![0_u16; seats.len()];
        for (idx, element) in to_visit.iter_mut().enumerate() {
            *element = idx as u16;
        }

        let changed = Vec::with_capacity(seats.len());

        Ok(Self {
            seats,
            visibility_map,
            visibility_array,
            to_visit,
            changed,
        })
    }

    fn evolve(&mut self, leave_when_seeing: u8) -> bool {
        let seats = &mut self.seats;
        let visibility_map = &self.visibility_map;
        let visibility_array = &self.visibility_array;
        let changed = &mut self.changed;

        self.to_visit.retain(|&visited_seat_idx| {
            let idx = visited_seat_idx as usize;
            let seat_occupied = seats[idx] == 1;
            let visibility = visibility_map[idx];
            let seen_from_here_count = visibility_array
                [(visibility.start as usize)..(visibility.end as usize)]
                .iter()
                .map(|&idx| seats[idx as usize])
                .sum::<u8>();

            // Free seat that is now taken or occupied seat that is now left.
            if (!seat_occupied && seen_from_here_count == 0)
                || (seat_occupied && seen_from_here_count >= leave_when_seeing)
            {
                changed.push(visited_seat_idx);
                true
            } else {
                // If seat occupance has not changed in a cycle, it will never change,
                // so we never need to re-visit it.
                false
            }
        });

        let changed = !self.changed.is_empty();
        self.changed
            .iter()
            .for_each(|&idx| seats[idx as usize] = (seats[idx as usize] + 1) % 2);
        self.changed.clear();
        changed
    }
}

pub fn solve(input: &mut Input) -> Result<usize, String> {
    const MAX_ITERATIONS: u32 = 10_000;
    let mut iteration = 0;

    let leave_when_seeing = input.part_values(4, 5);
    let part_one = input.is_part_one();

    #[cfg(feature = "visualization")]
    let mut renderer = Renderer::new(&mut input.painter);

    let mut grid = Grid::parse(
        input.text,
        part_one,
        #[cfg(feature = "visualization")]
        &mut renderer,
    )?;
    while grid.evolve(leave_when_seeing) {
        iteration += 1;
        if iteration >= MAX_ITERATIONS {
            return Err(format!("Aborting after {} iterations", iteration));
        }
    }

    return Ok(grid.seats.iter().filter(|&&s| s == 1).count());
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
pub fn limited_memory_allocations() {
    use crate::{test_part_one, test_part_two};
    let real_input = include_str!("day11_input.txt");
    let allocations = allocation_counter::count(|| {
        test_part_one!(real_input => 2222);
        test_part_two!(real_input => 2032);
    });
    assert!(allocations < 100);
}
