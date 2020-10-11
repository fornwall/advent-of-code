fn solution(input_string: &str, row_evaluator: fn(&Vec<u32>) -> u32) -> Result<u32, String> {
    let mut checksum = 0;
    for line in input_string.lines() {
        let values: Vec<u32> = line
            .split_ascii_whitespace()
            .map(|cell| cell.parse::<u32>().map_err(|_| "Invalid input"))
            .collect::<Result<_, _>>()?;
        checksum += row_evaluator(&values);
    }
    Ok(checksum)
}
pub fn part1(input_string: &str) -> Result<u32, String> {
    solution(input_string, |row| {
        let min = row.iter().min().unwrap_or(&0);
        let max = row.iter().max().unwrap_or(&0);
        max - min
    })
}

pub fn part2(input_string: &str) -> Result<u32, String> {
    solution(input_string, |row| {
        for (x_index, x) in row.iter().enumerate() {
            for (y_index, y) in row.iter().enumerate() {
                if x_index != y_index && x % y == 0 {
                    return x / y;
                }
            }
        }
        0
    })
}

#[test]
fn test_part1() {
    assert_eq!(
        Ok(18),
        part1(
            "5 1 9 5
7 5 3
2 4 6 8"
        )
    );
    assert_eq!(Ok(41919), part1(include_str!("day02_input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(
        Ok(9),
        part2(
            "5 9 2 8
9 4 7 3
3 8 6 5"
        )
    );
    assert_eq!(Ok(303), part2(include_str!("day02_input.txt")));
}
