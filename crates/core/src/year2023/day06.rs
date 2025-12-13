use crate::input::{Input, on_error};

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
    // (quadratic equation, with fused multiply-add (FMA) / mul_add()) =>
    let b = duration / 2.;
    let sqrt = b.mul_add(b, -record).sqrt();
    // For floating points a, b, and the open interval [a, b],
    // the integers contained in the interval are: [a.floor() + 1, b.ceil() - 1].
    // The reason for floor() + 1 instead of ceil() (and similarly for b.ceil() - 1 vs floor())
    // is that with a or b being exact integer values, the integer values are not
    // contained in the interval.
    let min = (b - sqrt).floor() as u64 + 1;
    let max = (b + sqrt).ceil() as u64 - 1;
    // +1 to beat the record:
    max - min + 1
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
    let test_input = "Time:      7  15   30\nDistance:  9  40  200";
    test_part_one_no_allocations!(test_input => 288);
    test_part_two_no_allocations!(test_input => 71503);

    let real_input = include_str!("day06_input.txt");
    test_part_one_no_allocations!(real_input => 503_424);
    test_part_two_no_allocations!(real_input => 32_607_562);
}
