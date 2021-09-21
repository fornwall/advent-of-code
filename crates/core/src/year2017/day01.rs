use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let index_offset_computer = if input.is_part_one() {
        |_| 1
    } else {
        |len| len / 2
    };

    let digits: Vec<u32> = input
        .text
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

#[test]
fn test() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("1122" => 3);
    test_part_one!("1111" => 4);
    test_part_one!("1234" => 0);
    test_part_one!("91212129" => 9);

    test_part_two!("1212" => 6);
    test_part_two!("1221" => 0);
    test_part_two!("123425" => 4);
    test_part_two!("123123" => 12);
    test_part_two!("12131415" => 4);

    let input = include_str!("day01_input.txt");
    test_part_one!(input => 1029);
    test_part_two!(input => 1220);
}
