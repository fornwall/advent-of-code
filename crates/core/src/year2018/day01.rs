use crate::input::Input;
use std::collections::HashSet;

type Frequency = i32;

fn parse_frequency_changes(
    input_string: &str,
) -> impl Iterator<Item = Result<Frequency, String>> + Clone + '_ {
    input_string.lines().enumerate().map(|(line_index, line)| {
        line.parse::<Frequency>().map_err(|error| {
            format!(
                "Invalid input on line {}: {}",
                line_index + 1,
                error.to_string()
            )
        })
    })
}

pub fn solve(input: &mut Input) -> Result<Frequency, String> {
    const MAX_ITERATIONS: usize = 1_000_000;
    let change_iterator = parse_frequency_changes(input.text);

    if input.is_part_one() {
        change_iterator.sum::<Result<_, _>>()
    } else {
        let mut frequency: Frequency = 0;
        let mut seen_frequencies = HashSet::new();

        for change in change_iterator.cycle().take(MAX_ITERATIONS) {
            if seen_frequencies.insert(frequency) {
                frequency = frequency.checked_add(change?).ok_or("Too high frequency")?;
            } else {
                return Ok(frequency);
            }
        }

        Err(format!(
            "Frequency not repeated after {} iterations",
            MAX_ITERATIONS
        ))
    }
}

#[test]
pub fn test() {
    use crate::{test_part_one, test_part_two};

    test_part_one!("+1\n-2\n+3\n+1" => 3);
    test_part_one!("+1\n+1\n+1" => 3);
    test_part_one!("+1\n+1\n-2" => 0);
    test_part_one!("-1\n-2\n-3" => -6);

    test_part_two!("+1\n-1" => 0);
    test_part_two!("+3\n+3\n+4\n-2\n-4" => 10);
    test_part_two!("-6\n+3\n+8\n+5\n-6" => 5);
    test_part_two!("+7\n+7\n-2\n-7\n-4" => 14);

    let real_input = include_str!("day01_input.txt");
    test_part_one!(real_input => 477);
    test_part_two!(real_input => 390);
}
