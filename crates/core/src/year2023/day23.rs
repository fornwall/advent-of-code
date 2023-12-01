use crate::input::Input;

pub const fn solve(_input: &Input) -> Result<u64, String> {
    Ok(0)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "0";
    test_part_one_no_allocations!(test_input => 0);
    test_part_two_no_allocations!(test_input => 0);

    let real_input = include_str!("day23_input.txt");
    test_part_one_no_allocations!(real_input => 0);
    test_part_two_no_allocations!(real_input => 0);
}
