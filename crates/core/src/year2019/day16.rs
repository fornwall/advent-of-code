use crate::input::Input;

fn parse_digits(input_string: &str) -> Result<Vec<i32>, String> {
    let result = input_string
        .chars()
        .map(|b| {
            b.to_digit(10)
                .map(|b| b as i32)
                .ok_or_else(|| "Invalid input".to_string())
        })
        .collect::<Result<Vec<_>, String>>()?;
    if result.len() > 1000 {
        return Err("Too big input".to_string());
    }
    Ok(result)
}

pub fn solve(input: &Input) -> Result<String, String> {
    let mut digits = parse_digits(input.text)?;

    if input.is_part_one() {
        const PHASES: usize = 100;
        let mut new_digits = vec![0; digits.len()];
        for _ in 0..PHASES {
            for (index, digit) in new_digits.iter_mut().enumerate() {
                let positives: i32 = (index..digits.len())
                    .step_by((index + 1) * 4)
                    .flat_map(|i| digits.iter().skip(i).take(index + 1))
                    .sum();
                let negatives: i32 = ((index + ((index + 1) * 2))..digits.len())
                    .step_by((index + 1) * 4)
                    .flat_map(|i| digits.iter().skip(i).take(index + 1))
                    .sum();
                *digit = (positives - negatives).abs() % 10;
            }

            std::mem::swap(&mut digits, &mut new_digits);
        }

        Ok(digits
            .iter()
            .take(8)
            .map(|b| ((b + 48) as u8) as char)
            .collect())
    } else {
        let offset = input
            .text
            .get(0..7)
            .map(str::parse)
            .ok_or_else(|| "Invalid input".to_string())?
            .map_err(|_| "Invalid input".to_string())?;

        let times_to_repeat = 10000;
        let message_length = input.text.len() * times_to_repeat;
        let end_sequence_length = message_length.checked_sub(offset).ok_or("Too big offset")?;

        let mut end_sequence: Vec<i32> = digits
            .into_iter()
            .cycle()
            .skip(offset)
            .take(end_sequence_length)
            .collect();

        for _ in 0..100 {
            for i in 0..(end_sequence.len() - 1) {
                let index = end_sequence_length - i - 1;
                end_sequence[index - 1] = (end_sequence[index - 1] + end_sequence[index]) % 10;
            }
        }

        Ok(end_sequence
            .iter()
            .take(8)
            .map(|&b| ((b + 48) as u8) as char)
            .collect())
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("80871224585914546619083218645595" => "24176176".to_string());

    let real_input = include_str!("day16_input.txt");
    test_part_one!(real_input => "37153056".to_string());
    test_part_two!(real_input => "60592199".to_string());
}
