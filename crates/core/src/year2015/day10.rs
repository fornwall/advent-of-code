use crate::common::int_to_ascii::IntToAsciiContext;
use crate::input::Input;

fn read_string(s: &[u8]) -> Vec<u8> {
    let mut ascii_bytes_context = IntToAsciiContext::new();
    let mut result = Vec::new();
    let mut last_digit = s[0];
    let mut num_digits = 1;
    for &digit in &s[1..] {
        if last_digit == digit {
            num_digits += 1;
        } else {
            result.extend_from_slice(ascii_bytes_context.ascii_bytes(num_digits));
            result.extend_from_slice(ascii_bytes_context.ascii_bytes(u32::from(last_digit - 48)));
            last_digit = digit;
            num_digits = 1;
        }
    }

    result.extend_from_slice(ascii_bytes_context.ascii_bytes(num_digits));
    result.extend_from_slice(ascii_bytes_context.ascii_bytes(u32::from(last_digit - 48)));

    result
}

pub fn solve(input: &Input) -> Result<u32, String> {
    if input.text.len() >= 16 {
        return Err("Too long input - max length is 16".to_string());
    } else if !input.text.chars().all(|c| c.is_ascii_digit()) {
        return Err("Input is not ASCII digits".to_string());
    }

    let mut s = input.text.as_bytes().to_vec();
    for _ in 0..input.part_values(40, 50) {
        s = read_string(&s);
    }
    Ok(s.len() as u32)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};

    assert_eq!(read_string(b"1"), b"11".to_vec());
    assert_eq!(read_string(b"11"), b"21".to_vec());
    assert_eq!(read_string(b"1211"), b"111221".to_vec());

    test_part_one_error!("+," => "Input is not ASCII digits");

    let real_input = include_str!("day10_input.txt");
    test_part_one!(real_input => 252_594);
    test_part_two!(real_input => 3_579_328);
}
