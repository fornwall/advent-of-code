use std::collections::HashMap;

use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    let desired: HashMap<&str, i32> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into();

    'outer: for (line_idx, line) in input.text.lines().enumerate() {
        let error = || format!("Line {}: Invalid format", line_idx + 1);
        let words = line.split(' ').collect::<Vec<_>>();
        if words.len() % 2 != 0 {
            return Err(error());
        }

        for name_idx in (2..words.len()).step_by(2) {
            let attribute_name = words[name_idx].strip_suffix(':').ok_or_else(error)?;
            let attribute_value = words[name_idx + 1]
                .strip_suffix(',')
                .unwrap_or(words[name_idx + 1])
                .parse::<i32>()
                .map_err(|_| "Unable to parse attribute value")?;

            let desired_value = *desired
                .get(attribute_name)
                .ok_or("Invalid attribute value")?;

            let does_match = match (input.is_part_two(), attribute_name) {
                (true, "cats" | "trees") => desired_value < attribute_value,
                (true, "pomeranians" | "goldfish") => desired_value > attribute_value,
                _ => desired_value == attribute_value,
            };
            if !does_match {
                continue 'outer;
            }
        }
        return Ok(line_idx as u32 + 1);
    }

    Err("No matching Sue found".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day16_input.txt");
    test_part_one!(real_input => 213);
    test_part_two!(real_input => 323);
}
