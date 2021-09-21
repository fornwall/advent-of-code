use super::assembly::{NumberValue, Program};
use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<NumberValue, String> {
    let mut program_zero = Program::parse(input.text)?;
    if input.is_part_one() {
        program_zero.run_until_recover(None);
        Ok(program_zero.last_played_frequency)
    } else {
        let mut program_one = program_zero.clone();

        // "Each program also has its own program ID (one 0 and the other 1);
        // the register p should begin with this value."
        program_zero.registers[(b'p' - b'a') as usize] = 0;
        program_one.registers[(b'p' - b'a') as usize] = 1;

        loop {
            if !program_zero.terminated {
                program_zero.run_until_recover(Some(&mut program_one.input_queue));
            }
            if !program_one.terminated {
                program_one.run_until_recover(Some(&mut program_zero.input_queue));
            }

            if (program_zero.terminated && program_one.terminated)
                || (program_zero.input_queue.is_empty() && program_one.input_queue.is_empty())
            {
                // Both programs terminated, or both programs deadlocking.
                break;
            }
        }

        Ok(program_one.sent_value_count)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example_input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
    test_part_two!(example_input => 3);

    let real_input = include_str!("day18_input.txt");
    test_part_one!(real_input => 3423);
    test_part_two!(real_input => 7493);
}
