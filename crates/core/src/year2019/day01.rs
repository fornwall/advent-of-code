use crate::input::Input;

fn sum_required_fuel(input_string: &str, fuel_calculator: fn(u32) -> u32) -> Result<u32, String> {
    input_string
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            let module_mass = line
                .parse::<u32>()
                .map_err(|error| format!("Line {}: {}", line_index + 1, error.to_string()))?;
            if module_mass < 6 {
                return Err(format!(
                    "Line {}: Too small module mass (less than 6)",
                    line_index + 1
                ));
            }
            Ok(fuel_calculator(module_mass))
        })
        .sum::<Result<_, _>>()
}

pub fn solve(input: &Input) -> Result<u32, String> {
    if input.is_part_one() {
        sum_required_fuel(input.text, |mass| mass / 3 - 2)
    } else {
        fn required_fuel(mass: u32) -> u32 {
            (mass / 3)
                .checked_sub(2)
                .map_or(0, |fuel| fuel + required_fuel(fuel))
        }
        sum_required_fuel(input.text, required_fuel)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};

    test_part_one!("12" => 2);
    test_part_one!("14" => 2);
    test_part_one!("1969" => 654);
    test_part_one!("100756" => 33583);

    let input = include_str!("day01_input.txt");
    test_part_one!(input => 3_262_358);
    test_part_one_error!(
        "\n" => "Line 1: cannot parse integer from empty string"
    );

    test_part_two!("14" => 2);
    test_part_two!("1969" => 966);
    test_part_two!("100756" => 50346);

    test_part_two!(input => 4_890_696);
}
