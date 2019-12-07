use crate::int_code::Program;

/// Generate all permutations of a sequence using Heap's algorithm.
fn all_permutations<F>(sequence: &mut Vec<i64>, size: usize, n: usize, on_permutation: &mut F)
where
    F: FnMut(&Vec<i64>),
{
    if size == 1 {
        on_permutation(sequence);
        return;
    }

    for i in 0..size {
        all_permutations(sequence, size - 1, n, on_permutation);

        if size % 2 == 1 {
            // If size is odd, swap first and last element.
            sequence.swap(0, size - 1);
        } else {
            // If size is even, swap ith and last element.
            sequence.swap(i, size - 1);
        }
    }
}

pub fn part1(input_string: &str) -> String {
    let program = Program::parse(input_string);
    let mut phase_settings = vec![0, 1, 2, 3, 4];
    let mut strongest_signal = 0;

    let size = phase_settings.len();
    all_permutations(&mut phase_settings, size, size, &mut |permutation: &Vec<
        i64,
    >| {
        let mut signal = 0;
        for &phase in permutation.iter() {
            let mut amplifier_program = program.clone();

            amplifier_program.input_values.push_back(phase);
            amplifier_program.input_values.push_back(signal);
            amplifier_program.run();

            signal = amplifier_program.last_output;
        }

        strongest_signal = std::cmp::max(strongest_signal, signal);
    });

    strongest_signal.to_string()
}

pub fn part2(_input_string: &str) -> String {
    String::from("")
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
    assert_eq!(part2(""), "");

    // assert_eq!(part2(include_str!("day07_input.txt")), "");
}
