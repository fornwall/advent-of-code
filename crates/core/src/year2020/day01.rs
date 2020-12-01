use crate::input::Input;
use core::cmp::Ordering::{Equal, Greater, Less};

fn subsequence_summing_to(sorted_sequence: &[u32], desired_sum: u32) -> Option<u32> {
    if sorted_sequence.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = sorted_sequence.len() - 1;

    while left != right {
        let left_value = sorted_sequence[left];
        let right_value = sorted_sequence[right];

        let candidate_sum = left_value + right_value;

        match candidate_sum.cmp(&desired_sum) {
            Equal => {
                return Some(left_value * right_value);
            }
            Less => {
                left += 1;
            }
            Greater => {
                right -= 1;
            }
        }
    }

    None
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    const DESIRED_SUM: u32 = 2020;

    let mut expenses = input
        .text
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            line.parse::<u32>().map_err(|parse_error| {
                format!(
                    "Line {}: Cannot parse expense ({})",
                    line_index + 1,
                    parse_error.to_string()
                )
            })
        })
        .collect::<Result<Vec<u32>, String>>()?;

    expenses.sort_unstable();

    if input.is_part_one() {
        if let Some(value) = subsequence_summing_to(&expenses, DESIRED_SUM) {
            return Ok(value);
        }
    } else {
        for (left_index, left_value) in expenses.iter().enumerate() {
            let desired_sub_sum = DESIRED_SUM - left_value;
            if let Some(value) =
                subsequence_summing_to(&expenses[(left_index + 1)..], desired_sub_sum)
            {
                return Ok(left_value * value);
            }
        }
    }

    Err(format!(
        "No {} expenses sum to {}",
        input.part_values(2, 3),
        DESIRED_SUM
    ))
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_one_error, test_part_two, test_part_two_error};

    test_part_one!("1721\n979\n366\n299\n675\n1456" => 514579);
    test_part_one_error!("" => "No 2 expenses sum to 2020");
    test_part_one_error!("1" => "No 2 expenses sum to 2020");
    test_part_one_error!("1\n2" => "No 2 expenses sum to 2020");
    test_part_one_error!("1\n2\n3" => "No 2 expenses sum to 2020");

    test_part_two!("1721\n979\n366\n299\n675\n1456" => 241861950);
    test_part_two_error!("asdf" => "Line 1: Cannot parse expense (invalid digit found in string)");
    test_part_two_error!("12\nasdf" => "Line 2: Cannot parse expense (invalid digit found in string)");
    test_part_two_error!("" => "No 3 expenses sum to 2020");
    test_part_two_error!("1" => "No 3 expenses sum to 2020");
    test_part_two_error!("1\n2" => "No 3 expenses sum to 2020");
    test_part_two_error!("1\n2\n3" => "No 3 expenses sum to 2020");

    let real_input = include_str!("day01_input.txt");
    test_part_one!(real_input => 121396);
    test_part_two!(real_input => 73616634);
}
