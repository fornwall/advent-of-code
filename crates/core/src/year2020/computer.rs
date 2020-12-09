pub type Word = i32;

#[derive(Copy, Clone)]
pub enum Instruction {
    Acc(Word),
    Jmp(Word),
    Nop(Word),
}

pub struct Computer {
    pub instructions: Vec<Instruction>,
    pub accumulator: Word,
    pub instruction_pointer: Word,
}

impl Computer {
    pub fn parse(program_text: &str) -> Result<Self, String> {
        let instructions = program_text
            .lines()
            .enumerate()
            .map(|(line_idx, line)| {
                if line.len() < 6 {
                    return Err(format!(
                        "Line {}: Too short line ({})",
                        line_idx + 1,
                        line.len()
                    ));
                }

                let argument = line[4..]
                    .parse::<Word>()
                    .map_err(|e| format!("Line {}: Cannot parse argument - {}", line_idx + 1, e))?;

                Ok(match &line[0..3] {
                    "acc" => Instruction::Acc(argument),
                    "jmp" => Instruction::Jmp(argument),
                    "nop" => Instruction::Nop(argument),
                    _ => {
                        return Err(format!(
                            "Line {}: Invalid line not starting with acc/jmp/nop",
                            line_idx + 1
                        ));
                    }
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            instructions,
            accumulator: 0,
            instruction_pointer: 0,
        })
    }

    pub fn execute_instruction(&mut self) -> Result<(), String> {
        if self.has_exited() {}
        match self.instructions.get(self.instruction_pointer as usize) {
            Some(Instruction::Acc(parameter)) => {
                self.accumulator += parameter;
            }
            Some(Instruction::Jmp(parameter)) => {
                self.instruction_pointer += parameter;
                return Ok(());
            }
            Some(Instruction::Nop(_)) => {}
            None => {
                return Err("Cannot executed an exited program".to_string());
            }
        };

        self.instruction_pointer += 1;
        Ok(())
    }

    pub fn has_exited(&self) -> bool {
        self.instruction_pointer < 0 || self.instruction_pointer >= self.instructions.len() as Word
    }
}
