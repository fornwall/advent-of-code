use crate::input::Input;

fn compute_checksum(input: &str) -> String {
    let candidate = input
        .as_bytes()
        .chunks(2)
        .map(|chunk| if chunk[0] == chunk[1] { '1' } else { '0' })
        .collect::<String>();
    if candidate.len() % 2 == 0 {
        // "If the length of the checksum is even, repeat the process until you end up with a checksum with an odd length."
        compute_checksum(&candidate)
    } else {
        candidate
    }
}

pub fn solve(input: &Input) -> Result<String, String> {
    let disk_length = input.part_values(272, 35_651_584);

    let mut a = input.text.to_string();

    while a.len() < disk_length {
        let b = a
            .chars()
            .rev()
            .map(|c| if c == '0' { '1' } else { '0' })
            .collect::<String>();
        a = a + "0" + &b;
    }

    // "Calculate the checksum only for the data that fits on the disk,
    // even if you generated more data than that in the previous step."
    a = a[0..disk_length].to_string();

    Ok(compute_checksum(&a))
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day16_input.txt");
    test_part_one!(real_input => "11100110111101110".to_string());
    test_part_two!(real_input => "10001101010000101".to_string());
}
