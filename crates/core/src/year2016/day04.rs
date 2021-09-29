use crate::input::Input;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut sector_ids_sum = 0;

    for (line_idx, line) in input.text.lines().enumerate() {
        let on_error = || format!("Line {}: Invalid input", line_idx + 1);

        let mut char_frequency = HashMap::new();

        let (room_name, sector_id_and_checksum) = line.rsplit_once('-').ok_or_else(on_error)?;
        for c in room_name.chars().filter(|&c| c != '-') {
            *char_frequency.entry(c).or_insert(0) += 1;
        }

        let (sector_id, checksum) = sector_id_and_checksum
            .split_once('[')
            .and_then(|(id, checksum)| Some((id.parse::<u32>().ok()?, checksum.strip_suffix(']')?)))
            .ok_or_else(on_error)?;

        let mut sorted_by_frequency = room_name
            .chars()
            .filter(|&c| c != '-')
            .collect::<HashSet<char>>()
            .into_iter()
            .collect::<Vec<char>>();

        sorted_by_frequency.sort_unstable_by(|c1, c2| {
            let f1 = char_frequency.get(c1).unwrap_or(&0);
            let f2 = char_frequency.get(c2).unwrap_or(&0);
            f2.cmp(f1).then(c1.cmp(c2))
        });

        sorted_by_frequency = sorted_by_frequency
            .into_iter()
            .take(5)
            .collect::<Vec<char>>();

        if sorted_by_frequency.into_iter().collect::<String>() == checksum {
            sector_ids_sum += sector_id;

            if input.is_part_two() {
                let decrypted_name = room_name
                    .chars()
                    .map(|a| match a {
                        '-' => ' ',
                        _ => (((a as u32 - 'a' as u32 + sector_id) % 26_u32) as u8 + b'a') as char,
                    })
                    .collect::<String>();
                if decrypted_name == "northpole object storage" {
                    return Ok(sector_id);
                }
            }
        }
    }

    Ok(sector_ids_sum)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day04_input.txt");

    test_part_one!(real_input => 245_102);
    test_part_two!(real_input => 324);
}
