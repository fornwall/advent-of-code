use super::assembunny::{Computer, Word};
use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<Word, String> {
    let mut computer = Computer::parse(input.text)?;
    computer.registers[2] = input.part_values(0, 1);
    Ok(computer.execute())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day12_input.txt");
    test_part_one!(real_input => 318_020);
    test_part_two!(real_input => 9_227_674);
}
