use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    Ok(input.text.len() as u32)
}

#[test]
pub fn tests() {
    use crate::test_part_one; // , test_part_one_error, test_part_two, test_part_two_error};

    test_part_one!("" => 0);

    // test_part_one_error!("1- b: asdf" => "Line 1: Invalid format - expected '$START-$END $CHAR: $PASSWORD'");
    // test_part_two_error!("1-3 a: asdf\nhi\n" => "Line 2: Invalid format - expected '$START-$END $CHAR: $PASSWORD'");

    // let real_input = include_str!("day04_input.txt");
    // test_part_one!(real_input => 636);
}
