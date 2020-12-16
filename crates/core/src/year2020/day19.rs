use crate::input::Input;

pub fn solve(_input: &mut Input) -> Result<u64, String> {
    Ok(0)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example_part_one = "";
    test_part_one!(example_part_one => 0);
    let example_part_two = "";
    test_part_two!(example_part_two => 0);

    // let real_input = include_str!("day19_input.txt");
    // test_part_one!(real_input => 0);
    // test_part_two!(real_input => 0);
}
