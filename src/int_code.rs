use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
pub struct Program {
    memory: HashMap<usize, i64>,
    instruction_pointer: usize,
    pub output_values: Vec<i64>,
    input_values: VecDeque<i64>,
    halted: bool,
    requires_input_to: Option<usize>,
    relative_base: i64,
}

enum Parameter {
    Value(i64),
    Address(usize),
}

impl Program {
    pub fn parse(input: &str) -> Program {
        Program {
            memory: input
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .enumerate()
                .collect(),
            instruction_pointer: 0,
            output_values: Vec::new(),
            input_values: VecDeque::new(),
            halted: false,
            requires_input_to: None,
            relative_base: 0,
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn run(&mut self) -> i64 {
        if self.requires_input_to != None {
            panic!("Cannot run program requiring input");
        } else if self.halted {
            panic!("Cannot run halted program");
        }

        while !self.halted && self.requires_input_to == None {
            self.evaluate();
        }
        self.read_memory(0)
    }

    pub fn input(&mut self, input_value: i64) {
        if let Some(save_address) = self.requires_input_to {
            self.write_memory(save_address, input_value);
            self.requires_input_to = None;
        } else {
            self.input_values.push_back(input_value);
        }
    }

    pub fn input_string(&mut self, input_string: &str) {
        input_string.chars().for_each(|c| {
            self.input(c as i64);
        });
    }

    fn parameter_mode(
        &self,
        opcode_and_parameter_modes: i64,
        parameter_position: u32,
    ) -> Parameter {
        let parameter = self.read_memory(self.instruction_pointer + parameter_position as usize);
        let divider = 10_i64.pow(parameter_position + 1);
        let mode = ((opcode_and_parameter_modes / divider) % 10) as u8;
        match mode {
            1 => Parameter::Value(parameter),
            2 => Parameter::Address((parameter + self.relative_base) as usize),
            _ => Parameter::Address(parameter as usize),
        }
    }

    fn output_location(&self, opcode_and_parameter_modes: i64, parameter_position: u32) -> usize {
        if let Parameter::Address(location) =
            self.parameter_mode(opcode_and_parameter_modes, parameter_position)
        {
            return location;
        }
        panic!("Output is not by address");
    }

    fn parameter_value(&self, opcode_and_parameter_modes: i64, parameter_position: u32) -> i64 {
        match self.parameter_mode(opcode_and_parameter_modes, parameter_position) {
            Parameter::Value(value) => value,
            Parameter::Address(location) => self.read_memory(location),
        }
    }

    fn evaluate(&mut self) {
        let opcode_and_parameter_modes = self.read_memory(self.instruction_pointer);
        let opcode = opcode_and_parameter_modes % 100;
        match opcode {
            1 | 2 => {
                let parameter1 = self.parameter_value(opcode_and_parameter_modes, 1);
                let parameter2 = self.parameter_value(opcode_and_parameter_modes, 2);
                let output_location = self.output_location(opcode_and_parameter_modes, 3);
                self.write_memory(
                    output_location as usize,
                    if opcode == 1 {
                        parameter1 + parameter2
                    } else {
                        parameter1 * parameter2
                    },
                );
                self.instruction_pointer += 4;
            }
            3 => {
                // Takes a single integer as input and saves it to the address given by its only parameter.
                let output_location = self.output_location(opcode_and_parameter_modes, 1);
                if let Some(input_value) = self.input_values.pop_front() {
                    self.write_memory(output_location as usize, input_value);
                } else {
                    self.requires_input_to = Some(output_location as usize);
                }
                self.instruction_pointer += 2;
            }
            4 => {
                // Opcode 4 outputs the value of its only parameter.
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

                let output_location = self.output_location(opcode_and_parameter_modes, 3);
                self.write_memory(output_location as usize, output_value);
                self.instruction_pointer += 4;
            }
            9 => {
                self.relative_base += self.parameter_value(opcode_and_parameter_modes, 1);
                self.instruction_pointer += 2;
            }
            99 => {
                self.halted = true;
            }
            _ => {
                panic!("Invalid opcode: {}", opcode);
            }
        }
    }

    fn read_memory(&self, address: usize) -> i64 {
        *self.memory.get(&address).unwrap_or(&0i64)
    }

    pub fn write_memory(&mut self, address: usize, value: i64) {
        self.memory.insert(address, value);
    }
}
