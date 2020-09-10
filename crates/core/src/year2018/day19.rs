use std::env;

#[derive(Copy, Clone, PartialEq)]
struct Registers {
    values: [u64; 6],
}

struct Program {
    instruction_pointer_index: u8,
    instructions: Vec<(Opcode, u64, u64, u64)>,
    registers: Registers,
}

impl Program {
    fn instruction_pointer(&self) -> u64 {
        self.registers.values[self.instruction_pointer_index as usize]
    }

    fn execute(&mut self) -> u64 {
        while self.execute_one_instruction() {
            // Go on.
        }
        self.registers.values[0]
    }

    fn execute_one_instruction(&mut self) -> bool {
        let ip = self.instruction_pointer();
        if ip as usize >= self.instructions.len() {
            return false;
        }
        let instruction = self.instructions[ip as usize];
        self.registers
            .apply(instruction.0, instruction.1, instruction.2, instruction.3);
        self.registers.values[self.instruction_pointer_index as usize] += 1;
        true
    }

    fn optimize(&mut self) {
        for (line, instruction) in self.instructions.iter_mut().enumerate() {
            match instruction.0 {
                Opcode::Addi => {
                    if instruction.1 as u8 == self.instruction_pointer_index {
                        instruction.0 = Opcode::Seti;
                        instruction.1 = line as u64 + instruction.2;
                        instruction.2 = 0; // ignored
                    }
                }
                Opcode::Mulr => {
                    if instruction.1 as u8 == self.instruction_pointer_index
                        && instruction.2 as u8 == self.instruction_pointer_index
                    {
                        instruction.0 = Opcode::Seti;
                        instruction.1 = line as u64 * line as u64;
                        instruction.2 = 0; // ignored
                    }
                }
                Opcode::Muli => {
                    if instruction.1 as u8 == self.instruction_pointer_index {
                        instruction.0 = Opcode::Seti;
                        instruction.1 = line as u64 * instruction.2;
                        instruction.2 = 0; // ignored
                    }
                }
                _ => {}
            }
        }
    }

    fn pretty_print(&self, title: &str) {
        if env::var("ADVENT_DEBUG").is_err() {
            return;
        }

        println!("# {}", title);

        for (line, &instruction) in self.instructions.iter().enumerate() {
            print!("{:02}: ", line);

            // If target register is the instruction pointer:
            let goto = instruction.3 as u8 == self.instruction_pointer_index;
            if goto {
                print!("goto ");
            } else {
                print!("r{} = ", instruction.3)
            }

            let a_is_value = match instruction.0 {
                Opcode::Seti | Opcode::Gtir | Opcode::Eqir => true,
                _ => false,
            };
            let b_is_value = match instruction.0 {
                Opcode::Addi
                | Opcode::Muli
                | Opcode::Bani
                | Opcode::Bori
                | Opcode::Gtri
                | Opcode::Eqri => true,
                _ => false,
            };

            let a = if a_is_value {
                format!("{}", instruction.1)
            } else if instruction.1 as u8 == self.instruction_pointer_index {
                line.to_string()
            } else {
                format!("r{}", instruction.1)
            };

            let b = if b_is_value {
                format!("{}", instruction.2)
            } else if instruction.2 as u8 == self.instruction_pointer_index {
                //b_is_value = true
                // instruction.2 = line
                line.to_string()
            } else {
                format!("r{}", instruction.2)
            };

            let pretty = match instruction.0 {
                Opcode::Addr | Opcode::Addi => {
                    if a_is_value && b_is_value {
                        format!("{}", instruction.1 + instruction.2)
                    } else {
                        format!("{} + {}", a, b)
                    }
                }
                Opcode::Mulr | Opcode::Muli => {
                    if a_is_value && b_is_value {
                        format!("{}", instruction.1 * instruction.2)
                    } else {
                        format!("{} * {}", a, b)
                    }
                }
                Opcode::Setr | Opcode::Seti => a.to_string(),
                Opcode::Gtrr => {
                    // (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
                    format!("({} > {}) ? 1 : 0", a, b)
                }
                Opcode::Eqrr => {
                    // (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
                    format!("({} == {}) ? 1 : 0", a, b)
                }
                _ => {
                    panic!("Unhandled opcode at line: {}", line);
                }
            };
            print!("{}", pretty);
            if goto {
                print!(" + 1");
            }
            println!();
        }
    }

    fn parse(input_string: &str) -> Program {
        let mut lines = input_string.lines();
        let first_line = lines.next().unwrap();

        let instruction_pointer_index = (&first_line[4..]).parse::<u8>().unwrap();

        let mut instructions = Vec::new();
        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let opcode = opcode_from_str(parts[0]);
            let a = parts[1].parse::<u64>().unwrap();
            let b = parts[2].parse::<u64>().unwrap();
            let c = parts[3].parse::<u64>().unwrap();
            instructions.push((opcode, a, b, c));
        }

        Program {
            instruction_pointer_index,
            instructions,
            registers: Registers::new(),
        }
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

fn opcode_from_str(name: &str) -> Opcode {
    match name {
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
            panic!("No matching opcode: {}", name);
        }
    }
}

impl Registers {
    fn new() -> Registers {
        Registers {
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

pub fn part1(input_string: &str) -> String {
    Program::parse(input_string).execute().to_string()
}

pub fn part2(input_string: &str) -> String {
    let mut program = Program::parse(input_string);

    program.registers.values[0] = 1;

    program.pretty_print("Initial");

    program.optimize();
    program.pretty_print("Optimized");

    // Assuming som of all factors program starts at instruction 1:
    while program.registers.values[1] == 0 {
        program.execute_one_instruction();
    }

    let mut sum = 0;
    // Assuming number to be factored is highest register value:
    let &seed = program.registers.values.iter().max().unwrap();
    for i in 1..=seed {
        if seed % i == 0 {
            sum += i;
        }
    }
    sum.to_string()
}

#[test]
fn tests_part1() {
    assert_eq!(
        "7",
        part1(
            "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"
        )
    );

    assert_eq!("978", part1(include_str!("day19_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!("10996992", part2(include_str!("day19_input.txt")));
}
