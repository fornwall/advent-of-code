use crate::input::Input;
use std::collections::HashMap;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let target_turn: u32 = input.part_values(2020, 30_000_000);
    let mut value_to_turn: HashMap<u32, u32> = HashMap::with_capacity(target_turn as usize / 1000);

    let starting_numbers = input
        .text
        .split(',')
        .map(|s| {
            s.parse::<u32>()
                .map_err(|error| format!("Invalid input: {}", error))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut next_number = starting_numbers[0];
    let mut turn = 1;

    loop {
        if turn == target_turn {
            return Ok(next_number);
        }

        next_number = value_to_turn
            .insert(next_number, turn)
            // If the number had been spoken before; the current player announces
            // how many turns apart the number is from when it was previously spoken:
            .map(|last_spoken_turn| turn - last_spoken_turn)
            // If that was the first time the number has been spoken, the current player says 0:
            .unwrap_or(0);

        // Actually, if we're still starting:
        if turn < starting_numbers.len() as u32 {
            next_number = starting_numbers[turn as usize];
        }

        turn += 1;
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "0,3,6";
    test_part_one!(example => 436);
    test_part_two!(example => 175_594);

    let real_input = include_str!("day15_input.txt");
    test_part_one!(real_input => 1194);
    test_part_two!(real_input => 48710);
}
