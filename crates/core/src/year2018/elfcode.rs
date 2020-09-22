#[derive(Copy, Clone, PartialEq)]
pub struct Registers {
    pub values: [u64; 6],
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    opcode: Opcode,
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

pub struct Program {
    instruction_pointer_index: u8,
    pub instructions: Vec<Instruction>,
    pub registers: Registers,
}

impl Program {
    pub const fn instruction_pointer(&self) -> u64 {
        self.registers.values[self.instruction_pointer_index as usize]
    }

    pub fn execute_one_instruction(&mut self) -> bool {
        let ip = self.instruction_pointer();
        if ip as usize >= self.instructions.len() {
            return false;
        }
        let instruction = self.instructions[ip as usize];
        self.registers.apply(
            instruction.opcode,
            instruction.a,
            instruction.b,
            instruction.c,
        );
        self.registers.values[self.instruction_pointer_index as usize] += 1;
        true
    }

    pub fn parse(input_string: &str) -> Result<Self, String> {
        let mut lines = input_string.lines();
        let first_line = lines.next().unwrap();

        let instruction_pointer_index = (&first_line[4..]).parse::<u8>().unwrap();

        let mut instructions = Vec::new();
        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let opcode = opcode_from_str(parts[0])?;
            let a = parts[1].parse::<u64>().unwrap();
            let b = parts[2].parse::<u64>().unwrap();
            let c = parts[3].parse::<u64>().unwrap();
            instructions.push(Instruction { opcode, a, b, c });
        }

        Ok(Self {
            instruction_pointer_index,
            instructions,
            registers: Registers::new(),
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Opcode {
    Addr, // (add register) stores into register C the result of adding register A and register B
    Addi, // (add immediate) stores into register C the result of adding register A and value B.
    Mulr, // (multiply register) stores into register C the result of multiplying register A and register B.
    Muli, // (multiply immediate) stores into register C the result of multiplying register A and value B.
    Banr, // (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
    Bani, // (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
    Borr, // (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
    Bori, // (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
    Setr, // (set register) copies the contents of register A into register C. (Input B is ignored.)
    Seti, // (set immediate) stores value A into register C. (Input B is ignored.)
    Gtir, // (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    Gtri, // (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    Gtrr, // (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
    Eqir, // (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    Eqri, // (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    Eqrr, // (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
}

fn opcode_from_str(name: &str) -> Result<Opcode, String> {
    Ok(match name {
        "addr" => Opcode::Addr,
        "addi" => Opcode::Addi,
        "mulr" => Opcode::Mulr,
        "muli" => Opcode::Muli,
        "banr" => Opcode::Banr,
        "bani" => Opcode::Bani,
        "borr" => Opcode::Borr,
        "bori" => Opcode::Bori,
        "setr" => Opcode::Setr,
        "seti" => Opcode::Seti,
        "gtir" => Opcode::Gtir,
        "gtri" => Opcode::Gtri,
        "gtrr" => Opcode::Gtrr,
        "eqir" => Opcode::Eqir,
        "eqri" => Opcode::Eqri,
        "eqrr" => Opcode::Eqrr,
        _ => {
            return Err(format!("No matching opcode: {}", name));
        }
    })
}

impl Registers {
    const fn new() -> Self {
        Self {
            values: [0, 0, 0, 0, 0, 0],
        }
    }

    fn reg(&mut self, index: u64) -> u64 {
        self.values[index as usize]
    }

    fn apply(&mut self, opcode: Opcode, a: u64, b: u64, c: u64) {
        let c = c as usize;
        self.values[c] = match opcode {
            Opcode::Addr => self.reg(a) + self.reg(b),
            Opcode::Addi => self.reg(a) + b,
            Opcode::Mulr => self.reg(a) * self.reg(b),
            Opcode::Muli => self.reg(a) * b,
            Opcode::Banr => self.reg(a) & self.reg(b),
            Opcode::Bani => self.reg(a) & b,
            Opcode::Borr => self.reg(a) | self.reg(b),
            Opcode::Bori => self.reg(a) | b,
            Opcode::Setr => self.reg(a),
            Opcode::Seti => a,
            Opcode::Gtir => (a > self.reg(b)) as u64,
            Opcode::Gtri => (self.reg(a) > b) as u64,
            Opcode::Gtrr => (self.reg(a) > self.reg(b)) as u64,
            Opcode::Eqir => (a == self.reg(b)) as u64,
            Opcode::Eqri => (self.reg(a) == b) as u64,
            Opcode::Eqrr => (self.reg(a) == self.reg(b)) as u64,
        }
    }
}
