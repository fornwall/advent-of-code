use crate::input::Input;

struct Disc {
    positions: u32,
    initial_position: u32,
}

pub fn solve(input: &Input) -> Result<u32, String> {
    const MAX_TIME: u32 = 10_000_000;

    let mut discs = input
        .text
        .lines()
        .map(|line| {
            let error_mapper = |_| "Invalid number".to_string();

            let words = line.split(' ').collect::<Vec<_>>();
            if words.len() != 12 {
                return Err("Line not containing 19 words".to_string());
            }

            let positions = words[3].parse::<u32>().map_err(error_mapper)?;
            let initial_position = words[11]
                .strip_suffix('.')
                .ok_or_else(|| "Line not ending with period".to_string())?
                .parse::<u32>()
                .map_err(error_mapper)?;
            Ok(Disc {
                positions,
                initial_position,
            })
        })
        .collect::<Result<Vec<Disc>, String>>()?;

    if input.is_part_two() {
        discs.push(Disc {
            positions: 11,
            initial_position: 0,
        });
    }

    (0..=MAX_TIME)
        .find(|&time| {
            discs.iter().enumerate().all(|(disc_idx, disc)| {
                let fall_time = (disc_idx + 1) as u32;
                let current_position = (disc.initial_position + time + fall_time) % disc.positions;
                current_position == 0
            })
        })
        .ok_or_else(|| format!("No solution within {MAX_TIME} seconds found"))
}

#[test]
pub fn tests() {
    test_part_one!("Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1." => 5);

    let real_input = include_str!("day15_input.txt");
    test_part_one!(real_input => 203_660);
    test_part_two!(real_input => 2_408_135);
}
