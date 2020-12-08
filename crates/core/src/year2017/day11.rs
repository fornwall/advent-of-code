/// Using double-height coordinates - see https://www.redblobgames.com/grids/hexagons/
pub fn solution(input_string: &str, part1: bool) -> Result<u32, String> {
    fn distance(location: (i32, i32)) -> u32 {
        location.0.abs() as u32 + std::cmp::max(0, (location.1.abs() - location.0.abs()) / 2) as u32
    }

    let mut furthest = 0;

    let mut location = (0_i32, 0_i32);
    for step in input_string.split(',') {
        let diff = match step {
            "n" => (0, 2),
            "ne" => (1, 1),
            "se" => (1, -1),
            "s" => (0, -2),
            "sw" => (-1, -1),
            "nw" => (-1, 1),
            _ => {
                return Err(format!("Invalid step: {}", step));
            }
        };

        location = (location.0 + diff.0, location.1 + diff.1);

        if !part1 {
            furthest = std::cmp::max(furthest, distance(location));
        }
    }
    Ok(if part1 { distance(location) } else { furthest })
}

pub fn part1(input_string: &str) -> Result<u32, String> {
    solution(input_string, true)
}

pub fn part2(input_string: &str) -> Result<u32, String> {
    solution(input_string, false)
}

#[test]
fn test_part1() {
    assert_eq!(Ok(650), part1(include_str!("day11_input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(Ok(1465), part2(include_str!("day11_input.txt")));
}
