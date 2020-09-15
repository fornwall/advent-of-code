fn sum_required_fuel(input_string: &str, fuel_calculator: fn(u32) -> u32) -> Result<u32, String> {
    let mut total_fuel: u32 = 0;
    for (line_number, line) in input_string.lines().enumerate() {
        match line.parse::<u32>() {
            Ok(value) => {
                total_fuel += fuel_calculator(value);
            }
            Err(error) => {
                return Err(format!(
                    "Parse error at line {}: {}",
                    line_number + 1,
                    error.to_string()
                ));
            }
        }
    }
    Ok(total_fuel)
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
    assert_eq!(2, part1("12").unwrap());
    assert_eq!(2, part1("14").unwrap());
    assert_eq!(654, part1("1969").unwrap());
    assert_eq!(33583, part1("100756").unwrap());

    assert_eq!(3_262_358, part1(include_str!("day01_input.txt")).unwrap());

    assert_eq!(
        "Parse error at line 1: cannot parse integer from empty string",
        part1("\n").err().unwrap()
    );
}

#[test]
fn tests_part2() {
    assert_eq!(2, part2("14").unwrap());
    assert_eq!(966, part2("1969").unwrap());
    assert_eq!(50346, part2("100756").unwrap());

    assert_eq!(4_890_696, part2(include_str!("day01_input.txt")).unwrap());
}
