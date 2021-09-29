use crate::input::Input;
type GridValue = i32;

/// A summed-area table is a data structure for quickly generating sum of values in a rectangular grid.
/// See https://en.wikipedia.org/wiki/Summed-area_table
/// Note that x and y coordinates are 1-based in method parameters.
struct SummedAreaTable {
    storage: [GridValue; (Self::SIZE * Self::SIZE) as usize],
}

impl SummedAreaTable {
    const SIZE: u32 = 300;

    fn new(serial_number: GridValue) -> Self {
        let mut result = Self {
            storage: [0; (Self::SIZE * Self::SIZE) as usize],
        };
        for y in 1..=Self::SIZE {
            let mut row_value: GridValue = 0;
            for x in 1..=Self::SIZE {
                row_value += cell_power(x, y, serial_number);
                let storage_index = x - 1 + (y - 1) * Self::SIZE;
                result.storage[storage_index as usize] = row_value + result.at(x, y - 1);
            }
        }
        result
    }

    fn at(&self, x: u32, y: u32) -> GridValue {
        if x < 1 || y < 1 {
            return 0;
        }
        self.storage
            .get((x - 1 + (y - 1) * Self::SIZE) as usize)
            .copied()
            .unwrap_or(0)
    }

    fn square_power(&self, x: u32, y: u32, size: u32) -> GridValue {
        self.at(x - 1, y - 1) + self.at(x + size - 1, y + size - 1)
            - self.at(x + size - 1, y - 1)
            - self.at(x - 1, y + size - 1)
    }
}

const fn cell_power(x: u32, y: u32, serial_number: GridValue) -> GridValue {
    // Find the fuel cell's rack ID, which is its X coordinate plus 10.
    let rack_id = x + 10;
    // Begin with a power level of the rack ID times the Y coordinate.
    let mut cell_power: GridValue = (rack_id * y) as GridValue;
    // Increase the power level by the value of the grid serial number (your puzzle input).
    cell_power += serial_number;
    // Set the power level to itself multiplied by the rack ID.
    cell_power *= rack_id as GridValue;
    // Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
    cell_power = (cell_power / 100) % 10;
    // Subtract 5 from the power level.
    cell_power - 5
}

pub fn solve(input: &mut Input) -> Result<String, String> {
    let serial_number = input
        .text
        .parse::<GridValue>()
        .map_err(|error| format!("Invalid input: {}", error.to_string()))?;

    if serial_number >= 10_000 {
        return Err("Too big serial number - max is 10,000".to_string());
    }

    let table = SummedAreaTable::new(serial_number);

    let mut optimal_power: GridValue = 0;
    let mut optimal_square_width = 0;
    let mut optimal_point = (0, 0);

    for square_width in input.part_values(3..=3, 1..=300) {
        for y in 1..=(SummedAreaTable::SIZE - square_width) {
            for x in 1..=(SummedAreaTable::SIZE - square_width) {
                let square_power = table.square_power(x, y, square_width);
                if square_power > optimal_power {
                    optimal_square_width = square_width;
                    optimal_power = square_power;
                    optimal_point = (x, y);
                }
            }
        }
    }

    Ok(if input.is_part_one() {
        format!("{},{}", optimal_point.0, optimal_point.1)
    } else {
        format!(
            "{},{},{}",
            optimal_point.0, optimal_point.1, optimal_square_width
        )
    })
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("18" => "33,45".into());
    test_part_one!("42" => "21,61".into());
    test_part_two!("18" => "90,269,16".into());
    test_part_two!("42" => "232,251,12".into());

    let input = include_str!("day11_input.txt");
    test_part_one!(input => "21,68".into());
    test_part_two!(input => "90,201,15".into());
}
