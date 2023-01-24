use crate::common::parser::parse_lines;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    const TARGET_SIZE: u8 = 150;

    let container_sizes = parse_lines::<u8>(input.text)?;

    if input.is_part_one() {
        let mut answers = vec![0; (TARGET_SIZE + 1) as usize];
        answers[0] = 1;
        for container_size in container_sizes {
            for next_size in (container_size..=TARGET_SIZE).rev() {
                answers[next_size as usize] += answers[(next_size - container_size) as usize];
            }
        }
        Ok(answers[TARGET_SIZE as usize])
    } else {
        let all_bits = u32::MAX >> (32 - container_sizes.len());

        for containers_to_use in 1..=container_sizes.len() {
            let possible_combinations = (1..=all_bits)
                .filter(|bit_mask| {
                    if bit_mask.count_ones() != containers_to_use as u32 {
                        return false;
                    }
                    let total_size: u32 = container_sizes
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, &size)| {
                            (((1 << idx) & bit_mask) != 0).then_some(u32::from(size))
                        })
                        .sum();
                    total_size == u32::from(TARGET_SIZE)
                })
                .count();
            if possible_combinations != 0 {
                return Ok(possible_combinations as u32);
            }
        }

        Err("No solution found".to_string())
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day17_input.txt");
    test_part_one!(real_input => 1304);
    test_part_two!(real_input => 18);
}
