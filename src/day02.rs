struct Program {
    values: Vec<u64>,
    instruction_pointer: usize,
}

impl Program {
    fn parse(input: &str) -> Program {
        Program {
            values: input
                .split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect(),
            instruction_pointer: 0,
        }
    }

    fn patch(&mut self, index: usize, value: u64) {
        self.values[index] = value;
    }

    fn run(&mut self) -> u64 {
        loop {
            if !self.evaluate() {
                break self.values[0];
            }
        }
    }

    fn evaluate(&mut self) -> bool {
        let opcode = self.values[self.instruction_pointer];
        match opcode {
            99 => false,
            1 | 2 => {
                let parameter1_address = self.values[self.instruction_pointer + 1] as usize;
                let parameter2_location = self.values[self.instruction_pointer + 2] as usize;
                let output_location = self.values[self.instruction_pointer + 3] as usize;
                let parameter1 = self.values[parameter1_address];
                let parameter2 = self.values[parameter2_location];
                self.values[output_location] = if opcode == 1 {
                    parameter1 + parameter2
                } else {
                    parameter1 * parameter2
                };
                self.instruction_pointer += 4;
                true
            }
            _ => {
                panic!("Invalid opcode: {}", opcode);
            }
        }
    }
}

pub fn part1(input_string: &str) -> String {
    part1_patch(input_string, true)
}

pub fn part1_patch(input_string: &str, patch: bool) -> String {
    let mut program = Program::parse(input_string);

    if patch {
        // To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2.
        program.patch(1, 12);
        program.patch(2, 2);
    }

    program.run().to_string()
}

pub fn part2(input_string: &str) -> String {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = Program::parse(input_string);
            program.patch(1, noun);
            program.patch(2, verb);
            if program.run() == 19_690_720 {
                return (100 * noun + verb).to_string();
            }
        }
    }
    "ERROR".to_string()
}

#[test]
pub fn tests_part1() {
    assert_eq!("3500", part1_patch("1,9,10,3,2,3,11,0,99,30,40,50", false));
    assert_eq!("2", part1_patch("1,0,0,0,99", false));
    assert_eq!("2", part1_patch("2,3,0,3,99", false));
    assert_eq!("2", part1_patch("2,4,4,5,99,0", false));
    assert_eq!("30", part1_patch("1,1,1,4,99,5,6,0,99", false));

    assert_eq!("4570637", part1(include_str!("day2_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!("5485", part2(include_str!("day2_input.txt")));
}
