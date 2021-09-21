use super::assembunny::{Computer, Instruction, ValueOrRegister};
use crate::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let computer = Computer::parse(input.text)?;
    if let Instruction::Copy(ValueOrRegister::Value(a), _register) = computer.instructions[1] {
        if let Instruction::Copy(ValueOrRegister::Value(b), _register) = computer.instructions[2] {
            let start_value = a * b;
            for initial_value in 1..1000 {
                let value = start_value + initial_value;
                if format!("{:b}", value).replace("10", "").is_empty() {
                    return Ok(initial_value as u32);
                }
            }
        }
    }

    Err("Input does not match expectations".to_string())
}

#[test]
pub fn tests() {
    use crate::input::test_part_one;

    let real_input = include_str!("day25_input.txt");
    test_part_one!(real_input => 196);
}
