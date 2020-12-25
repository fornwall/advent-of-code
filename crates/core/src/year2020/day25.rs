use crate::input::Input;
use crate::mod_exp::mod_exp;

const MODULO: u64 = 20_201_227;

const fn find_loop_size(subject_number: u64, desired_value: u64) -> u64 {
    let mut loop_count = 1;
    let mut value = 1;
    loop {
        value *= subject_number;
        value %= MODULO;
        if value == desired_value {
            break loop_count;
        }
        loop_count += 1;
    }
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    if input.is_part_two() {
        return Ok(0);
    }

    let on_error = || "Invalid input".to_string();

    let mut lines = input.text.lines();
    let card_public_key = lines
        .next()
        .ok_or_else(on_error)?
        .parse::<u64>()
        .map_err(|_| on_error())?;
    let door_public_key = lines
        .next()
        .ok_or_else(on_error)?
        .parse::<u64>()
        .map_err(|_| on_error())?;

    let card_loop_size = find_loop_size(7, card_public_key);

    let encryption_key = mod_exp(
        i128::from(door_public_key),
        i128::from(card_loop_size),
        i128::from(MODULO),
    ) as u64;

    Ok(encryption_key as u64)
}

#[test]
pub fn tests() {
    use crate::test_part_one;

    let example = "5764801\n17807724";
    test_part_one!(example => 14_897_079);
    let real_input = include_str!("day25_input.txt");
    test_part_one!(real_input => 18_862_163);
}
