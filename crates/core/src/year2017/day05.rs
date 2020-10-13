fn solution(input_string: &str, jump_change_computer: fn(i32) -> i32) -> Result<u32, String> {
    let mut jumps: Vec<i32> = input_string
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            line.parse::<i32>().map_err(|error| {
                format!(
                    "Invalid input at line {}: {}",
                    line_index + 1,
                    error.to_string()
                )
            })
        })
        .collect::<Result<_, _>>()?;

    let mut position: i32 = 0;
    for step in 1..100_000_000 {
        let old_position = position;
        position += jumps[position as usize];
        if position < 0 || position as usize >= jumps.len() {
            return Ok(step);
        }
        jumps[old_position as usize] += jump_change_computer(jumps[old_position as usize]);
    }
    Err("No solution found".to_string())
}

pub fn part1(input_string: &str) -> Result<u32, String> {
    solution(input_string, |_| 1)
}

pub fn part2(input_string: &str) -> Result<u32, String> {
    solution(input_string, |offset| if offset >= 3 { -1 } else { 1 })
}

#[test]
fn test_part1() {
    assert_eq!(Ok(374269), part1(include_str!("day05_input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(Ok(27720699), part2(include_str!("day05_input.txt")));
}
