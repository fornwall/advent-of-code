use std::collections::HashSet;

pub fn part1(input_string: &str) -> String {
    let result: i32 = input_string
        .replace(',', "\n")
        .lines()
        .map(|w| w.parse::<i32>().unwrap())
        .sum();

    result.to_string()
}

pub fn part2(input_string: &str) -> String {
    let input: Vec<i32> = input_string
        .replace(',', "\n")
        .lines()
        .map(|w| w.parse::<i32>().unwrap())
        .collect();

    let mut i = 0;
    let mut frequency = 0;
    let mut seen_frequencies = HashSet::new();

    while seen_frequencies.insert(frequency) {
        frequency += input[i];
        i = (i + 1) % input.len();
    }

    frequency.to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    pub fn part2_scan(input_string: &str) -> String {
        let mut seen_frequencies = HashSet::new();
        seen_frequencies.insert(0);

        input_string
            .replace(',', "\n")
            .lines()
            .map(|w| w.parse::<i32>().unwrap())
            .cycle()
            .scan(0, |frequency, change| {
                *frequency += change;
                Some(*frequency)
            })
            .find(|frequency| !seen_frequencies.insert(*frequency))
            .unwrap()
            .to_string()
    }
}

#[test]
pub fn tests_part1() {
    assert_eq!("3", part1("+1,-2,+3,+1"));
    assert_eq!("3", part1("+1,+1,+1"));
    assert_eq!("0", part1("+1,+1,-2"));
    assert_eq!("-6", part1("-1,-2,-3"));

    assert_eq!("477", part1(include_str!("day01_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!("0", part2("+1,-1"));
    assert_eq!("10", part2("+3,+3,+4,-2,-4"));
    assert_eq!("5", part2("-6,+3,+8,+5,-6"));
    assert_eq!("14", part2("+7,+7,-2,-7,-4"));

    assert_eq!("390", part2(include_str!("day01_input.txt")));

    assert_eq!("0", tests::part2_scan("+1,-1"));
    assert_eq!("10", tests::part2_scan("+3,+3,+4,-2,-4"));
    assert_eq!("5", tests::part2_scan("-6,+3,+8,+5,-6"));
    assert_eq!("14", tests::part2_scan("+7,+7,-2,-7,-4"));
}
