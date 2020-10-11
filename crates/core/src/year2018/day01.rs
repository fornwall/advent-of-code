use std::collections::HashSet;

fn parse_input(input_string: &str) -> Result<Vec<i32>, String> {
    input_string
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            line.parse::<i32>().map_err(|error| {
                format!(
                    "Invalid input on line {}: {}",
                    line_index + 1,
                    error.to_string()
                )
            })
        })
        .collect::<Result<_, _>>()
}

pub fn part1(input_string: &str) -> Result<i32, String> {
    Ok(parse_input(input_string)?.iter().sum())
}

pub fn part2(input_string: &str) -> Result<i32, String> {
    let input = parse_input(input_string)?;

    let mut i = 0;
    let mut frequency: i32 = 0;
    let mut seen_frequencies = HashSet::new();

    while seen_frequencies.insert(frequency) {
        frequency = frequency
            .checked_add(input[i])
            .ok_or("Too high frequency")?;
        i = (i + 1) % input.len();
    }

    Ok(frequency)
}

#[test]
pub fn tests_part1() {
    assert_eq!(Ok(3), part1("+1\n-2\n+3\n+1"));
    assert_eq!(Ok(3), part1("+1\n+1\n+1"));
    assert_eq!(Ok(0), part1("+1\n+1\n-2"));
    assert_eq!(Ok(-6), part1("-1\n-2\n-3"));
    assert_eq!(Ok(477), part1(include_str!("day01_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(0), part2("+1\n-1"));
    assert_eq!(Ok(10), part2("+3\n+3\n+4\n-2\n-4"));
    assert_eq!(Ok(5), part2("-6\n+3\n+8\n+5\n-6"));
    assert_eq!(Ok(14), part2("+7\n+7\n-2\n-7\n-4"));
    assert_eq!(Ok(390), part2(include_str!("day01_input.txt")));
}
