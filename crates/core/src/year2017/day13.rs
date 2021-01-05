use crate::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    const MAX_DELAY: usize = 10_000_000;

    let layers = input.text.lines().count();
    let mut scanner_ranges = vec![0; layers];

    for (line_index, line) in input.text.lines().enumerate() {
        let error_message = || {
            format!(
                "Invalid input at line {}: Not '${{NUMBER}}: ${{NUMBER}}'",
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

    'delay: for delay in 0..input.part_values(1, MAX_DELAY) {
        let mut trip_severity = 0;

        for (position, &current_range) in scanner_ranges.iter().enumerate() {
            if current_range != 0 {
                let time = delay + position;
                let caught = time % (2 * (current_range - 1)) == 0;
                if caught {
                    if input.is_part_one() {
                        trip_severity += position * current_range;
                    } else {
                        continue 'delay;
                    }
                }
            }
        }

        return Ok(input.part_values(trip_severity, delay));
    }

    Err("No solution found".to_string())
}

#[test]
fn tests() {
    use crate::{test_part_one, test_part_two};
    let real_input = include_str!("day13_input.txt");
    test_part_one!(real_input => 748);
    test_part_two!(real_input => 3_873_662);
}
