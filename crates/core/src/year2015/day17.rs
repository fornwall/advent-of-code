use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    const TARGET_SIZE: u8 = 150;

    let container_sizes = input
        .text
        .lines()
        .map(|line| line.parse::<u8>().map_err(|_| "Invalid container size"))
        .collect::<Result<Vec<u8>, _>>()?;

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
        let mut all_bits = 0_u32;
        for i in 0..container_sizes.len() {
            all_bits |= 1 << i;
        }
        for containers_to_use in 1..=container_sizes.len() {
            let mut possible_combinations = 0;
            for bit_mask in 1..=all_bits {
                if bit_mask.count_ones() == containers_to_use as u32 {
                    let total_size: u32 = container_sizes
                        .iter()
                        .map(|&size| u32::from(size))
                        .enumerate()
                        .filter(|&(idx, _size)| ((1 << idx) & bit_mask) > 0)
                        .map(|(_idx, size)| size)
                        .sum();
                    if total_size == u32::from(TARGET_SIZE) as u32 {
                        possible_combinations += 1;
                    }
                }
            }
            if possible_combinations != 0 {
                return Ok(possible_combinations);
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
