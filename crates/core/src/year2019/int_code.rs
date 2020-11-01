use std::collections::{HashMap, VecDeque};

pub type Word = i64;

#[derive(Clone)]
pub struct Program {
    memory: HashMap<usize, Word>,
    instruction_pointer: usize,
    output_values: Vec<Word>,
    input_values: VecDeque<Word>,
    halted: bool,
    requires_input_to: Option<usize>,
    relative_base: Word,
}

enum Parameter {
    Value(Word),
    Address(usize),
}

impl Program {
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut memory: Vec<Word> = Vec::new();
        for word_string in input.trim().split(',') {
            match word_string.parse::<Word>() {
                Ok(value) => {
                    memory.push(value);
                }
                Err(error) => {
                    return Err(format!(
                        "Line 1: Unable to parse program word ({})",
                        error.to_string()
                    ));
                }
            }
        }
        Ok(Self {
            memory: memory.into_iter().enumerate().collect(),
            instruction_pointer: 0,
            output_values: Vec::new(),
            input_values: VecDeque::new(),
            halted: false,
            requires_input_to: None,
            relative_base: 0,
        })
    }

    pub const fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn run_until_halt_or_input(&mut self, max_instructions: u32) -> Result<(), String> {
        if self.requires_input_to != None {
            return Err("Cannot run program requiring input".to_string());
        } else if self.halted {
            return Err("Cannot run halted program".to_string());
        }

        let mut current_instruction = 0;
        while !self.halted && self.requires_input_to == None {
            self.evaluate()?;

            current_instruction += 1;
            if current_instruction == max_instructions {
                return Err(format!("Aborted after {} instructions", max_instructions));
            }
        }
        Ok(())
    }

    pub fn run_for_output(&mut self) -> Result<Vec<Word>, String> {
        self.run_until_halt_or_input(1_000_000_000)?;
        Ok(std::mem::replace(&mut self.output_values, Vec::new()))
    }

    pub fn run_for_output_limited(&mut self, max_instructions: u32) -> Result<Vec<Word>, String> {
        self.run_until_halt_or_input(max_instructions)?;
        Ok(std::mem::replace(&mut self.output_values, Vec::new()))
    }

    pub fn input(&mut self, input_value: Word) {
        if let Some(save_address) = self.requires_input_to {
            self.write_memory(save_address, input_value);
            self.requires_input_to = None;
        } else {
            self.input_values.push_back(input_value);
        }
    }

    pub fn input_string(&mut self, input_string: &str) {
        input_string.chars().for_each(|c| {
            self.input(c as Word);
        });
    }

    fn parameter_mode(
        &self,
        opcode_and_parameter_modes: Word,
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

    fn output_location(
        &self,
        opcode_and_parameter_modes: Word,
        parameter_position: u32,
    ) -> Result<usize, String> {
        if let Parameter::Address(location) =
            self.parameter_mode(opcode_and_parameter_modes, parameter_position)
        {
            return Ok(location);
        }
        Err("Invalid parameter mode for where to write".to_string())
    }

    fn parameter_value(&self, opcode_and_parameter_modes: Word, parameter_position: u32) -> Word {
        match self.parameter_mode(opcode_and_parameter_modes, parameter_position) {
            Parameter::Value(value) => value,
            Parameter::Address(location) => self.read_memory(location),
        }
    }

    fn evaluate(&mut self) -> Result<(), String> {
        let opcode_and_parameter_modes = self.read_memory(self.instruction_pointer);
        let opcode = opcode_and_parameter_modes % 100;
        match opcode {
            1 | 2 => {
                let parameter1 = self.parameter_value(opcode_and_parameter_modes, 1);
                let parameter2 = self.parameter_value(opcode_and_parameter_modes, 2);
                let output_location = self.output_location(opcode_and_parameter_modes, 3)?;
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
                let output_location = self.output_location(opcode_and_parameter_modes, 1)?;
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

                let output_location = self.output_location(opcode_and_parameter_modes, 3)?;
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
                return Err(format!("Invalid opcode: {}", opcode));
            }
        }

        Ok(())
    }

    pub fn read_memory(&self, address: usize) -> Word {
        *self.memory.get(&address).unwrap_or(&0_i64)
    }

    pub fn write_memory(&mut self, address: usize, value: Word) {
        self.memory.insert(address, value);
    }
}
