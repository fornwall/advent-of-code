use crate::input::Input;

const fn part_1_fuel_consumption(distance: i32) -> i32 {
    distance
}

const fn part_2_fuel_consumption(distance: i32) -> i32 {
    // Fuel consumption is (with Gauss method):
    //   1+2+..+distance = distance * (distance + 1) / 2
    (distance * (distance + 1)) / 2
}

pub fn solve(input: &Input) -> Result<u32, String> {
    let mut positions = input
        .text
        .split(',')
        .map(str::parse::<u16>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Input is not comma-separated u16 values".to_string())?;

    let fuel_consumption: fn(i32) -> i32 =
        input.part_values(part_1_fuel_consumption, part_2_fuel_consumption);

    let range = if input.is_part_one() {
        positions.sort_unstable();
        let median = positions[positions.len() / 2];
        median..=median
    } else {
        let mean = (positions.iter().map(|n| *n as usize).sum::<usize>() / positions.len()) as u16;
        (if mean == 0 { 0 } else { mean - 1 })..=mean + 2
    };
    Ok(range
        .map(|candidate_spot| {
            positions
                .iter()
                .map(|n| fuel_consumption((i32::from(*n) - i32::from(candidate_spot)).abs()))
                .sum::<i32>() as u32
        })
        .min()
        .unwrap_or_default())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "16,1,2,0,4,2,7,1,2,14";
    test_part_one!(example => 37);
    test_part_two!(example => 168);

    let real_input = include_str!("day07_input.txt");
    test_part_one!(real_input => 328_318);
    test_part_two!(real_input => 89_791_146);
}
