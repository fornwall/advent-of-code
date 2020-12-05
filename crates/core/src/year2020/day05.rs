use crate::input::Input;

fn parse_seat_specifier(specifier: &str) -> u16 {
    specifier
        .chars()
        .map(|c| matches!(c, 'B' | 'R') as u16)
        .rev()
        .enumerate()
        .map(|(offset, bit_flag)| bit_flag << offset)
        .sum()
}

pub fn solve(input: &mut Input) -> Result<u16, String> {
    let seat_ids = input.text.lines().map(parse_seat_specifier);

    if input.is_part_one() {
        seat_ids
            .max()
            .ok_or_else(|| "No seats in input".to_string())
    } else {
        let mut seats = [0_u16; 127];
        for seat_id in seat_ids {
            let row = seat_id / 8;
            let col = seat_id % 8;
            seats[row as usize] |= 1 << col;
        }

        for this_seat_id in 0..u16::MAX {
            let row = this_seat_id / 8;
            let col = this_seat_id % 8;

            if seats[row as usize] & (1 << col) > 0 {
                // This is set - is next?
                let next_seat_id = this_seat_id + 1;
                let row = next_seat_id / 8;
                let col = next_seat_id % 8;
                if seats[row as usize] & (1 << col) == 0 {
                    return Ok(next_seat_id);
                }
            }
        }
        Err("No gap found".to_string())
        /*
        let mut seat_ids = seat_ids.collect::<Vec<_>>();
        seat_ids.sort_unstable();
        let non_adjacent_seats = seat_ids
            .windows(2)
            // "It's a completely full flight, so your seat should be the only missing boarding pass in your list":
            .filter(|two_seats| two_seats[0] + 1 != two_seats[1])
            .collect::<Vec<_>>();
        if non_adjacent_seats.len() == 1 {
            Ok(non_adjacent_seats[0][0] + 1)
        } else {
            Err("No unique gap found".to_string())
        }
         */
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two}; // , test_part_one_error, test_part_two, test_part_two_error};

    let real_input = include_str!("day05_input.txt");
    test_part_one!(real_input => 828);
    test_part_two!(real_input => 565);
}
