use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Program {
    memory: Vec<i64>,
    instruction_pointer: usize,
    pub output_values: Vec<i64>,
    input_values: VecDeque<i64>,
    halted: bool,
    requires_input_to: Option<usize>,
}

impl Program {
    pub fn parse(input: &str) -> Program {
        Program {
            memory: input
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect(),
            instruction_pointer: 0,
            output_values: Vec::new(),
            input_values: VecDeque::new(),
            halted: false,
            requires_input_to: None,
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn run(&mut self) -> i64 {
        if self.requires_input_to != None {
            panic!("Cannot run program requiring input");
        }

        while !self.halted && self.requires_input_to == None {
            self.evaluate();
        }
        self.memory[0]
    }

    pub fn input(&mut self, input_value: i64) {
        if let Some(save_address) = self.requires_input_to {
            self.memory[save_address] = input_value;
            self.requires_input_to = None;
        } else {
            self.input_values.push_back(input_value);
        }
    }

    pub fn patch(&mut self, index: usize, value: i64) {
        self.memory[index] = value;
    }

    fn parameter_value(&self, opcode_and_parameter_modes: i64, parameter_position: u32) -> i64 {
        fn is_immediate(opcode_and_parameter_modes: i64, position: u32) -> bool {
            let divider = 10_i64.pow(position + 1);
            (opcode_and_parameter_modes / divider) % 10 == 1
        }

        let parameter = self.memory[self.instruction_pointer + parameter_position as usize];
        if is_immediate(opcode_and_parameter_modes, parameter_position) {
            parameter
        } else {
            self.memory[parameter as usize]
        }
    }

    fn evaluate(&mut self) {
        let opcode_and_parameter_modes = self.memory[self.instruction_pointer];
        let opcode = opcode_and_parameter_modes % 100;
        match opcode {
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
            }
            3 => {
                // Takes a single integer as input and saves it to the address given by its only parameter.
                let save_address = self.memory[self.instruction_pointer + 1];
                if self.input_values.is_empty() {
                    self.requires_input_to = Some(save_address as usize);
                } else {
                    self.memory[save_address as usize] =
                        self.input_values.pop_front().expect("No available input");
                }
                self.instruction_pointer += 2;
            }
            4 => {
                // Takes a single integer as input and saves it to the address given by its only parameter.
                self.output_values
                    .push(self.parameter_value(opcode_and_parameter_modes, 1));
                self.instruction_pointer += 2;
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
            }
            99 => {
                self.halted = true;
            }
            _ => {
                panic!("Invalid opcode: {}", opcode);
            }
        }
    }
}
