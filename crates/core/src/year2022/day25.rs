use crate::input::Input;

pub fn solve(_input: &mut Input) -> Result<i32, String> {
    Ok(0)
}

#[test]
pub fn tests() {
    use crate::input::test_part_one;

    let real_input = include_str!("day25_input.txt");
    test_part_one!(real_input => 0);
}
