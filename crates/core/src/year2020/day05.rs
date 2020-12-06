use crate::input::Input;

type SeatId = u16;

/// Parse a 10 letter seat specifier as a binary string.
/// It treats 'B' and 'R' as '1', others ('L' and 'F') as '0'.
fn parse_seat_specifier(specifier: &str) -> SeatId {
    specifier
        .chars()
        .map(|c| matches!(c, 'B' | 'R') as SeatId)
        .enumerate()
        .map(|(bit_index, bit_flag)| bit_flag << (9 - bit_index))
        .sum()
}

pub fn solve(input: &mut Input) -> Result<SeatId, String> {
    if let Some(invalid_line_idx) = input.text.lines().enumerate().find_map(|(line_idx, line)| {
        if line.len() != 10
            || !line[0..7].chars().all(|c| matches!(c, 'F' | 'B'))
            || !line[7..10].chars().all(|c| matches!(c, 'L' | 'R'))
        {
            Some(line_idx)
        } else {
            None
        }
    }) {
        return Err(format!(
            "Line {}: Not expected format (7 'F' or 'B' characters followed by 3 'L' or 'R' ones)",
            invalid_line_idx
        ));
    }

    let seat_ids = input.text.lines().map(parse_seat_specifier);

    if input.is_part_one() {
        seat_ids
            .max()
            .ok_or_else(|| "No seats in input".to_string())
    } else {
        let mut seats = [0_u8; 127];
        for seat_id in seat_ids {
            let row = seat_id / 8;
            let col = seat_id % 8;
            seats[row as usize] |= 1 << col;
        }

        for this_seat_id in 0..SeatId::MAX {
            let (row, col) = (this_seat_id / 8, this_seat_id % 8);
            if seats[row as usize] & (1 << col) > 0 {
                // This seat is occupied - is the next one?
                // Otherwise we have found the searched after gap.
                let next_seat_id = this_seat_id + 1;
                let (row, col) = (next_seat_id / 8, next_seat_id % 8);
                if seats[row as usize] & (1 << col) == 0 {
                    return Ok(next_seat_id);
                }
            }
        }
        Err("No gap found".to_string())
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day05_input.txt");
    test_part_one!(real_input => 828);
    test_part_two!(real_input => 565);
}
