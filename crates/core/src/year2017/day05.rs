use crate::common::parser::parse_lines;
use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let jump_change_computer = |offset| {
        if input.is_part_one() || offset < 3 {
            1
        } else {
            -1
        }
    };

    let mut jumps: Vec<i32> = parse_lines::<i32>(input.text)?;

    let mut position: i32 = 0;
    for step in 1..100_000_000 {
        let old_position = position;
        position += jumps[position as usize];
        if position < 0 || position as usize >= jumps.len() {
            return Ok(step);
        }
        jumps[old_position as usize] += jump_change_computer(jumps[old_position as usize]);
    }
    Err("No solution found".to_string())
}

#[test]
fn test() {
    use crate::input::{test_part_one, test_part_two};
    let real_input = include_str!("day05_input.txt");
    test_part_one!(real_input => 374_269);
    test_part_two!(real_input => 27_720_699);
}
