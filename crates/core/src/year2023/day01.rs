use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    Ok(input
        .text
        .lines()
        .map(|line| calibration_value(line, input.is_part_two()))
        .sum())
}

fn calibration_value(line: &str, part2: bool) -> u32 {
    let (mut first_digit, mut last_digit) = (0, 0);

    let mut bits = 0_u64;
    for byte in line.bytes() {
        bits = bits << 8 | u64::from(byte);

        let (m1, m2, m3) = if part2 {
            (0x00ff_ffff, 0xffff_ffff, 0xff_ffff_ffff)
        } else {
            (0, 0, 0)
        };

        #[allow(clippy::unreadable_literal)]
        let digit = match (bits & 0xff, bits & m1, bits & m2, bits & m3) {
            (0b110001, _, _, _) | (_, 0b11011110110111001100101, _, _) => 1,
            (0b110010, _, _, _) | (_, 0b11101000111011101101111, _, _) => 2,
            (0b110011, _, _, _) | (_, _, _, 0b111010001101000011100100110010101100101) => 3,
            (0b110100, _, _, _) | (_, _, 0b1100110011011110111010101110010, _) => 4,
            (0b110101, _, _, _) | (_, _, 0b1100110011010010111011001100101, _) => 5,
            (0b110110, _, _, _) | (_, 0b11100110110100101111000, _, _) => 6,
            (0b110111, _, _, _) | (_, _, _, 0b111001101100101011101100110010101101110) => 7,
            (0b111000, _, _, _) | (_, _, _, 0b110010101101001011001110110100001110100) => 8,
            (0b111001, _, _, _) | (_, _, 0b1101110011010010110111001100101, _) => 9,
            _ => {
                continue;
            }
        };
        if first_digit == 0 {
            first_digit = digit;
        }
        last_digit = digit;
    }

    first_digit * 10 + last_digit
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
