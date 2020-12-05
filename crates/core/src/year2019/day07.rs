use super::int_code::{Program, Word};
use super::permutation::all_permutations;
use crate::input::Input;
use std::cell::RefCell;

pub fn solve(input: &mut Input) -> Result<i64, String> {
    let program = Program::parse(input.text)?;
    let mut phase_settings = if input.is_part_one() {
        [0, 1, 2, 3, 4]
    } else {
        [5, 6, 7, 8, 9]
    };
    let mut strongest_signal = 0;

    all_permutations(&mut phase_settings, &mut |permutation: &[Word]| {
        let mut amplifier_programs = Vec::with_capacity(5);
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
                let output = current_program.run_for_output_limited(10_000)?;

                if i == 4 {
                    if let Some(&value) = output.last() {
                        last_signal_output = value;
                    }
                    if input.is_part_one() || current_program.is_halted() {
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
        Ok(())
    })?;

    Ok(strongest_signal)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0" => 43210);
    test_part_one!("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"=>54321);
    test_part_one!("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0" => 65210);

    test_part_two!("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5" => 139629729);
    test_part_two!("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10" => 18216);

    let real_input = include_str!("day07_input.txt");
    test_part_one!(real_input => 51679);
    test_part_two!(real_input => 19539216);
}
