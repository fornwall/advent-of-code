use crate::Input;
use std::collections::VecDeque;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let n = input
        .text
        .parse::<u32>()
        .map_err(|e| format!("Invalid number of elves: {}", e))?;

    // See https://www.youtube.com/watch?v=uCsD3ZGzMgE:
    if input.is_part_one() {
        for bit_offset in (0..32).rev() {
            let bit = 1 << bit_offset;
            if bit & n > 0 {
                let with_msb_cleared = n & !bit;
                let with_lsb_added = (with_msb_cleared << 1) | 1;
                return Ok(with_lsb_added);
            }
        }
    } else {
        // TODO: From https://pastebin.com/Zm7tLbAe, understand
        // TODO: Use a common divide_rounding_up() function?
        let mut v1: VecDeque<u32> = (1..(n + 1) / 2 + 1).collect();
        let mut v2: VecDeque<u32> = ((n + 1) / 2 + 1..(n + 1)).collect();
        loop {
            if v2.len() >= v1.len() {
                v2.pop_front();
                if v2.is_empty() {
                    return Ok(v1[0]);
                }
            } else {
                v1.pop_back();
            }
            v1.push_back(v2.pop_front().ok_or("Internal error: Empty v2")?);
            v2.push_back(v1.pop_front().ok_or("Internal error: Empty v1")?);
        }
    }

    Err("No solution for zero".to_string())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day19_input.txt");
    test_part_one!(real_input => 1_808_357);
    test_part_two!(real_input => 1_407_007);
}
