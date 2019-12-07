use crate::int_code::Program;
use crate::permutation::all_permutations;

pub fn part1(input_string: &str) -> String {
    let program = Program::parse(input_string);
    let mut phase_settings = vec![0, 1, 2, 3, 4];
    let mut strongest_signal = 0;

    all_permutations(&mut phase_settings, &mut |permutation: &Vec<i64>| {
        let mut signal = 0;
        for &phase in permutation.iter() {
            let mut amplifier_program = program.clone();

            amplifier_program.input(phase);
            amplifier_program.input(signal);
            amplifier_program.run();

            signal = *amplifier_program.output_values.last().unwrap();
        }

        strongest_signal = std::cmp::max(strongest_signal, signal);
    });

    strongest_signal.to_string()
}

pub fn part2(input_string: &str) -> String {
    let program = Program::parse(input_string);
    let mut phase_settings = vec![5, 6, 7, 8, 9];
    let mut strongest_signal = 0;

    all_permutations(&mut phase_settings, &mut |permutation: &Vec<i64>| {
        let mut amplifier_programs = Vec::new();
        for &phase in permutation.iter() {
            let mut new_program = program.clone();
            new_program.input(phase);
            amplifier_programs.push(new_program);
        }

        amplifier_programs[0].input(0);

        let mut last_signal_output = 0;
        'outer: loop {
            for i in 0..5 {
                let current_program = &mut amplifier_programs[i];
                current_program.run();

                if i == 4 {
                    if !current_program.output_values.is_empty() {
                        last_signal_output = *current_program.output_values.last().unwrap();
                    }
                    if current_program.is_halted() {
                        break 'outer;
                    }
                }

                let current_output = current_program.output_values.clone(); // XXX: Avoid clone()?
                current_program.output_values.clear();

                let next_program = &mut amplifier_programs[(i + 1) % 5];
                for &value in current_output.iter() {
                    next_program.input(value);
                }
            }
        }

        strongest_signal = std::cmp::max(strongest_signal, last_signal_output);
    });

    strongest_signal.to_string()
}

#[test]
pub fn tests_part1() {
    assert_eq!(
        part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
        "43210"
    );
    assert_eq!(
        part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
        "54321"
    );
    assert_eq!(part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), "65210");

    assert_eq!(part1(include_str!("day07_input.txt")), "51679");
}

#[test]
fn tests_part2() {
    assert_eq!(
        part2(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
        ),
        "139629729"
    );
    assert_eq!(part2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), "18216");

    assert_eq!(part2(include_str!("day07_input.txt")), "19539216");
}
