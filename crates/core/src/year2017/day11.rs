use crate::input::Input;

/// Using double-height coordinates - see <https://www.redblobgames.com/grids/hexagons//>
pub fn solve(input: &Input) -> Result<u32, String> {
    fn distance(location: (i32, i32)) -> u32 {
        location.0.unsigned_abs()
            + std::cmp::max(0, (location.1.abs() - location.0.abs()) / 2) as u32
    }

    let mut furthest = 0;

    let mut location = (0_i32, 0_i32);
    for step in input.text.split(',') {
        let diff = match step {
            "n" => (0, 2),
            "ne" => (1, 1),
            "se" => (1, -1),
            "s" => (0, -2),
            "sw" => (-1, -1),
            "nw" => (-1, 1),
            _ => {
                return Err(format!("Invalid step: {step}"));
            }
        };

        location = (location.0 + diff.0, location.1 + diff.1);

        if input.is_part_two() {
            furthest = std::cmp::max(furthest, distance(location));
        }
    }
    Ok(input.part_values(distance(location), furthest))
}

#[test]
fn tests() {
    let real_input = include_str!("day11_input.txt");
    test_part_one!(real_input => 650);
    test_part_two!(real_input => 1465);
}
