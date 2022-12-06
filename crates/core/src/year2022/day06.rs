use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    let transmission = input.text.as_bytes();
    let packet_len = input.part_values(4, 14);

    if transmission.iter().any(|b| !b.is_ascii_lowercase()) {
        return Err("Input is not lower case characters".to_string());
    }

    transmission
        .windows(packet_len)
        .enumerate()
        .find_map(|(window_idx, window)| {
            let distinct_count = window
                .iter()
                .fold(0_u32, |acc, x| acc | 1 << (x - b'a'))
                .count_ones() as usize;

            (distinct_count == packet_len).then_some(window_idx + packet_len)
        })
        .ok_or_else(|| "No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};

    let test_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    test_part_one!(test_input => 7);
    test_part_two!(test_input => 19);

    let real_input = include_str!("day06_input.txt");
    test_part_one!(real_input => 1109);
    test_part_two!(real_input => 3965);

    test_part_one_error!("abc" => "No solution found");
    test_part_one_error!("abcc" => "No solution found");
}
