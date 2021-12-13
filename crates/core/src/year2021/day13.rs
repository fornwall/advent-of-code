use crate::common::character_recognition::recognize;
use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<String, String> {
    let mut dots = Vec::new();

    let parse_number = |num_str: &str| {
        num_str
            .parse::<u16>()
            .map_err(|_| "Invalid number - not an u16")
    };

    for line in input.text.lines() {
        if let Some((x, y)) = line.split_once(',') {
            let x = parse_number(x)?;
            let y = parse_number(y)?;
            dots.push((x, y));
        } else if let Some((prefix, coord)) = line.split_once('=') {
            let coord = parse_number(coord)?;
            if prefix.ends_with('x') {
                for p in dots.iter_mut() {
                    if p.0 > coord {
                        p.0 = 2 * coord - p.0;
                    }
                }
            } else if prefix.ends_with('y') {
                for p in dots.iter_mut() {
                    if p.1 > coord {
                        p.1 = 2 * coord - p.1;
                    }
                }
            } else {
                return Err("Invalid line not ending width x=.. or y=..".to_string());
            }
            if input.is_part_one() {
                dots.sort_unstable();
                dots.dedup();
                return Ok(dots.len().to_string());
            }
        }
    }

    let mut code = String::new();
    for letter in 0..8 {
        let mut char_string = String::new();
        for y in 0..6 {
            for x in 0..5 {
                let x_pos = letter * 5 + x;
                char_string.push(if dots.contains(&(x_pos, y)) {
                    '█'
                } else {
                    ' '
                });
            }
            if y != 5 {
                char_string.push('\n');
            }
        }
        let c = recognize(&char_string)?;
        code.push(c);
    }

    Ok(code)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    test_part_one!(example => "17".to_string());

    let real_input = include_str!("day13_input.txt");
    test_part_one!(real_input => "763".to_string());
    test_part_two!(real_input => "RHALRCRA".to_string());
}
