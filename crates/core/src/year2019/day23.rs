use super::int_code::{Program, Word};
use crate::input::Input;
use std::collections::VecDeque;

pub fn solve(input: &mut Input) -> Result<Word, String> {
    let program = Program::parse(input.text)?;
    let mut programs = vec![program; 50];
    let mut input_queues = vec![VecDeque::<(Word, Word)>::new(); 50];

    // Assign network addresses:
    for (i, program) in programs.iter_mut().enumerate() {
        program.input(i as Word);
    }

    let mut last_packet_to_nat = (-1, -1);
    let mut last_emitted_packet_from_nat = (-1, -1);

    loop {
        for (program, input_queue) in programs.iter_mut().zip(input_queues.iter_mut()) {
            if input_queue.is_empty() {
                program.input(-1);
            } else {
                while let Some((x, y)) = input_queue.pop_front() {
                    program.input(x);
                    program.input(y);
                }
            }
        }

        let mut network_idle = true;
        for program in programs.iter_mut() {
            let output = program.run_for_output()?;
            for chunk in output.chunks_exact(3) {
                let (destination_address, packet) = (chunk[0], (chunk[1], chunk[2]));

                if destination_address == 255 {
                    if input.is_part_one() {
                        return Ok(packet.1);
                    } else {
                        last_packet_to_nat = packet;
                    }
                } else {
                    network_idle = false;
                    input_queues
                        .get_mut(destination_address as usize)
                        .ok_or("Destination address out of bonds")?
                        .push_back(packet);
                }
            }
        }

        if network_idle {
            if last_packet_to_nat.1 == last_emitted_packet_from_nat.1 {
                return Ok(last_packet_to_nat.1);
            }

            last_emitted_packet_from_nat = last_packet_to_nat;
            input_queues[0].push_back(last_packet_to_nat);
        }
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};
    let input = include_str!("day23_input.txt");
    test_part_one!(input => 16549);
    test_part_two!(input => 11462);
}
