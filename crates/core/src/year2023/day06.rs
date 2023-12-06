use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut lines = input.text.lines();
    let (l1, l2) = (
        lines.next().ok_or_else(on_error)?,
        lines.next().ok_or_else(on_error)?,
    );

    Ok(if input.is_part_one() {
        let l1 = l1.split_ascii_whitespace();
        let l2 = l2.split_ascii_whitespace();
        l1.zip(l2)
            .skip(1)
            .map(num_wins)
            .reduce(|acc, x| acc * x)
            .unwrap_or_default()
    } else {
        num_wins((l1, l2))
    })
}

fn num_wins((duration, record): (&str, &str)) -> u64 {
    let (duration, record) = (parse_digits(duration), parse_digits(record));
    // With hold_time being the number of ms the button is held,
    // which results in a speed of hold_time, the travel distance is:
    //   travel_distance = (duration - hold_time) * hold_time
    // We need `travel_distance > record`:
    //   duration * hold_time - hold_time^2 > record
    // =>
    //   hold_time^2 - duration * hold_time + record < 0
    // => (quadratic equation) for boundary points:
    let d2 = duration / 2.;
    let sqrt = d2.mul_add(d2, -record).sqrt();
    let max = (d2 + sqrt - 1.).ceil();
    let min = (d2 - sqrt + 1.).floor();
    (max - min + 1.).round() as u64
}

fn parse_digits(s: &str) -> f64 {
    s.bytes().fold(0, |acc, x| {
        if x.is_ascii_digit() {
            acc * 10 + u64::from(x - b'0')
        } else {
            acc
        }
    }) as f64
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "Time:      7  15   30\nDistance:  9  40  200";
    test_part_one_no_allocations!(test_input => 288);
    test_part_two_no_allocations!(test_input => 71503);

    let real_input = include_str!("day06_input.txt");
    test_part_one_no_allocations!(real_input => 503_424);
    test_part_two_no_allocations!(real_input => 32_607_562);
}
