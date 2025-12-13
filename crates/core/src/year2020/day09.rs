use crate::common::parser::parse_lines;
use crate::input::Input;

/// Search for a subsequence which sums to the desired sum.
fn subsequence_summing_to<T>(sequence: &[T], desired_sum: T) -> Option<&[T]>
where
    T: std::ops::AddAssign + Copy + PartialEq + PartialOrd + std::ops::SubAssign,
{
    let mut window_start = 0;
    let mut window_sum = sequence[window_start];

    for window_end in 1..sequence.len() {
        if window_sum == desired_sum {
            return Some(&sequence[window_start..window_end]);
        }

        window_sum += sequence[window_end];

        // Shorten window from the left as long as the current sum is too high:
        while window_sum > desired_sum && window_start < window_end - 1 {
            window_sum -= sequence[window_start];
            window_start += 1;
        }
    }

    None
}

pub fn solve(input: &Input) -> Result<u64, String> {
    const PREAMBLE_LENGTH: usize = 25;

    let numbers = parse_lines::<u64>(input.text)?;

    if numbers.len() <= PREAMBLE_LENGTH {
        return Err(format!("Too few input numbers ({})", numbers.len()));
    }

    let invalid_number = numbers
        .iter()
        .enumerate()
        .skip(PREAMBLE_LENGTH)
        .find_map(|(idx, &number)| {
            for j in (idx - PREAMBLE_LENGTH)..idx {
                for k in j + 1..idx {
                    if numbers[j] + numbers[k] == number {
                        return None;
                    }
                }
            }
            Some(number)
        })
        .ok_or_else(|| "No invalid number".to_string())?;

    if input.is_part_one() {
        return Ok(invalid_number);
    }

    subsequence_summing_to(&numbers, invalid_number)
        .map(|subsequence| {
            let (min, max) = subsequence
                .iter()
                .fold((u64::MAX, u64::MIN), |(min, max), &number| {
                    (std::cmp::min(min, number), std::cmp::max(max, number))
                });
            min + max
        })
        .ok_or_else(|| format!("No contiguous set summing to {invalid_number}"))
}

#[test]
pub fn tests() {
    let real_input = include_str!("day09_input.txt");
    test_part_one!(real_input => 50_047_984);
    test_part_two!(real_input => 5_407_707);
}
