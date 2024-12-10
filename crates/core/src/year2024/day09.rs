use crate::common::array_stack::ArrayStack;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    let mut checksum = 0;
    let blocks = input.text.as_bytes();

    if input.is_part_one() {
        let mut left_block_offset = 0;
        let mut left_digit_offset = 0;
        let mut left_num_free = 0;
        let mut right_digit_offset = blocks.len() - 1;

        loop {
            if right_digit_offset % 2 == 1 {
                right_digit_offset -= 1;
            }
            if right_digit_offset <= left_digit_offset {
                if right_digit_offset == left_digit_offset {
                    let num_occupied_blocks = blocks[left_digit_offset] - b'0';
                    let file_id = left_digit_offset / 2;
                    for _ in 0..num_occupied_blocks {
                        checksum += left_block_offset * file_id;
                        left_block_offset += 1;
                    }
                }
                break;
            }

            let file_id = right_digit_offset / 2;
            let mut blocks_to_move = blocks[right_digit_offset] - b'0';
            right_digit_offset -= 1;

            while blocks_to_move > 0 {
                let can_place = blocks_to_move.min(left_num_free);
                for _ in 0..can_place {
                    checksum += left_block_offset * file_id;
                    left_block_offset += 1;
                    blocks_to_move -= 1;
                    left_num_free -= 1;
                }

                if blocks_to_move > 0 {
                    let num_occupied_blocks = blocks[left_digit_offset] - b'0';
                    let file_id = left_digit_offset / 2;
                    for _ in 0..num_occupied_blocks {
                        checksum += left_block_offset * file_id;
                        left_block_offset += 1;
                    }
                    left_digit_offset += 1;

                    left_num_free = blocks[left_digit_offset] - b'0';
                    left_digit_offset += 1;
                    if right_digit_offset <= left_digit_offset {
                        left_num_free = u8::MAX;
                    }
                }
            }
        }
    } else {
        let mut file_blocks = ArrayStack::<10_000, (usize, usize, usize)>::new();
        let mut free_blocks = ArrayStack::<10_000, (usize, usize)>::new();
        let mut first_free_indexed_by_block_size = [usize::MAX; 9];

        let mut block_offset = 0;
        for (i, &block_digit) in blocks.iter().enumerate() {
            let block_size = (block_digit - b'0') as usize;
            if i % 2 == 0 {
                let file_id = i / 2;
                file_blocks.push((file_id, block_size, block_offset))?;
            } else {
                let new_free_block_idx = free_blocks.len();
                for b in first_free_indexed_by_block_size
                    .iter_mut()
                    .take(block_size)
                    .filter(|b| new_free_block_idx < **b)
                {
                    *b = new_free_block_idx;
                }
                free_blocks.push((block_size, block_offset))?;
            }
            block_offset += block_size;
        }

        for &(file_id, file_size, file_block_idx) in file_blocks.slice().iter().rev() {
            let mut final_block_idx = file_block_idx;

            let free_idx = first_free_indexed_by_block_size[file_size - 1];
            if free_idx < free_blocks.len() {
                let (free_block_size, free_block_idx) = free_blocks.elements[free_idx];
                let new_free_block_size = free_block_size - file_size;
                free_blocks.elements[free_idx] = (new_free_block_size, free_block_idx + file_size);

                for (i, b) in first_free_indexed_by_block_size
                    .iter_mut()
                    .enumerate()
                    .take(free_block_size)
                    .skip(new_free_block_size)
                    .filter(|(_i, b)| **b == free_idx)
                {
                    let start_search_idx = free_idx + 1;
                    *b = free_blocks.slice()[start_search_idx..]
                        .iter()
                        .position(|&(size, _)| size > i)
                        .unwrap_or(usize::MAX)
                        .saturating_add(start_search_idx);
                }

                final_block_idx = free_block_idx;
            }
            checksum += file_id * (final_block_idx..(final_block_idx + file_size)).sum::<usize>();
            free_blocks.pop();
        }
    }

    Ok(checksum)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "02202"; // => ..1122 => 2211 => 0*2 + 1*2 + 2*1 + 3*1
    test_part_one_no_allocations!(test_input => 7);

    let test_input = "2333133121414131402";
    test_part_one_no_allocations!(test_input => 1928);
    test_part_two_no_allocations!(test_input => 2858);

    let real_input = include_str!("day09_input.txt");
    test_part_one_no_allocations!(real_input => 6_356_833_654_075);
    test_part_two_no_allocations!(real_input => 6_389_911_791_746);
}
