use crate::int_code::Program;
use std::collections::VecDeque;

pub fn part1(input_string: &str) -> String {
    const SIZE: i32 = 50;
    let program = Program::parse(input_string);

    let mut programs = Vec::new();
    let mut input_queues = Vec::new();

    for i in 0..SIZE {
        let mut new_program = program.clone();
        // Assign network address:
        new_program.input(i as i64);
        programs.push(new_program);

        let new_queue: VecDeque<(i64, i64)> = VecDeque::new();
        input_queues.push(new_queue);
    }

    loop {
        for i in 0..SIZE {
            let program = &mut programs[i as usize];
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

        for i in 0..SIZE {
            let program = &mut programs[i as usize];
            program.run();

            for j in (0..program.output_values.len()).step_by(3) {
                let destination_address = program.output_values[j];
                let x = program.output_values[j + 1];
                let y = program.output_values[j + 2];

                if destination_address == 255 {
                    return y.to_string();
                }

                let destination_queue = &mut input_queues[destination_address as usize];
                destination_queue.push_back((x, y));
            }
            program.output_values.clear();
        }
    }
}

pub fn part2(_input_string: &str) -> String {
    String::from("")
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day23_input.txt")), "16549");
}

#[test]
fn tests_part2() {
    //assert_eq!(part2(include_str!("day23_input.txt")), "");
}
