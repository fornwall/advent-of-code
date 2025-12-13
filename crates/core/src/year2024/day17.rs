use crate::common::array_deque::ArrayDeque;
use crate::common::array_stack::ArrayStack;
use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<String, String> {
    let mut program = ArrayStack::<32, u8>::new();
    let mut registers = [0_u64; 3];
    let mut current_register = 0;
    for line in input.text.lines() {
        if let Some(reg_value) = line.strip_prefix("Register ") {
            registers[current_register] = reg_value[3..].parse().map_err(|_| on_error())?;
            current_register += 1;
        } else if let Some(program_str) = line.strip_prefix("Program: ") {
            for n in program_str.split(',') {
                program.push(n.parse().map_err(|_| on_error())?)?;
            }
        }
    }

    let mut computer = Computer {
        program: program.slice(),
        instruction_pointer: 0,
        registers,
    };

    if input.is_part_one() {
        let mut result = String::new();
        while let Some(output) = computer.run_for_output() {
            if !result.is_empty() {
                result.push(',');
            }
            result.push_str(&format!("{output}"));
        }
        Ok(result)
    } else {
        if (
            computer.program[computer.program.len() - 2],
            computer.program[computer.program.len() - 1],
        ) != (3, 0)
        {
            return Err("Program does not end with 'jnz 0'".to_string());
        }

        // Program ends with:
        //   adv 3        which can be written as A >>= 4
        //   jnz 0 .      which ends if A is zero
        // and the whole program can be written as:
        //   loop {
        //     B = ..(computed from A)..
        //     output(B);
        //     A >>= 3;
        //     if (A == 0) break;
        //   }
        // So the last loop iteration, which outputs the last program instruction, can only have the lowest 0..3 bits set (as A>>3 is zero).
        // And the second to last last output can only have bits 0..6 bits set (as A>>3 twice is zero).
        // So starting from the last instruction we can determine which of the 0..3 lowest bits can give the desired last instruction output,
        // then determine the next bits 3..6 which gives the second to last output, and so on.
        let mut stack = ArrayDeque::<320, (u64, u64)>::new();
        stack.push_back((computer.program.len() as u64, 0))?;
        while let Some((offset_from_end, register_a_bits_so_far)) = stack.pop_front() {
            for first_three_bits in 0..=0b111 {
                let register_a_next_three_bits = (register_a_bits_so_far << 3) | first_three_bits;
                computer.registers[0] = register_a_next_three_bits;
                computer.instruction_pointer = 0;
                if computer.run_for_output()
                    == Some(computer.program[offset_from_end as usize - 1] as u64)
                {
                    if offset_from_end - 1 == 0 {
                        return Ok(format!("{register_a_next_three_bits}"));
                    }
                    stack.push_back((offset_from_end - 1, register_a_next_three_bits))?;
                }
            }
        }

        Err("No solution found".to_string())
    }
}

struct Computer<'a> {
    instruction_pointer: usize,
    registers: [u64; 3],
    program: &'a [u8],
}

impl Computer<'_> {
    fn combo_operand(&self) -> u64 {
        let val = self.program[self.instruction_pointer + 1];
        match val {
            0..=3 => val as u64,
            4..=6 => self.registers[val as usize - 4],
            _ => {
                unreachable!();
            }
        }
    }

    const fn literal_operand(&self) -> u64 {
        self.program[self.instruction_pointer + 1] as u64
    }

    fn run_for_output(&mut self) -> Option<u64> {
        loop {
            if (self.instruction_pointer + 1) >= self.program.len() {
                return None;
            }
            let instruction_val = self.program[self.instruction_pointer];
            match instruction_val {
            0 /* adv */ => {
                self.registers[0] >>= self.combo_operand() as u32;
            }
            1 /* bxl */ => {
                self.registers[1] ^= self.literal_operand();
            }
            2 /* bst */ => {
                self.registers[1] = self.combo_operand() % 8;
            }
            3 /* jnz */ => {
                if self.registers[0] != 0 {
                    self.instruction_pointer = self.literal_operand() as usize;
                    continue;
                }
            }
            4 /* bxc */ => {
                self.registers[1] ^= self.registers[2];
            }
            5 /* out */ => {
                let result = Some(self.combo_operand() % 8);
                self.instruction_pointer += 2;
                return result;
            }
            6 /* bdv */ => {
                self.registers[1] = self.registers[0] >> self.combo_operand() as u32;
            }
            7 /* cdv */ => {
                self.registers[2] = self.registers[0] >> self.combo_operand() as u32;
            }
            _ => {
                return None;
            }
        }
            self.instruction_pointer += 2;
        }
    }
}

#[test]
pub fn tests() {
    let test_input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    test_part_one!(test_input => "4,6,3,5,6,3,5,2,1,0".to_string());

    let real_input = include_str!("day17_input.txt");
    test_part_one!(real_input => "3,7,1,7,2,1,0,6,3".to_string());
    test_part_two!(real_input => "37221334433268".to_string());
}
