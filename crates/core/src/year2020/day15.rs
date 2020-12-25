use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    const NEVER_SEEN: u32 = 0;

    let target_turn: u32 = input.part_values(2020, 30_000_000);
    let mut value_to_turn: Vec<u32> = vec![0; target_turn as usize];
    let mut next_number = 0;
    let mut turn = 0;

    for (idx, starting_number) in input
        .text
        .split(',')
        .map(|s| {
            s.parse::<u32>()
                .map_err(|error| format!("Invalid input: {}", error))
        })
        .enumerate()
    {
        next_number = starting_number?;
        value_to_turn[next_number as usize] = (idx + 1) as u32;
        turn += 1;
    }

    while turn != target_turn {
        if next_number >= target_turn {
            return Err(format!("Too big number: {}", next_number));
        }

        let last_spoken_turn = std::mem::replace(&mut value_to_turn[next_number as usize], turn);

        next_number = if last_spoken_turn == NEVER_SEEN {
            // If that was the first time the number has been spoken, the current player says 0:
            0
        } else {
            // If the number had been spoken before; the current player announces
            // how many turns apart the number is from when it was previously spoken:
            turn - last_spoken_turn
        };

        turn += 1;
    }

    Ok(next_number)
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
