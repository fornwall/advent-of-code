use super::int_code::{Program, Word};
use std::collections::HashMap;
use std::env;

pub fn part1(input_string: &str) -> Result<usize, String> {
    let mut program = Program::parse(input_string)?;
    let output = program.run_for_output()?;
    Ok(output
        .iter()
        .skip(2)
        .step_by(3)
        .filter(|&&t| t == 2)
        .count())
}

fn render(current_score: Word, tiles: &HashMap<(Word, Word), Word>) {
    let mut output = String::new();
    output.push_str("\x1b[2J\x1b[H");
    let mut min_x = Word::MAX;
    let mut max_x = Word::MIN;
    let mut min_y = Word::MAX;
    let mut max_y = Word::MIN;
    for &(x, y) in tiles.keys() {
        min_x = std::cmp::min(min_x, x);
        max_x = std::cmp::max(max_x, x);
        min_y = std::cmp::min(min_y, y);
        max_y = std::cmp::max(max_y, y);
    }

    output.push_str(format!("Score: {}\n", current_score).as_str());
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let character = match tiles.get(&(x, y)) {
                Some(1) => '█',
                Some(2) => '▬',
                Some(3) => '▢',
                Some(4) => '○',
                _ => ' ',
            };
            output.push(character);
        }
        output.push('\n');
    }
    println!("{}", output);
    std::thread::sleep(std::time::Duration::from_millis(10));
}

pub fn part2(input_string: &str) -> Result<Word, String> {
    let mut program = Program::parse(input_string)?;

    // "Memory address 0 represents the number of quarters that
    // have been inserted; set it to 2 to play for free."
    program.write_memory(0, 2);

    let mut tiles = HashMap::new();
    let mut current_score = 0;
    let mut ball_x = -1;
    let mut paddle_x = -1;
    let debug = env::var("ADVENT_DEBUG").is_ok();

    loop {
        let output = program.run_for_output()?;
        output.chunks_exact(3).for_each(|chunk| {
            let x = chunk[0];
            let y = chunk[1];
            let third = chunk[2];
            if x == -1 && y == 0 {
                current_score = third;
            } else {
                if debug {
                    tiles.insert((x, y), third);
                }
                if third == 3 {
                    paddle_x = x;
                } else if third == 4 {
                    ball_x = x;
                };
            }
        });

        if debug {
            render(current_score, &tiles);
        }

        if program.is_halted() {
            break;
        }

        program.input(match ball_x {
            _ if ball_x > paddle_x => 1,
            _ if ball_x < paddle_x => -1,
            _ => 0,
        });
    }

    Ok(current_score)
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day13_input.txt")), Ok(462));
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day13_input.txt")), Ok(23981));
}
