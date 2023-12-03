use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAP_SIZE: usize = 256;
    let mut numbers = vec![0; MAP_SIZE * MAP_SIZE];

    for (row_idx, row_str) in input.text.lines().enumerate() {
        if row_idx >= MAP_SIZE || row_str.len() >= MAP_SIZE {
            return Err("Too big schematic".to_string());
        }

        let mut num_start_idx = None;
        for (col_idx, col_byte) in row_str.bytes().chain(std::iter::once(b'.')).enumerate() {
            if col_byte.is_ascii_digit() {
                if num_start_idx.is_none() {
                    num_start_idx = Some(col_idx);
                }
            } else if let Some(start_idx) = num_start_idx {
                let num_str = &row_str[start_idx..col_idx];
                let num = num_str
                    .parse::<u16>()
                    .map_err(|_| "Invalid number".to_string())?;
                for i in start_idx..col_idx {
                    numbers[row_idx * MAP_SIZE + i] = num;
                }
                num_start_idx = None;
            }
        }
    }

    let mut sum = 0;

    for (row_idx, row_str) in input.text.lines().enumerate() {
        for (col_idx, col_byte) in row_str.bytes().chain(std::iter::once(b'.')).enumerate() {
            let interesting = if input.is_part_one() {
                !col_byte.is_ascii_digit() && col_byte != b'.'
            } else {
                col_byte == b'*'
            };
            if interesting {
                let mut num_neighbour_count = 0;
                let mut this_value = input.part_values(0, 1);
                for dx in -1_i32..=1 {
                    for dy in -1_i32..=1 {
                        if (col_idx, dx) == (0, -1)
                            || (row_idx, dy) == (0, -1)
                            || (dx == 0 && dy == 0)
                        {
                            continue;
                        }
                        let x = col_idx as i32 + dx;
                        let y = row_idx as i32 + dy;
                        if matches!((dx, dy), (0 | 1, -1 | 1))
                            && numbers[(y as usize) * MAP_SIZE + (x - 1) as usize] != 0
                        {
                            // Avoid counting same number multiple times.
                            continue;
                        }
                        let adjacent_num = numbers[y as usize * MAP_SIZE + x as usize];
                        if input.is_part_one() {
                            this_value += u64::from(adjacent_num);
                        } else if adjacent_num != 0 {
                            num_neighbour_count += 1;
                            this_value *= u64::from(adjacent_num);
                        }
                    }
                }
                sum += if input.is_part_one() || (num_neighbour_count == 2) {
                    this_value
                } else {
                    0
                };
            }
        }
    }

    Ok(sum)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_num_allocations, test_part_two_num_allocations};

    let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    test_part_one_num_allocations!(test_input => 4361, 1);
    test_part_two_num_allocations!(test_input => 467_835, 1);

    let real_input = include_str!("day03_input.txt");
    test_part_one_num_allocations!(real_input => 546_563, 1);
    test_part_two_num_allocations!(real_input => 91_031_374, 1);
}
