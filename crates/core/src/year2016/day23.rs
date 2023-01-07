use super::assembunny::{Computer, Instruction, ValueOrRegister, Word};
use crate::input::Input;

fn factorial(num: Word) -> Word {
    match num {
        0 | 1 => 1,
        _ => factorial(num - 1) * num,
    }
}

pub fn solve(input: &Input) -> Result<Word, String> {
    let mut computer = Computer::parse(input.text)?;
    let register_a_value = input.part_values(7, 12);
    if computer.instructions.len() > 20 {
        if let Instruction::Copy(ValueOrRegister::Value(a), _) = computer.instructions[19] {
            if let Instruction::Jump(ValueOrRegister::Value(b), _) = computer.instructions[20] {
                return Ok(a * b + factorial(register_a_value));
            }
        }
    }

    computer.registers[0] = register_a_value;
    Ok(computer.execute())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example_input = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";
    test_part_one!(example_input => 3);

    let real_input = include_str!("day23_input.txt");
    test_part_one!(real_input => 13_685);
    test_part_two!(real_input => 479_010_245);
}
