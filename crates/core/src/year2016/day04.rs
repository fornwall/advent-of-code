use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    const NUM_ASCII_LOWERCASE: usize = 26;

    let mut sector_ids_sum = 0;

    for (line_idx, line) in input.text.lines().enumerate() {
        let on_error = || format!("Line {}: Invalid input", line_idx + 1);

        let (room_name, sector_id_and_checksum) = line.rsplit_once('-').ok_or_else(on_error)?;

        let (sector_id, stated_checksum) = sector_id_and_checksum
            .split_once('[')
            .and_then(|(id, checksum_str)| {
                let checksum: usize =
                    checksum_str
                        .bytes()
                        .take(5)
                        .enumerate()
                        .try_fold(0, |acc, (idx, b)| {
                            b.is_ascii_lowercase()
                                .then_some(acc * idx * NUM_ASCII_LOWERCASE + (b - b'a') as usize)
                        })?;

                Some((id.parse::<u32>().ok()?, checksum))
            })
            .ok_or_else(on_error)?;

        if input.is_part_one() {
            let mut char_frequency: [(u8, u32); NUM_ASCII_LOWERCASE] =
                std::array::from_fn(|i| (i as u8, 0));
            for c in room_name.bytes().filter(u8::is_ascii_lowercase) {
                char_frequency[(c - b'a') as usize].1 += 1;
            }

            char_frequency.sort_unstable_by(|(c1, f1), (c2, f2)| f2.cmp(f1).then(c1.cmp(c2)));

            let computed_checksum = char_frequency
                .iter()
                .take(5)
                .enumerate()
                .fold(0, |acc, (idx, (c, _f))| {
                    acc * idx * NUM_ASCII_LOWERCASE + *c as usize
                });

            if computed_checksum == stated_checksum {
                sector_ids_sum += sector_id;
            }
        } else {
            let desired_name = b"northpole object storage".iter();
            if room_name
                .bytes()
                .map(|a| match a {
                    b'-' => b' ',
                    _ if a.is_ascii_lowercase() => {
                        ((u32::from(a) - 'a' as u32 + sector_id) % (NUM_ASCII_LOWERCASE as u32))
                            as u8
                            + b'a'
                    }
                    _ => u8::MAX,
                })
                .zip(desired_name)
                .all(|(a, &b)| a == b)
            {
                return Ok(sector_id);
            }
        }
    }

    Ok(sector_ids_sum)
}

#[test]
pub fn tests() {
    let real_input = include_str!("day04_input.txt");

    test_part_one_no_allocations!(real_input => 245_102);
    test_part_two_no_allocations!(real_input => 324);
}
