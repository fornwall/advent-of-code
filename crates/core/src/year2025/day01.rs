use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut dial_pointing_at = 50;
    let mut times_pointing_at_zero = 0;
    for (line_idx, line) in input.text.lines().enumerate() {
        let first_char = line.as_bytes().first();
        let direction = match first_char {
            Some(b'L') => -1,
            Some(b'R') => 1,
            _ => {
                return Err(format!("Unable to parse line {line_idx}: '{line}'"));
            }
        };
        let steps: i32 = line[1..]
            .parse()
            .map_err(|_| format!("Unable to parse line {line_idx}: '{line}'"))?;

        let new_dial_pointing_at = (dial_pointing_at + direction * steps).rem_euclid(100);
        if input.is_part_one() {
            times_pointing_at_zero += (new_dial_pointing_at == 0) as i32;
        } else {
            times_pointing_at_zero += (dial_pointing_at + direction * steps).div_euclid(100).abs();
            let going_left = (direction == -1) as i32;
            let starting_at_zero = (dial_pointing_at == 0) as i32;
            let ending_at_zero = (new_dial_pointing_at == 0) as i32;
            times_pointing_at_zero += going_left * (ending_at_zero - starting_at_zero);
        }
        dial_pointing_at = new_dial_pointing_at;
    }
    Ok(times_pointing_at_zero as u64)
}

#[test]
pub fn tests() {
    let test_input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    test_part_one_no_allocations!(test_input => 3);
    test_part_two_no_allocations!(test_input => 6);

    let test_input = "L50";
    test_part_one_no_allocations!(test_input => 1);
    test_part_two_no_allocations!(test_input => 1);

    let real_input = include_str!("day01_input.txt");
    test_part_one_no_allocations!(real_input => 1023);
    test_part_two_no_allocations!(real_input => 5899);
}
