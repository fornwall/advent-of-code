use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut valid_passwords = 0;

    for (line_index, line) in input.text.lines().enumerate() {
        let on_error = || {
            format!(
                "Line {}: Invalid format - expected '$START-$END $CHAR: $PASSWORD'",
                line_index + 1
            )
        };

        let mut line_parts = line.split(": ");
        let password_policy_str = line_parts.next().ok_or_else(on_error)?;
        let password = line_parts.next().ok_or_else(on_error)?;

        let mut policy_parts = password_policy_str.split(' ');
        let mut occurrences_parts = policy_parts.next().ok_or_else(on_error)?.split('-');
        let policy_char = policy_parts
            .next()
            .ok_or_else(on_error)?
            .chars()
            .next()
            .ok_or_else(on_error)?;

        let policy_start = occurrences_parts
            .next()
            .ok_or_else(on_error)?
            .parse::<usize>()
            .map_err(|_| on_error())?;
        let policy_end = occurrences_parts
            .next()
            .ok_or_else(on_error)?
            .parse::<usize>()
            .map_err(|_| on_error())?;

        if input.is_part_one() {
            let actual_occurrences = password.chars().filter(|&c| c == policy_char).count();
            if (policy_start..=policy_end).contains(&actual_occurrences) {
                valid_passwords += 1;
            }
        } else {
            let correct_count = password
                .chars()
                .enumerate()
                .filter(|(index, c)| {
                    (policy_start == index + 1 || policy_end == index + 1) && *c == policy_char
                })
                .count();
            if correct_count == 1 {
                valid_passwords += 1;
            }
        }
    }
    Ok(valid_passwords)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_one_error, test_part_two, test_part_two_error};

    test_part_one!("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc" => 2);
    test_part_two!("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc" => 1);

    test_part_one_error!("1- b: asdf" => "Line 1: Invalid format - expected '$START-$END $CHAR: $PASSWORD'");
    test_part_two_error!("1-3 a: asdf\nhi\n" => "Line 2: Invalid format - expected '$START-$END $CHAR: $PASSWORD'");

    let real_input = include_str!("day02_input.txt");
    test_part_one!(real_input => 636);
    test_part_two!(real_input => 588);
}
