use crate::Input;

fn read_string(s: &str) -> String {
    let mut result = String::new();
    let mut last_digit = None;
    let mut num_digits = 0;
    for digit in s.chars() {
        if last_digit == Some(digit) {
            num_digits += 1;
        } else {
            if let Some(last_digit_value) = last_digit {
                result.push_str(&num_digits.to_string());
                result.push_str(&last_digit_value.to_string());
            }
            last_digit = Some(digit);
            num_digits = 1;
        }
    }

    result.push_str(&num_digits.to_string());
    result.push_str(&last_digit.unwrap_or('0').to_string());

    result
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    if input.text.len() >= 16 {
        return Err("Too long input - max length is 16".to_string());
    }

    let mut s = input.text.to_string();
    for _ in 0..input.part_values(40, 50) {
        s = read_string(&s);
    }
    Ok(s.len() as u32)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    assert_eq!(read_string("1"), "11");
    assert_eq!(read_string("11"), "21");
    assert_eq!(read_string("1211"), "111221");

    let real_input = include_str!("day10_input.txt");
    test_part_one!(real_input => 252_594);
    test_part_two!(real_input => 3_579_328);
}
