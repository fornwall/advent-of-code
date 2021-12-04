#[cfg(feature = "visualization")]
use super::day01_renderer::{render_part_one, render_part_two};
use crate::common::parser::parse_lines;
use crate::input::Input;
use core::cmp::Ordering::{Equal, Greater, Less};

fn subsequence_summing_to(sorted_sequence: &[u32], desired_sum: u32) -> Option<u32> {
    if sorted_sequence.is_empty() || sorted_sequence.len() < 2 {
        return None;
    }

    let (mut left, mut right) = (0, sorted_sequence.len() - 1);

    while left != right {
        let (left_value, right_value) = (sorted_sequence[left], sorted_sequence[right]);
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

    let mut expenses = parse_lines::<u32>(input.text)?;
    expenses.sort_unstable();

    let result = if input.is_part_one() {
        #[cfg(feature = "visualization")]
        render_part_one(&mut expenses, &mut input.painter);
        subsequence_summing_to(&expenses, DESIRED_SUM)
    } else {
        #[cfg(feature = "visualization")]
        render_part_two(&mut expenses, &mut input.painter);
        expenses
            .iter()
            .enumerate()
            .find_map(|(left_index, &left_value)| {
                if left_value > DESIRED_SUM {
                    return None;
                }
                let desired_sub_sum = DESIRED_SUM - left_value;
                subsequence_summing_to(&expenses[(left_index + 1)..], desired_sub_sum)
                    .map(|value| value * left_value)
            })
    };

    result.ok_or_else(|| {
        format!(
            "No {} expenses sum to {}",
            input.part_values(2, 3),
            DESIRED_SUM
        )
    })
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two, test_part_two_error};

    test_part_one!("1721\n979\n366\n299\n675\n1456" => 514_579);
    test_part_one_error!("" => "No 2 expenses sum to 2020");
    test_part_one_error!("1" => "No 2 expenses sum to 2020");
    test_part_one_error!("1\n2" => "No 2 expenses sum to 2020");
    test_part_one_error!("1\n2\n3" => "No 2 expenses sum to 2020");

    test_part_two!("1721\n979\n366\n299\n675\n1456" => 241_861_950);
    test_part_two_error!("asdf" => "Line 1: Not a valid integer");
    test_part_two_error!("12\nasdf" => "Line 2: Not a valid integer");
    test_part_two_error!("" => "No 3 expenses sum to 2020");
    test_part_two_error!("1" => "No 3 expenses sum to 2020");
    test_part_two_error!("1\n2" => "No 3 expenses sum to 2020");
    test_part_two_error!("1\n2\n3" => "No 3 expenses sum to 2020");

    let real_input = include_str!("day01_input.txt");
    test_part_one!(real_input => 138_379);
    test_part_two!(real_input => 85_491_920);
}
