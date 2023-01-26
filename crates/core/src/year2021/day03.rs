use crate::input::Input;

/// Constructs a u16 with the lowest n bits set.
///
/// Does not work for n == 0.
fn set_lowest_bits(n: u8) -> u16 {
    u16::MAX >> (u16::BITS as u16 - u16::from(n))
}

fn is_bit_mostly_set(numbers: &[u16], bit_idx: usize) -> bool {
    numbers.iter().fold(0, |acc, x| {
        acc + if x & (1 << bit_idx) == 0 { -1 } else { 1 }
    }) >= 0
}

fn keep_one_according_to_bitset_criteria(
    candidates: &mut [u16],
    num_bits: usize,
    criteria_wants_most: bool,
) -> Result<u16, String> {
    let mut num_candidates = candidates.len();
    for current_bit_idx in (0..num_bits).rev() {
        let mostly_set = is_bit_mostly_set(&candidates[0..num_candidates], current_bit_idx);
        let mut current_candidate_idx = 0;
        while current_candidate_idx < num_candidates {
            let is_bit_set = (candidates[current_candidate_idx] & (1 << current_bit_idx)) != 0;
            if (is_bit_set == mostly_set) == criteria_wants_most {
                current_candidate_idx += 1;
            } else {
                num_candidates -= 1;
                candidates.swap(current_candidate_idx, num_candidates);
            }
        }
        if num_candidates == 1 {
            return Ok(candidates[0]);
        }
    }

    Err("Bit criteria did not result in single number".to_string())
}

pub fn solve(input: &Input) -> Result<u32, String> {
    const MAX_BITS: usize = u16::BITS as usize;

    let num_bits = input.text.lines().next().map(str::len).unwrap_or_default();
    if num_bits == 0 || num_bits > MAX_BITS {
        return Err(format!(
            "Invalid number of bits - must be between 1 and {MAX_BITS}"
        ));
    }

    if input.is_part_one() {
        let set_bits_at_position = &mut [0; 16][0..num_bits];
        for line in input.text.lines() {
            if line.bytes().len() != num_bits {
                return Err("All lines does not have equal length".to_string());
            }
            for (idx, byte) in line.bytes().rev().enumerate() {
                set_bits_at_position[idx] += if byte == b'1' { 1 } else { -1 };
            }
        }

        let gamma: u16 = set_bits_at_position
            .iter()
            .enumerate()
            .filter_map(|(bit_idx, &count)| (count >= 0).then_some(1 << bit_idx))
            .sum();
        let epsilon = !gamma & set_lowest_bits(num_bits as u8);
        Ok(u32::from(gamma) * u32::from(epsilon))
    } else {
        let mut numbers = input
            .text
            .lines()
            .enumerate()
            .map(|(line_idx, line)| {
                u16::from_str_radix(line, 2)
                    .map_err(|_| format!("Line {}: Not a binary integer", line_idx + 1))
            })
            .collect::<Result<Vec<u16>, _>>()?;

        let oxygen_generator_rating =
            keep_one_according_to_bitset_criteria(&mut numbers, num_bits, true)?;
        let co2_scrubber_rating =
            keep_one_according_to_bitset_criteria(&mut numbers, num_bits, false)?;
        Ok(u32::from(oxygen_generator_rating) * u32::from(co2_scrubber_rating))
    }
}

#[test]
fn test_set_bits() {
    assert_eq!(set_lowest_bits(1), 0b0001);
    assert_eq!(set_lowest_bits(2), 0b0011);
    assert_eq!(set_lowest_bits(3), 0b0111);
    assert_eq!(set_lowest_bits(16), u16::MAX);
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    test_part_one!(example => 198);
    test_part_two!(example => 230);

    let real_input = include_str!("day03_input.txt");
    test_part_one!(real_input => 3_985_686);
    test_part_two!(real_input => 2_555_739);
}
