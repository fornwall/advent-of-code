use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAX_WIDTH: usize = 150;

    let line_width = input
        .text
        .lines()
        .next()
        .ok_or_else(|| "Empty input".to_string())?
        .len();
    let num_lines = input.text.lines().count();
    if line_width > MAX_WIDTH || num_lines > MAX_WIDTH {
        return Err("Input too large".to_string());
    }

    let mut grid = [false; MAX_WIDTH * MAX_WIDTH];
    let mut grid_ng = [false; MAX_WIDTH * MAX_WIDTH];
    for (y, line) in input.text.lines().enumerate() {
        for (x, ch) in line.bytes().enumerate() {
            grid[y * line_width + x] = ch == b'@';
        }
    }

    let mut total_removed = 0;
    loop {
        let removed = evolve(&grid, &mut grid_ng, line_width, num_lines);
        total_removed += removed;
        if removed == 0 || input.is_part_one() {
            return Ok(total_removed);
        } else {
            std::mem::swap(&mut grid, &mut grid_ng);
        }
    }
}

fn evolve(grid: &[bool], grid_ng: &mut [bool], width: usize, height: usize) -> u64 {
    let mut num_changes = 0;
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            if !grid[index] {
                grid_ng[index] = false;
                continue;
            }

            let mut num_surrounding = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if (dx == 0 && dy == 0)
                        || nx < 0
                        || nx >= width as i32
                        || ny < 0
                        || ny >= height as i32
                    {
                        continue;
                    }
                    let index = ny as usize * width + nx as usize;
                    num_surrounding += u32::from(grid[index]);
                }
            }
            if num_surrounding < 4 {
                num_changes += 1;
                grid_ng[index] = false;
            } else {
                grid_ng[index] = true;
            }
        }
    }
    num_changes
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    test_part_one_no_allocations!(test_input => 13);
    test_part_two_no_allocations!(test_input => 43);

    let real_input = include_str!("day04_input.txt");
    test_part_one_no_allocations!(real_input => 1437);
    test_part_two_no_allocations!(real_input => 8765);
}
