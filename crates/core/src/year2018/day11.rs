use std::collections::VecDeque;

fn cell_power(x: usize, y: usize, serial_number: i64) -> i64 {
    // Find the fuel cell's rack ID, which is its X coordinate plus 10.
    let rack_id = x + 10;
    // Begin with a power level of the rack ID times the Y coordinate.
    let mut cell_power: i64 = (rack_id * y) as i64;
    // Increase the power level by the value of the grid serial number (your puzzle input).
    cell_power += serial_number;
    // Set the power level to itself multiplied by the rack ID.
    cell_power *= rack_id as i64;
    // Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
    cell_power = (cell_power / 100) % 10;
    // Subtract 5 from the power level.
    cell_power - 5
}

pub fn part1(input_string: &str) -> String {
    let serial_number = input_string.parse::<i64>().unwrap();

    let mut optimal_power = 0i64;
    let mut optimal_point = (0, 0);

    for x in 1..=298 {
        for y in 1..=298 {
            let mut square_power = 0;

            for i in 0..=2 {
                for j in 0..=2 {
                    let x_coordinate = x + i;
                    let y_coordinate = y + j;
                    square_power += cell_power(x_coordinate, y_coordinate, serial_number);
                }
            }

            if square_power > optimal_power {
                optimal_power = square_power;
                optimal_point = (x, y);
            }
        }
    }

    format!("{},{}", optimal_point.0, optimal_point.1)
}

pub fn part2(input_string: &str) -> String {
    let serial_number = input_string.parse::<i64>().unwrap();

    let mut optimal_power = 0i64;
    let mut optimal_square_width = 0;
    let mut optimal_point = (0, 0);

    for left_column in 0..300usize {
        let mut row_sums = [0; 300];
        for right_column in left_column..300 {
            let square_width = right_column - left_column + 1;

            for (row, value) in row_sums.iter_mut().enumerate() {
                // From zero based to 1 based:
                *value += cell_power(right_column + 1, row + 1, serial_number);
            }

            let mut deque = VecDeque::with_capacity(square_width as usize);
            let mut square_power = 0;

            for &row_sum in row_sums.iter().take(square_width - 1) {
                deque.push_front(row_sum);
                square_power += row_sum;
            }

            for (end_row, &row_value) in row_sums.iter().enumerate().skip(square_width - 1) {
                // for end_row in square_width - 1..300 {
                deque.push_front(row_value);
                square_power += row_value;

                if square_power > optimal_power {
                    optimal_point = (left_column + 1, end_row + 1 - (square_width - 1));
                    optimal_square_width = square_width;
                    optimal_power = square_power;
                }

                square_power -= deque.pop_back().unwrap();
            }
        }
    }

    format!(
        "{},{},{}",
        optimal_point.0, optimal_point.1, optimal_square_width
    )
}

#[test]
fn tests_part1() {
    assert_eq!("33,45", part1("18"));
    assert_eq!("21,61", part1("42"));
    assert_eq!("21,68", part1(include_str!("day11_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!("90,269,16", part2("18"));
    assert_eq!("232,251,12", part2("42"));
    assert_eq!("90,201,15", part2(include_str!("day11_input.txt")));
}
