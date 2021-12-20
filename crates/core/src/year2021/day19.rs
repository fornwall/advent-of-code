use crate::input::Input;

pub fn solve(_input: &mut Input) -> Result<String, String> {
    Ok("".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "--- scanner 0 ---
0,2
4,1
3,3

--- scanner 1 ---
-1,-1
-5,0
-2,1";
    test_part_one!(example => "".to_string());
    test_part_two!(example => "".to_string());

    let real_input = include_str!("day19_input.txt");
    test_part_one!(real_input => "".to_string());
    test_part_two!(real_input => "".to_string());
}
