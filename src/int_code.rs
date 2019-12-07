use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Program {
    memory: Vec<i64>,
    instruction_pointer: usize,
    pub last_output: i64,
    pub input_values: VecDeque<i64>,
}

impl Program {
    pub fn parse(input: &str) -> Program {
        Program {
            memory: input
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect(),
            instruction_pointer: 0,
            last_output: 0,
            input_values: VecDeque::new(),
        }
    }

    pub fn run(&mut self) -> i64 {
        while self.evaluate() {}
        self.memory[0]
    }

    pub fn patch(&mut self, index: usize, value: i64) {
        self.memory[index] = value;
    }

    fn is_immediate(opcode_and_parameter_modes: i64, position: u32) -> bool {
        let divider = 10_i64.pow(position + 1);
        (opcode_and_parameter_modes / divider) % 10 == 1
    }

    fn parameter_value(&self, opcode_and_parameter_modes: i64, parameter_position: u32) -> i64 {
        let parameter = self.memory[self.instruction_pointer + parameter_position as usize];
        if Program::is_immediate(opcode_and_parameter_modes, parameter_position) {
            parameter
        } else {
            self.memory[parameter as usize]
        }
    }

    fn evaluate(&mut self) -> bool {
        let opcode_and_parameter_modes = self.memory[self.instruction_pointer];
        let opcode = opcode_and_parameter_modes % 100;
        match opcode {
            99 => false,
            1 | 2 => {
                let parameter1 = self.parameter_value(opcode_and_parameter_modes, 1);
                let parameter2 = self.parameter_value(opcode_and_parameter_modes, 2);
                let output_location = self.memory[self.instruction_pointer + 3] as usize;
                self.memory[output_location] = if opcode == 1 {
                    parameter1 + parameter2
                } else {
                    parameter1 * parameter2
                };
                self.instruction_pointer += 4;
                true
            }
            3 => {
                // Takes a single integer as input and saves it to the address given by its only parameter.
                let save_address = self.memory[self.instruction_pointer + 1];
                self.memory[save_address as usize] =
                    self.input_values.pop_front().expect("No available input");
                self.instruction_pointer += 2;
                true
            }
            4 => {
                // Takes a single integer as input and saves it to the address given by its only parameter.
                self.last_output = self.parameter_value(opcode_and_parameter_modes, 1);
                self.instruction_pointer += 2;
                true
            }
            5 | 6 => {
                // Opcode 5 is is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the
                // value from the second parameter. Otherwise, it does nothing.
                // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer
                // to the value from the second parameter. Otherwise, it does nothing.
                let jump_if = opcode == 5;
                let parameter_1_true = self.parameter_value(opcode_and_parameter_modes, 1) != 0;
                if parameter_1_true == jump_if {
                    self.instruction_pointer =
                        self.parameter_value(opcode_and_parameter_modes, 2) as usize;
                } else {
                    self.instruction_pointer += 3;
                }
                true
            }
            7 | 8 => {
                // Opcode 7 is less than: if the first parameter is less than the second parameter,
                // it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                // Opcode 8 is equals: if the first parameter is equal to the second parameter,
                // it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                let parameter_1 = self.parameter_value(opcode_and_parameter_modes, 1);
                let parameter_2 = self.parameter_value(opcode_and_parameter_modes, 2);
                let output_value = if (opcode == 7 && (parameter_1 < parameter_2))
                    || (opcode == 8 && (parameter_1 == parameter_2))
                {
                    1
                } else {
                    0
                };

                let save_address = self.memory[self.instruction_pointer + 3];
                self.memory[save_address as usize] = output_value;
                self.instruction_pointer += 4;
                true
            }
            _ => {
                panic!("Invalid opcode: {}", opcode);
            }
        }
    }
}
