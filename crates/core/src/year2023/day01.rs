use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    Ok(input
        .text
        .lines()
        .map(|line| calibration_value(line, input.is_part_two()))
        .sum())
}

fn calibration_value(line: &str, part2: bool) -> u32 {
    let mut first_digit = None;
    let mut last_digit = 0;

    let mut bits = 0_u64;
    for byte in line.bytes() {
        bits = bits << 8 | u64::from(byte);
        for &(check_bits, val, mask) in [
            (u64::from(b'1'), 1_u32, 0xff),
            (u64::from(b'2'), 2, 0xff),
            (u64::from(b'3'), 3, 0xff),
            (u64::from(b'4'), 4, 0xff),
            (u64::from(b'5'), 5, 0xff),
            (u64::from(b'6'), 6, 0xff),
            (u64::from(b'7'), 7, 0xff),
            (u64::from(b'8'), 8, 0xff),
            (u64::from(b'9'), 9, 0xff),
            (
                u64::from(b'o') << 16 | u64::from(b'n') << 8 | u64::from(b'e'),
                1,
                0x00ff_ffff,
            ),
            (
                u64::from(b't') << 16 | u64::from(b'w') << 8 | u64::from(b'o'),
                2,
                0x00ff_ffff,
            ),
            (
                u64::from(b't') << 32
                    | u64::from(b'h') << 24
                    | u64::from(b'r') << 16
                    | u64::from(b'e') << 8
                    | u64::from(b'e'),
                3,
                0xff_ffff_ffff,
            ),
            (
                u64::from(b'f') << 24
                    | u64::from(b'o') << 16
                    | u64::from(b'u') << 8
                    | u64::from(b'r'),
                4,
                0xffff_ffff,
            ),
            (
                u64::from(b'f') << 24
                    | u64::from(b'i') << 16
                    | u64::from(b'v') << 8
                    | u64::from(b'e'),
                5,
                0xffff_ffff,
            ),
            (
                u64::from(b's') << 16 | u64::from(b'i') << 8 | u64::from(b'x'),
                6,
                0xff_ffff,
            ),
            (
                u64::from(b's') << 32
                    | u64::from(b'e') << 24
                    | u64::from(b'v') << 16
                    | u64::from(b'e') << 8
                    | u64::from(b'n'),
                7,
                0xff_ffff_ffff,
            ),
            (
                u64::from(b'e') << 32
                    | u64::from(b'i') << 24
                    | u64::from(b'g') << 16
                    | u64::from(b'h') << 8
                    | u64::from(b't'),
                8,
                0xff_ffff_ffff,
            ),
            (
                u64::from(b'n') << 24
                    | u64::from(b'i') << 16
                    | u64::from(b'n') << 8
                    | u64::from(b'e'),
                9,
                0xffff_ffff,
            ),
        ]
        .iter()
        .take(if part2 { 18 } else { 9 })
        {
            if mask & bits == check_bits {
                if first_digit.is_none() {
                    first_digit = Some(val);
                }
                last_digit = val;
            }
        }
    }

    first_digit.unwrap_or_default() * 10 + last_digit
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    test_part_one_no_allocations!(test_input => 142);

    let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
five3threeqgtwone
twone
oneight
nine
ninine
eightwo";
    test_part_two_no_allocations!(test_input => 281 + 51 + 21 + 18 + 99 + 99 + 82);
    let test_input = "cneightwotdkfxxxjfdpz3zkkthree";
    test_part_two_no_allocations!(test_input => 83);

    let real_input = include_str!("day01_input.txt");
    test_part_one_no_allocations!(real_input => 55386);
    test_part_two_no_allocations!(real_input => 54824);
}
