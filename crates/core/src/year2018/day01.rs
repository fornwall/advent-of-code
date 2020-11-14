use std::collections::HashSet;

type Frequency = i32;

fn parse_frequency_changes<'a>(
    input_string: &'a str,
) -> impl Iterator<Item = Result<Frequency, String>> + Clone + 'a {
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

pub fn part1(input_string: &str) -> Result<Frequency, String> {
    Ok(parse_frequency_changes(input_string).sum::<Result<_, _>>()?)
}

pub fn part2(input_string: &str) -> Result<Frequency, String> {
    const MAX_ITERATIONS: usize = 1_000_000;

    let mut frequency: Frequency = 0;
    let mut seen_frequencies = HashSet::new();

    for change in parse_frequency_changes(input_string)
        .cycle()
        .take(MAX_ITERATIONS)
    {
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
