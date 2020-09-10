use std::cmp::Ordering;

pub fn part1(input_string: &str) -> String {
    let mut parts = input_string.trim().split('-');
    let from = parts.next().unwrap().parse::<i32>().unwrap();
    let to = parts.next().unwrap().parse::<i32>().unwrap();

    let mut meeting_criteria_count = 0;
    'outer: for i in from..=to {
        let mut divider = 1;
        let mut last_digit = 10;
        let mut two_digits_adjacent = false;

        while divider <= 100_000 {
            let digit = (i / divider) % 10;
            match digit.cmp(&last_digit) {
                Ordering::Greater => {
                    continue 'outer;
                }
                Ordering::Equal => {
                    two_digits_adjacent = true;
                }
                _ => {}
            }

            last_digit = digit;
            divider *= 10;
        }
        if two_digits_adjacent {
            meeting_criteria_count += 1;
        }
    }

    meeting_criteria_count.to_string()
}

pub fn part2(input_string: &str) -> String {
    let mut parts = input_string.trim().split('-');
    let from = parts.next().unwrap().parse::<i32>().unwrap();
    let to = parts.next().unwrap().parse::<i32>().unwrap();

    let mut meeting_criteria_count = 0;
    'outer: for i in from..=to {
        let mut divider = 1;
        let mut last_digit = 10;
        let mut two_digits_adjacent = false;
        let mut digits_adjacent_streak = 1;

        while divider <= 100_000 {
            let digit = (i / divider) % 10;
            match digit.cmp(&last_digit) {
                Ordering::Greater => {
                    continue 'outer;
                }
                Ordering::Equal => {
                    digits_adjacent_streak += 1;
                }
                Ordering::Less => {
                    if digits_adjacent_streak == 2 {
                        two_digits_adjacent = true;
                    }
                    digits_adjacent_streak = 1;
                }
            }

            last_digit = digit;
            divider *= 10;
        }

        if digits_adjacent_streak == 2 {
            two_digits_adjacent = true;
        }

        if two_digits_adjacent {
            meeting_criteria_count += 1;
        }
    }

    meeting_criteria_count.to_string()
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1("100010-100011"), "0");
    assert_eq!(part1("111110-111111"), "1");

    assert_eq!(part1(include_str!("day04_input.txt")), "1675");
}

#[test]
fn tests_part2() {
    assert_eq!(part2("112233-112233"), "1");
    assert_eq!(part2("123444-123444"), "0");
    assert_eq!(part2("111122-111122"), "1");

    assert_eq!(part2(include_str!("day04_input.txt")), "1142");
}
