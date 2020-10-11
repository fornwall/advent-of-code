fn solution(input_string: &str, index_offset_computer: fn(usize) -> usize) -> Result<u32, String> {
    let digits: Vec<u32> = input_string
        .chars()
        .map(|c| c.to_digit(10).ok_or("Invalid input - not all digits"))
        .collect::<Result<_, _>>()?;
    if digits.len() > 10_000 {
        return Err("Invalid input - too long".to_string());
    }
    Ok(digits
        .iter()
        .enumerate()
        .map(|(index, &digit)| {
            if digit == digits[(index + index_offset_computer(digits.len())) % digits.len()] {
                digit
            } else {
                0
            }
        })
        .sum())
}

pub fn part1(input_string: &str) -> Result<u32, String> {
    solution(input_string, |_| 1)
}

pub fn part2(input_string: &str) -> Result<u32, String> {
    solution(input_string, |len| len / 2)
}

#[test]
fn test_part1() {
    assert_eq!(Ok(3), part1("1122"));
    assert_eq!(Ok(4), part1("1111"));
    assert_eq!(Ok(0), part1("1234"));
    assert_eq!(Ok(9), part1("91212129"));
    assert_eq!(Ok(1029), part1(include_str!("day01_input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(Ok(6), part2("1212"));
    assert_eq!(Ok(0), part2("1221"));
    assert_eq!(Ok(4), part2("123425"));
    assert_eq!(Ok(12), part2("123123"));
    assert_eq!(Ok(4), part2("12131415"));
    assert_eq!(Ok(1220), part2(include_str!("day01_input.txt")));
}
