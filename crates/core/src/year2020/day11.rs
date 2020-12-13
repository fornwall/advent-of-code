#[cfg(feature = "visualization")]
use super::day11_renderer::Renderer;
use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    const MAX_ITERATIONS: u32 = 10_000;

    let leave_when_seeing = input.part_values(4, 5);
    let part_one = input.is_part_one();

    #[cfg(feature = "visualization")]
    let mut renderer = Renderer::new(&mut input.painter);

    let rows = input.text.lines().count() as i32;
    let cols = input.text.lines().next().ok_or("No lines")?.len() as i32;
    if input.text.lines().any(|line| line.len() != cols as usize) {
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

    let data: Vec<u8> = input.text.bytes().filter(|&c| c != b'\n').collect();

    let mut data_pos_to_seat_idx = vec![0; data.len()];
    let mut seats_counter = 0;
    for (idx, &c) in data.iter().enumerate().filter(|(_, &c)| c != b'.') {
        if c != b'L' {
            return Err("Invalid input - only 'L', '.' and '\n' expected".to_string());
        }
        data_pos_to_seat_idx[idx] = seats_counter as u16;

        #[cfg(feature = "visualization")]
        renderer.add_idx_mapping(seats_counter, idx as i32 % cols, idx as i32 / cols);

        seats_counter += 1;
    }

    // An extra proxy at end for pointing to.
    let mut seats = vec![false; seats_counter + 1];
    let mut to_visit = (0..seats_counter as u16).collect::<Vec<u16>>();
    let mut visibility_map = Vec::with_capacity(seats_counter);
    for (idx, _) in data.iter().enumerate().filter(|(_, &c)| c != b'.') {
        let x = (idx as i32) % cols;
        let y = (idx as i32) / cols;
        let mut visibility_entry = [seats_counter as u16; 8];
        let mut visibility_count = 0;
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
                        visibility_entry[visibility_count] =
                            data_pos_to_seat_idx[visited_idx] as u16;
                        visibility_count += 1;
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

        visibility_map.push(visibility_entry);
    }

    let mut changes: Vec<u16> = Vec::with_capacity(seats_counter);

    let mut iteration = 0;
    loop {
        to_visit.retain(|&u16_idx| {
            let idx = u16_idx as usize;
            let seen_from_here_count = visibility_map[idx]
                .iter()
                .filter(|&&idx| seats[idx as usize])
                .count();

            // Free seat that is now taken or ccupied seat that is now left:
            if (!seats[idx] && seen_from_here_count == 0)
                || (seats[idx] && seen_from_here_count >= leave_when_seeing)
            {
                changes.push(u16_idx);
                true
            } else {
                false
            }
        });

        changes.retain(|&change_idx| {
            seats[change_idx as usize] = !seats[change_idx as usize];
            false
        });

        if to_visit.is_empty() {
            return Ok(seats.iter().filter(|&&occupied| occupied).count());
        } else {
            iteration += 1;
            if iteration >= MAX_ITERATIONS {
                return Err(format!("Aborting after {} iterations", iteration));
            }
        }
    }
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
