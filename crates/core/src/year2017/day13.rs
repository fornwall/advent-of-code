pub fn part1(input_string: &str) -> Result<usize, String> {
    let layers = input_string.lines().count();
    let mut scanner_ranges = vec![0; layers];

    for line in input_string.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            return Err("Invalid input".to_string());
        }
        let depth = parts[0]
            .parse::<usize>()
            .map_err(|_| "Invalid input".to_string())?;
        let range = parts[1]
            .parse::<usize>()
            .map_err(|_| "Invalid input".to_string())?;
        if scanner_ranges.len() <= depth {
            scanner_ranges.resize(depth * 2, 0);
        }
        scanner_ranges[depth] = range;
    }

    let mut trip_severity = 0;

    for position in 0..scanner_ranges.len() {
        if scanner_ranges[position] != 0 {
            let modulo = position % (2 * (scanner_ranges[position] - 1));
            let scanner_position = if modulo < scanner_ranges[position] {
                modulo
            } else {
                scanner_ranges[position] - 2 - (modulo % scanner_ranges[position])
            };
            if scanner_position == 0 {
                trip_severity += position * scanner_ranges[position];
            }
        }
    }
    Ok(trip_severity)
}

pub fn part2(_input_string: &str) -> Result<u32, String> {
    Ok(0)
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
    //assert_eq!(Ok(0), part2(include_str!("day13_input.txt")));
}
