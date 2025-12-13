use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut result = 0;
    let num_digits = input.part_values(2, 12);

    for line in input.text.lines() {
        let bytes = line.as_bytes();
        let len = bytes.len();

        let mut current_offset = 0;
        for digit_offset_from_right in (0..num_digits).rev() {
            let (picked_digit, new_offset) = bytes[current_offset..(len - digit_offset_from_right)]
                .iter()
                .enumerate()
                .fold((0, 0), |(max_digit, max_idx), (idx, &d)| {
                    let d = d - b'0';
                    if d > max_digit {
                        (d, current_offset + idx + 1)
                    } else {
                        (max_digit, max_idx)
                    }
                });
            result += (10_u64.pow(digit_offset_from_right as u32)) * u64::from(picked_digit);
            current_offset = new_offset;
        }
    }
    Ok(result)
}

#[test]
pub fn tests() {
    let test_input = "987654321111111
811111111111119
234234234234278
818181911112111";
    test_part_one_no_allocations!(test_input => 357);
    test_part_two_no_allocations!(test_input => 3_121_910_778_619);

    let real_input = include_str!("day03_input.txt");
    test_part_one_no_allocations!(real_input => 16_887);
    test_part_two_no_allocations!(real_input => 167_302_518_850_275);
}
