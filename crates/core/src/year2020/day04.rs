use crate::input::Input;

fn is_valid(field_idx: usize, value: &str) -> bool {
    fn in_range(string: &str, start: u32, end: u32) -> bool {
        (start..=end).contains(&string.parse::<u32>().unwrap_or_default())
    }

    match field_idx {
        0 => value.len() == 4 && in_range(value, 1920, 2002),
        1 => value.len() == 4 && in_range(value, 2010, 2020),
        2 => value.len() == 4 && in_range(value, 2020, 2030),
        3 => {
            (value.ends_with("cm") && in_range(&value[0..(value.len() - 2)], 150, 193))
                || (value.ends_with("in") && in_range(&value[0..(value.len() - 2)], 59, 76))
        }
        4 => {
            value.starts_with('#')
                && value.len() == 7
                && value[1..]
                    .bytes()
                    .all(|c| c.is_ascii_digit() || c.is_ascii_lowercase())
        }
        5 => matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        6 => value.len() == 9 && value.parse::<u32>().is_ok(),
        _ => false,
    }
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    const FIELD_NAMES: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut fields_validity = [false; 7];
    let mut valid_passports_count = 0;

    for line in input.text.lines().chain(std::iter::once("")) {
        if line.is_empty() {
            if fields_validity.iter().all(|&ok| ok) {
                valid_passports_count += 1;
            }
            fields_validity.iter_mut().for_each(|ok| *ok = false);
        } else {
            for (line_idx, entry) in line.split(' ').enumerate() {
                let parts: Vec<&str> = entry.split(':').collect();
                if parts.len() != 2 {
                    return Err(format!(
                        "Line {} contains an entry not separated by a colon",
                        line_idx
                    ));
                }

                let key = parts[0];
                if let Some(field_idx) = FIELD_NAMES.iter().position(|&field| field == key) {
                    let value = parts[1];
                    fields_validity[field_idx] = input.is_part_one() || is_valid(field_idx, value);
                }
            }
        }
    }

    Ok(valid_passports_count)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in" => 2);

    let real_input = include_str!("day04_input.txt");
    test_part_one!(real_input => 210);
    test_part_two!(real_input => 131);
}
