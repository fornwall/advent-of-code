use crate::input::Input;
use std::collections::HashMap;

// https://www.reddit.com/r/adventofcode/comments/7k572l/2017_day_16_solutions/drbqb27/
fn parse(data: &str, programs: &[u8]) -> Result<(Vec<u8>, HashMap<u8, u8>), String> {
    let mut moves: Vec<u8> = (0..programs.len()).map(|u| u as u8).collect();
    let mut substitutions: HashMap<u8, u8> = programs.iter().map(|&c| (c, c)).collect();

    for dance_move in data.split(',') {
        if dance_move.is_empty() {
            return Err("Empty move".to_string());
        }
        let args = &mut dance_move[1..].split('/');

        if dance_move.starts_with('s') {
            // "Spin, written sX, makes X programs move from the end to the
            // front, but maintain their order otherwise. (For example, s3
            // on abcde produces cdeab)."
            let arg_1 = args
                .next()
                .ok_or("Spin not followed by argument")?
                .parse::<u8>()
                .map_err(|e| format!("Unable to parse Spin argument: {}", e))?;
            moves.rotate_right(arg_1 as usize);
        } else if dance_move.starts_with('x') {
            // "Exchange, written xA/B, makes the programs at positions A and B swap places."
            let arg_1 = args
                .next()
                .ok_or_else(|| "Exchange not followed by two arguments".to_string())?
                .parse::<u8>()
                .map_err(|e| format!("Unable to parse Exchange argument: {}", e))?;
            let arg_2 = args
                .next()
                .ok_or_else(|| "Exchange not followed by two arguments".to_string())?
                .parse::<u8>()
                .map_err(|e| format!("Unable to parse Exchange argument: {}", e))?;
            moves.swap(arg_1 as usize, arg_2 as usize);
        } else if dance_move.starts_with('p') {
            // "Partner, written pA/B, makes the programs named A and B swap places."
            let arg_1 = args
                .next()
                .ok_or_else(|| "Partner not followed by two arguments".to_string())?
                .bytes()
                .next()
                .ok_or_else(|| "Empty Partner argument".to_string())?;
            let arg_2 = args
                .next()
                .ok_or_else(|| "Partner not followed by two arguments".to_string())?
                .bytes()
                .next()
                .ok_or_else(|| "Empty Partner argument".to_string())?;
            for (_key, value) in substitutions.iter_mut() {
                if *value == arg_1 {
                    *value = arg_2;
                } else if *value == arg_2 {
                    *value = arg_1;
                }
            }
        } else {
            panic!();
        }
    }

    Ok((moves, substitutions))
}

pub fn solve(input: &mut Input) -> Result<String, String> {
    let mut programs = (b'a'..=b'p').collect::<Vec<u8>>();

    let mut rounds = input.part_values(1, 1_000_000_000);
    let (mut moves, mut substitutions) = parse(input.text, &programs)?;

    while rounds > 0 {
        if rounds % 2 == 1 {
            // Apply the current 2^n dance:
            programs = moves
                .iter()
                .map(|&i| substitutions.get(&programs[i as usize]).cloned())
                .collect::<Option<Vec<u8>>>()
                .ok_or_else(|| "Internal error".to_string())?;
        }

        // Double the number of dances each application will perform:
        moves = moves.iter().map(|&i| moves[i as usize]).collect();
        substitutions = substitutions
            .iter()
            .map(|(&key, value)| (key, substitutions[value]))
            .collect();

        rounds /= 2;
    }

    Ok(programs.iter().map(|b| *b as char).collect())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day16_input.txt");
    test_part_one!(real_input => "iabmedjhclofgknp".to_string());
    test_part_two!(real_input => "oildcmfeajhbpngk".to_string());
}
