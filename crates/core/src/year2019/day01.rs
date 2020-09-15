fn sum_required_fuel(input_string: &str, fuel_calculator: fn(u32) -> u32) -> Result<u32, String> {
    let parts = input_string
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            let module_mass = line.parse::<u32>().map_err(|error| {
                format!(
                    "Input error at line {}: {}",
                    line_index + 1,
                    error.to_string()
                )
            })?;
            Ok(fuel_calculator(module_mass))
        })
        .collect::<Result<Vec<u32>, String>>()?;
    Ok(parts.iter().sum())
}

pub fn part1(input_string: &str) -> Result<u32, String> {
    sum_required_fuel(input_string, |mass| mass / 3 - 2)
}

pub fn part2(input_string: &str) -> Result<u32, String> {
    fn required_fuel(mass: u32) -> u32 {
        match (mass / 3).checked_sub(2) {
            Some(fuel) => fuel + required_fuel(fuel),
            None => 0,
        }
    }

    sum_required_fuel(input_string, required_fuel)
}

#[test]
pub fn tests_part1() {
    assert_eq!(Ok(2), part1("12"));
    assert_eq!(Ok(2), part1("14"));
    assert_eq!(Ok(654), part1("1969"));
    assert_eq!(Ok(33583), part1("100756"));

    assert_eq!(Ok(3_262_358), part1(include_str!("day01_input.txt")));

    assert_eq!(
        Some("Input error at line 1: cannot parse integer from empty string".to_string()),
        part1("\n").err()
    );
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(2), part2("14"));
    assert_eq!(Ok(966), part2("1969"));
    assert_eq!(Ok(50346), part2("100756"));

    assert_eq!(Ok(4_890_696), part2(include_str!("day01_input.txt")));
}
