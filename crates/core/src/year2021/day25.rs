use crate::input::Input;

pub fn solve(_input: &mut Input) -> Result<String, String> {
    Ok("".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("" => "".to_string());
    test_part_two!("" => "".to_string());

    let real_input = include_str!("day25_input.txt");
    test_part_one!(real_input => "".to_string());
    test_part_two!(real_input => "".to_string());
}
