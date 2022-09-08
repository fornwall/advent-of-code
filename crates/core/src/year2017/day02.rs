use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let row_evaluator = if input.is_part_one() {
        |row: &[u32]| {
            let min = row.iter().min().unwrap_or(&0);
            let max = row.iter().max().unwrap_or(&0);
            max - min
        }
    } else {
        |row: &[u32]| {
            for (x_index, x) in row.iter().enumerate() {
                for (y_index, &y) in row.iter().enumerate() {
                    if x_index != y_index && y != 0 && x % y == 0 {
                        return x / y;
                    }
                }
            }
            0
        }
    };

    let mut checksum = 0;
    for line in input.text.lines() {
        let values: Vec<u32> = line
            .split_ascii_whitespace()
            .map(|cell| cell.parse::<u32>().map_err(|_| "Invalid input"))
            .collect::<Result<_, _>>()?;
        checksum += row_evaluator(&values);
    }
    Ok(checksum)
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("5 1 9 5\n7 5 3\n2 4 6 8" => 18);
    test_part_two!("5 9 2 8\n9 4 7 3\n3 8 6 5" => 9);

    let input = include_str!("day02_input.txt");
    test_part_one!(input => 41919);
    test_part_two!(input => 303);
}
