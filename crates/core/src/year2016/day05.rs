use crate::Input;
use md5::digest::generic_array::arr;
use md5::digest::FixedOutput;
use md5::{Digest, Md5};
use std::iter::FromIterator;

pub fn solve(input: &mut Input) -> Result<String, String> {
    const MAX_INDEX: i32 = 100_000_000;

    let mut password = input.part_values(Vec::new(), vec![' '; 8]);
    let door_id = input.text.as_bytes();

    let mut hasher = Md5::new();
    let mut output = arr![u8; 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 , 0, 0];

    for index in 0..MAX_INDEX {
        hasher.update(door_id);
        hasher.update(index.to_string().as_bytes());

        hasher.finalize_into_reset(&mut output);

        // Check if hash starts with five zeros without converting it to a string:
        if output[..2] == [0, 0] && output[2] <= 0x0F {
            if input.is_part_one() {
                password.push(
                    format!("{:x?}", (output[2] & 0x0F_u8))
                        .as_str()
                        .chars()
                        .next()
                        .unwrap_or('_'),
                );
                if password.len() == 8 {
                    return Ok(String::from_iter(password));
                }
            } else {
                let position = output[2] & 0x0F_u8;
                let character = output[3] >> 4;
                // "Use only the first result for each position, and ignore invalid positions.":
                if position < 8 && password[position as usize] == ' ' {
                    password[position as usize] =
                        format!("{:x?}", character).chars().next().unwrap_or('_');
                    if !password.contains(&' ') {
                        return Ok(String::from_iter(password));
                    }
                }
            }
        }
    }

    return Err(format!("Aborting after {} iterations", MAX_INDEX));
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!("abc" => "18f47a30".to_string());
    test_part_two!("abc" => "05ace8e3".to_string());

    let real_input = include_str!("day05_input.txt");
    test_part_one!(real_input => "1a3099aa".to_string());
    test_part_two!(real_input => "694190cd".to_string());
}
