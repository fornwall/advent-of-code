use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u16, String> {
    let mut seat_ids = input
        .text
        .lines()
        .enumerate()
        .map(|(line_idx, line)| {
            let binary_string = line
                .replace(&['B', 'R'][..], "1")
                .replace(&['F', 'L'][..], "0");

            u16::from_str_radix(&binary_string, 2).map_err(|_| {
                format!(
                    "Line {}: Invalid format - not eight B|F|R|L characters",
                    line_idx + 1
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    seat_ids.sort_unstable();

    if input.is_part_one() {
        seat_ids
            .last()
            .copied()
            .ok_or_else(|| "No seats in input".to_string())
    } else {
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
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two}; // , test_part_one_error, test_part_two, test_part_two_error};

    let real_input = include_str!("day05_input.txt");
    test_part_one!(real_input => 828);
    test_part_two!(real_input => 565);
}
