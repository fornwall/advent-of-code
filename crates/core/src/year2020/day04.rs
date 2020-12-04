use crate::input::Input;
use std::collections::HashMap;

fn is_valid(field_name: &str, field_value: &str) -> bool {
    match field_name {
        "byr" => {
            field_value.len() == 4
                && (1920..=2002).contains(&field_value.parse::<u32>().unwrap_or_default())
        }
        "iyr" => {
            field_value.len() == 4
                && (2010..=2020).contains(&field_value.parse::<u32>().unwrap_or_default())
        }
        "eyr" => {
            field_value.len() == 4
                && (2020..=2030).contains(&field_value.parse::<u32>().unwrap_or_default())
        }
        "hgt" => {
            (field_value.ends_with("cm")
                && (150..=193).contains(
                    &field_value[0..(field_value.len() - 2)]
                        .parse::<u32>()
                        .unwrap_or_default(),
                ))
                || (field_value.ends_with("in")
                    && (59..=76).contains(
                        &field_value[0..(field_value.len() - 2)]
                            .parse::<u32>()
                            .unwrap_or_default(),
                    ))
        }
        "hcl" => {
            field_value.starts_with('#')
                && field_value.len() == 7
                && (&field_value[1..])
                    .chars()
                    .all(|c| c.is_ascii_digit() || c.is_ascii_lowercase())
        }
        "ecl" => matches!(
            field_value,
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
        ),
        "pid" => field_value.len() == 9 && field_value.parse::<u32>().is_ok(),
        _ => false,
    }
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let all_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut current_password: HashMap<&str, String> = HashMap::new();
    let mut valid_count = 0;

    for line in input.text.lines().chain(std::iter::once("")) {
        if line.is_empty() {
            let mut this_passport_valid = true;
            for &field in &all_fields {
                if !current_password.contains_key(field) {
                    this_passport_valid = false;
                } else if input.is_part_two()
                    && !is_valid(field, current_password.get(field).unwrap())
                {
                    this_passport_valid = false;
                }
            }

            current_password.clear();

            if this_passport_valid {
                valid_count += 1;
            }
        } else {
            for entry in line.split(' ') {
                let key = entry.split(':').next().unwrap();
                let value = entry.split(':').nth(1).unwrap();
                current_password.insert(key, value.to_string());
            }
        }
    }

    Ok(valid_count)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two}; // , test_part_one_error, test_part_two, test_part_two_error};

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
