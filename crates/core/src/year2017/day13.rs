fn solution(input_string: &str, part1: bool) -> Result<usize, String> {
    const MAX_DELAY: usize = 10_000_000;

    let layers = input_string.lines().count();
    let mut scanner_ranges = vec![0; layers];

    for (line_index, line) in input_string.lines().enumerate() {
        let error_message = || {
            format!(
                "Invalid input at line {}: Not '${{NUMBER}}: ${{NUMBER}}",
                line_index + 1
            )
        };

        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            return Err(error_message());
        }
        let depth = parts[0].parse::<usize>().map_err(|_| error_message())?;
        let range = parts[1].parse::<usize>().map_err(|_| error_message())?;
        if scanner_ranges.len() <= depth {
            scanner_ranges.resize(depth * 2, 0);
        }
        scanner_ranges[depth] = range;
    }

    'delay: for delay in if part1 { 0..1 } else { 0..MAX_DELAY } {
        let mut trip_severity = 0;

        for (position, &current_range) in scanner_ranges.iter().enumerate() {
            if current_range != 0 {
                let time = delay + position;
                let modulo = time % (2 * (current_range - 1));
                let scanner_position = if modulo < current_range {
                    modulo
                } else {
                    current_range - 2 - (modulo % current_range)
                };

                if scanner_position == 0 {
                    if part1 {
                        trip_severity += position * current_range;
                    } else {
                        continue 'delay;
                    }
                }
            }
        }

        return Ok(if part1 { trip_severity } else { delay });
    }

    Err("No solution found".to_string())
}
pub fn part1(input_string: &str) -> Result<usize, String> {
    solution(input_string, true)
}

pub fn part2(input_string: &str) -> Result<usize, String> {
    solution(input_string, false)
}

#[test]
fn test_part1() {
    assert_eq!(
        Ok(24),
        part1(
            "0: 3
1: 2
4: 4
6: 4"
        )
    );
    assert_eq!(Ok(748), part1(include_str!("day13_input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(
        Ok(10),
        part2(
            "0: 3
1: 2
4: 4
6: 4"
        )
    );
    assert_eq!(Ok(3873662), part2(include_str!("day13_input.txt")));
}
