use crate::input::Input;
use std::collections::HashMap;

/*
pub fn solve(input: &mut Input) -> Result<String, String> {
    let mut programs = vec![b'a'; 16];
    for (idx, char) in programs.iter_mut().enumerate() {
        *char = b'a' + idx as u8;
    }

    for dance_move in input.text.split(',') {
        match dance_move.chars().next() {
            Some('s') => {
                // Spin, sX, makes X programs move from end to the front.
                let &num_programs = &dance_move[1..].parse::<usize>().unwrap();
                programs.rotate_right(num_programs as usize);
            }
            Some('x') => {
                // Exchange, written xA/B, makes the programs at positions A and B swap places.
                let parts = &mut dance_move[1..].split('/');
                let a = parts.next().unwrap().parse::<usize>().unwrap();
                let b = parts.next().unwrap().parse::<usize>().unwrap();
                programs.swap(a as usize, b as usize);
            }
            Some('p') => {
                // Partner, written pA/B, makes the programs named A and B swap places.
                if let Some(a) = dance_move.chars().nth(1) {
                    if let Some(b) = dance_move.chars().nth(3) {
                        let pos_a = programs.iter().position(|p| *p == a as u8).unwrap();
                        let pos_b = programs.iter().position(|p| *p == b as u8).unwrap();
                        programs.swap(pos_a, pos_b);
                    }
                }
            }
            _ => {
                return Err("Invalid line not starting with 's', 'x' or 'p'".to_string());
            }
        }
    }

    Ok(programs.iter().map(|b| *b as char).collect())
}
 */

// https://www.reddit.com/r/adventofcode/comments/7k572l/2017_day_16_solutions/drbqb27/
fn compile(data: &str, programs: &[u8]) -> (Vec<u8>, HashMap<u8, u8>) {
    #![allow(clippy::unwrap_used)]
    let mut moves: Vec<u8> = (0..programs.len()).map(|u| u as u8).collect();
    let mut substitutions: HashMap<u8, u8> = programs.iter().map(|&c| (c, c)).collect();

    for dance_move in data.split(',') {
        let args = &mut dance_move[1..].split('/');
        let arg_1 = args.next().unwrap().parse::<u8>().unwrap();

        if dance_move.starts_with('s') {
            moves.rotate_right(arg_1 as usize);
        } else if dance_move.starts_with('x') {
            let arg_2 = args.next().unwrap().parse::<u8>().unwrap();
            moves.swap(arg_1 as usize, arg_2 as usize);
        } else if dance_move.starts_with('p') {
            let arg_2 = args.next().unwrap().parse::<u8>().unwrap();
            for (_key, value) in substitutions.iter_mut() {
                if *value == arg_1 {
                    *value = arg_2;
                } else if *value == arg_2 {
                    *value = arg_1;
                }
            }
        }
    }

    (moves, substitutions)
}

pub fn solve(input: &mut Input) -> Result<String, String> {
    let mut programs = vec![b'a'; 16];
    let mut rounds = input.part_values(1, 1_000_000_000);
    let (mut moves, substitutions) = compile(&input.text, &programs);

    while rounds > 0 {
        if (rounds & 1) == 1 {
            // Apply the current 2 * *n dance:
            for (idx, &dance_move) in moves.iter().enumerate() {
                programs[idx] = *substitutions.get(&programs[dance_move as usize]).unwrap();
            }
        }

        // Double the number of dances each application will perform:
        for _dance_move in moves.iter_mut() {
            // *dance_move = moves[*dance_move as usize];
        }
        // TODO: s = s.transform_values { |v| s[v] }

        rounds >>= 1
    }

    Ok(programs.iter().map(|b| *b as char).collect())
}

#[test]
#[ignore]
pub fn tests() {
    use crate::test_part_one;

    let real_input = include_str!("day16_input.txt");
    test_part_one!(real_input => "iabmedjhclofgknp".to_string());
}
