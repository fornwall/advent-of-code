use std::iter::once;

pub fn part1(input_string: &str) -> String {
    let mut digits: Vec<i32> = input_string.bytes().map(|b| (b - 48) as i32).collect();
    let mut new_digits = vec![0; digits.len()];
    for _ in 0..100 {
        for (index, digit) in new_digits.iter_mut().enumerate() {
            *digit = digits
                .iter()
                .zip(
                    once(0)
                        .cycle()
                        .take(index + 1)
                        .chain(once(1).cycle().take(index + 1))
                        .chain(once(0).cycle().take(index + 1))
                        .chain(once(-1).cycle().take(index + 1))
                        .cycle()
                        .skip(1),
                )
                .fold(0, |acc, (&digit, mult)| acc + digit * mult)
                .abs()
                % 10;
        }

        std::mem::swap(&mut digits, &mut new_digits);
    }

    digits
        .iter()
        .take(8)
        .map(|b| ((b + 48) as u8) as char)
        .collect()
}

pub fn part2(_input_string: &str) -> String {
    String::from("")
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1("80871224585914546619083218645595"), "24176176");
    assert_eq!(part1(include_str!("day16_input.txt")), "37153056");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(""), "");

    // assert_eq!(part2(include_str!("day16_input.txt")), "");
}
