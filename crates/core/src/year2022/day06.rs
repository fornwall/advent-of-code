use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let transmission = input.text.as_bytes();
    let packet_len = input.part_values(4, 14);

    if transmission.iter().any(|b| !b.is_ascii_lowercase()) {
        return Err("Input is not lower case characters".to_string());
    } else if transmission.len() < packet_len {
        return Err("Input too small".to_string());
    }

    for i in 0..transmission.len() - packet_len {
        let distinct_count = transmission[i..(i + packet_len)]
            .iter()
            .fold(0_u32, |acc, x| acc | 1 << (x - b'a'))
            .count_ones();

        if distinct_count as usize == packet_len {
            return Ok((i + packet_len) as u32);
        }
    }

    Err("No solution found".to_string())
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

    test_part_one_error!("abc" => "Input too small");
    test_part_one_error!("abcc" => "No solution found");
}
