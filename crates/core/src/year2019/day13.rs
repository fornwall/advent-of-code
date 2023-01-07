use super::int_code::{Program, Word};
use crate::input::Input;

pub fn solve(input: &Input) -> Result<Word, String> {
    let mut program = Program::parse(input.text)?;

    let is_part_one = input.is_part_one();

    // "Memory address 0 represents the number of quarters that have been
    // inserted; set it to 2 to play for free."
    if !is_part_one {
        program.write_memory(0, 2);
    }

    let mut current_score = 0;
    let mut ball_x = -1;
    let mut paddle_x = -1;

    loop {
        let output = program.run_for_output()?;
        output.chunks_exact(3).for_each(|chunk| {
            let (x, y, third) = (chunk[0], chunk[1], chunk[2]);
            if x == -1 && y == 0 {
                current_score = third;
            } else {
                match third {
                    3 => paddle_x = x,
                    4 => ball_x = x,
                    _ => {}
                }
            }
        });

        if is_part_one {
            return Ok(output
                .iter()
                .skip(2)
                .step_by(3)
                .filter(|&&t| t == 2)
                .count() as Word);
        }

        if program.is_halted() {
            break;
        }

        program.input(ball_x.cmp(&paddle_x) as Word);
    }

    Ok(current_score)
}

#[test]
pub fn tests_part1() {
    assert_eq!(
        solve(&Input::part_one(include_str!("day13_input.txt"))),
        Ok(462)
    );
}

#[test]
fn tests_part2() {
    assert_eq!(
        solve(&Input::part_two(include_str!("day13_input.txt"))),
        Ok(23981)
    );
}
