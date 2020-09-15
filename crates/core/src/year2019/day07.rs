use super::int_code::Program;
use super::permutation::all_permutations;
use std::cell::RefCell;

pub fn part1(input_string: &str) -> Result<String, String> {
    let program = Program::parse(input_string)?;
    let mut phase_settings = vec![0, 1, 2, 3, 4];
    let mut strongest_signal = 0;

    all_permutations(&mut phase_settings, &mut |permutation: &Vec<i64>| {
        let mut signal = 0;
        for &phase in permutation.iter() {
            let mut amplifier_program = program.clone();

            amplifier_program.input(phase);
            amplifier_program.input(signal);

            signal = *amplifier_program.run_for_output().last().unwrap();
        }

        strongest_signal = std::cmp::max(strongest_signal, signal);
    });

    Ok(strongest_signal.to_string())
}

pub fn part2(input_string: &str) -> Result<String, String> {
    let program = Program::parse(input_string)?;
    let mut phase_settings = vec![5, 6, 7, 8, 9];
    let mut strongest_signal = 0;

    all_permutations(&mut phase_settings, &mut |permutation: &Vec<i64>| {
        let mut amplifier_programs = Vec::new();
        for &phase in permutation.iter() {
            let mut new_program = program.clone();
            new_program.input(phase);
            amplifier_programs.push(RefCell::new(new_program));
        }

        amplifier_programs[0].borrow_mut().input(0);

        let mut last_signal_output = 0;
        'outer: loop {
            for i in 0..5 {
                let mut current_program = amplifier_programs[i].borrow_mut();
                let output = current_program.run_for_output();

                if i == 4 {
                    if let Some(&value) = output.last() {
                        last_signal_output = value;
                    }
                    if current_program.is_halted() {
                        break 'outer;
                    }
                }

                let mut next_program = amplifier_programs[(i + 1) % 5].borrow_mut();
                for &value in output.iter() {
                    next_program.input(value);
                }
            }
        }

        strongest_signal = std::cmp::max(strongest_signal, last_signal_output);
    });

    Ok(strongest_signal.to_string())
}

#[test]
pub fn tests_part1() {
    assert_eq!(
        part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
        Ok("43210".to_string())
    );
    assert_eq!(
        part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
        Ok("54321".to_string())
    );
    assert_eq!(part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), Ok("65210".to_string()));

    assert_eq!(
        part1(include_str!("day07_input.txt")),
        Ok("51679".to_string())
    );
}

#[test]
fn tests_part2() {
    assert_eq!(
        part2(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
        ),
        Ok("139629729".to_string())
    );
    assert_eq!(part2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), Ok("18216".to_string()));

    assert_eq!(
        part2(include_str!("day07_input.txt")),
        Ok("19539216".to_string())
    );
}
