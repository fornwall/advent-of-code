use std::collections::HashSet;

fn parse_input(input_string: &str) -> Result<Vec<i32>, String> {
    let mut result = Vec::new();
    for (index, line) in input_string.replace(',', "\n").lines().enumerate() {
        let line_number = index + 1;
        let value = line.parse::<i32>().map_err(|error| {
            format!(
                "Invalid input on line {}: {}",
                line_number,
                error.to_string()
            )
        })?;
        result.push(value);
    }
    Ok(result)
}

pub fn part1(input_string: &str) -> Result<i32, String> {
    let input = parse_input(input_string)?;
    Ok(input.iter().sum())
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
    assert_eq!(Ok(3), part1("+1,-2,+3,+1"));
    assert_eq!(Ok(3), part1("+1,+1,+1"));
    assert_eq!(Ok(0), part1("+1,+1,-2"));
    assert_eq!(Ok(-6), part1("-1,-2,-3"));

    assert_eq!(Ok(477), part1(include_str!("day01_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(0), part2("+1,-1"));
    assert_eq!(Ok(10), part2("+3,+3,+4,-2,-4"));
    assert_eq!(Ok(5), part2("-6,+3,+8,+5,-6"));
    assert_eq!(Ok(14), part2("+7,+7,-2,-7,-4"));

    assert_eq!(Ok(390), part2(include_str!("day01_input.txt")));
}
