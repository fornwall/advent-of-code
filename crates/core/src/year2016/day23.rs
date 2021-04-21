use super::assembunny::{Computer, Word};
use crate::Input;

pub fn solve(input: &mut Input) -> Result<Word, String> {
    let mut computer = Computer::parse(input.text)?;
    computer.registers[0] = input.part_values(7, 12);
    Ok(computer.execute())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day23_input.txt");
    test_part_one!(real_input => 13_685);
    test_part_two!(real_input => 479_010_245);
}
