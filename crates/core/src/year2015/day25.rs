use crate::Input;

pub fn solve(_input: &mut Input) -> Result<u32, String> {
    Err("Not yet implemented".to_string())
}

#[test]
pub fn tests() {
    use crate::test_part_one_error;

    let real_input = include_str!("day25_input.txt");
    test_part_one_error!(real_input => "Not yet implemented".to_string());
}
