use crate::int_code::Program;
use std::collections::VecDeque;

pub fn run_simulation(input_string: &str, part1: bool) -> String {
    let mut programs = vec![Program::parse(input_string); 50];
    let mut input_queues = vec![VecDeque::<(i64, i64)>::new(); 50];

    // Assign network addresses:
    for (i, program) in programs.iter_mut().enumerate() {
        program.input(i as i64);
    }

    let mut last_packet_to_nat = (-1, -1);
    let mut last_emitted_packet_from_nat = (-1, -1);

    loop {
        for (i, program) in programs.iter_mut().enumerate() {
            let input_queue = &mut input_queues[i as usize];
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
            program.run();

            for j in (0..program.output_values.len()).step_by(3) {
                network_idle = false;

                let destination_address = program.output_values[j];
                let packet = (program.output_values[j + 1], program.output_values[j + 2]);

                if destination_address == 255 {
                    if part1 {
                        return packet.1.to_string();
                    } else {
                        last_packet_to_nat = packet;
                        continue;
                    }
                }

                input_queues[destination_address as usize].push_back(packet);
            }
            program.output_values.clear();
        }

        if network_idle {
            if last_packet_to_nat.1 == last_emitted_packet_from_nat.1 {
                return last_packet_to_nat.1.to_string();
            }

            last_emitted_packet_from_nat = last_packet_to_nat;
            input_queues[0].push_back(last_packet_to_nat);
        }
    }
}

pub fn part1(input_string: &str) -> String {
    run_simulation(input_string, true)
}

pub fn part2(input_string: &str) -> String {
    run_simulation(input_string, false)
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day23_input.txt")), "16549");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day23_input.txt")), "11462");
}
