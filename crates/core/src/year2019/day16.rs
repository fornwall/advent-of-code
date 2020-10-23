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

pub fn part1(input_string: &str) -> Result<String, String> {
    const PHASES: usize = 100;
    let mut digits = parse_digits(input_string)?;
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
}

/// Length of input: 650
/// We now repeat it 10000 times, so length is 6,500,000 (six and a half millions).
/// "The first seven digits of your initial input signal also represent the message offset."
/// Offset: 5,973,431
///
/// Matrix is _diagonal_:
/// 1  0 -1  0 [0  0  1  1  0  0  -1 -1] [repeating and extending]
/// 0  1  1  0  0 -1 -1  0  0  [..]
/// 0  0  1  1  1  0  0  0 -1 -1 -1 [...
///
/// The offset is huge, so simple pattern at end:
/// 5,973,430 zeros, 5,973,431 ones.
/// - Simplifying: Diagonal (so only depends on later digits)
/// - Just ones: So simple sum (and mod 10) on later digits.
/// - Repeating: Input is repeated every 650 digits.
///   Every time input is repeated just adds one digit: sum(digits)%650.
///   Last 650 digits easy to compute (just matrix multiply 100 times).
///
/// 1 1 1 1 1 1   d1
/// 0 1 1 1 1 1   d2
/// 0 0 1 1 1 1 * d3
/// 0 0 0 1 1 1   d4
/// 0 0 0 0 1 1   d5
/// 0 0 0 0 0 1   d6
/// [...]
///
/// Starting from the end, the last element is always the same.
/// The next element (second latest) is itself plus last element.
/// The next element (third latest) is itself plus second latest and last element=itself + previous element!
pub fn part2(input_string: &str) -> Result<String, String> {
    let offset = input_string
        .get(0..7)
        .map(|slice| slice.parse::<usize>())
        .ok_or_else(|| "Invalid input".to_string())?
        .map_err(|_| "Invalid input".to_string())?;
    let digits = parse_digits(input_string)?;

    let times_to_repeat = 10000;
    let end_sequence_length = input_string.len() * times_to_repeat - offset as usize;

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

#[test]
pub fn tests_part1() {
    assert_eq!(
        part1("80871224585914546619083218645595"),
        Ok("24176176".to_string())
    );
    assert_eq!(
        part1(include_str!("day16_input.txt")),
        Ok("37153056".to_string())
    );
}

#[test]
fn tests_part2() {
    assert_eq!(
        part2(include_str!("day16_input.txt")),
        Ok("60592199".to_string())
    );
}
