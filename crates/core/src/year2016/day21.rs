use crate::common::permutation::all_permutations;
use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<String, String> {
    let mut password = [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'];
    if input.is_part_one() {
        scramble(input.text, &mut password)?;
        Ok(password.iter().map(|&b| b as char).collect::<String>())
    } else {
        // fbgdceah
        let desired = [b'f', b'b', b'g', b'd', b'c', b'e', b'a', b'h'];
        let mut answer = None;
        all_permutations(&mut password, &mut |permutation| {
            let mut copy = [0, 0, 0, 0, 0, 0, 0, 0];
            copy.copy_from_slice(permutation);
            scramble(input.text, &mut copy)?;
            if copy == desired {
                answer = Some(permutation.iter().map(|&b| b as char).collect::<String>());
            }
            Ok(())
        })?;

        answer.ok_or_else(|| "No solution found".to_string())
    }
}

fn scramble(input: &str, password: &mut [u8]) -> Result<(), String> {
    let error_mapper = |_| "Invalid input";
    for line in input.lines() {
        let words = line.split(' ').collect::<Vec<_>>();
        match words[0] {
            "swap" => {
                if words[1] == "position" {
                    let x = words[2].parse::<usize>().map_err(error_mapper)?;
                    let y = words[5].parse::<usize>().map_err(error_mapper)?;
                    password.swap(x, y);
                } else {
                    // Swap letters
                    let x = words[2].as_bytes()[0];
                    let y = words[5].as_bytes()[0];
                    password.iter_mut().for_each(|c| {
                        let orig = *c;
                        *c = if orig == x {
                            y
                        } else if orig == y {
                            x
                        } else {
                            orig
                        };
                    });
                }
            }
            "rotate" => {
                let rotation = if words[1] == "based" {
                    let letter = words[6].as_bytes()[0];
                    if let Some((idx, _)) =
                        password.iter().enumerate().find(|&(_idx, &c)| c == letter)
                    {
                        ((1 + idx + if idx >= 4 { 1 } else { 0 }) % password.len()) as i32
                    } else {
                        return Err(format!(
                            "Unable to find letter for rotation: '{}'",
                            letter as char
                        ));
                    }
                } else {
                    words[2].parse::<i32>().map_err(error_mapper)?
                        * if words[1] == "left" { -1 } else { 1 }
                };

                if rotation < 0 {
                    password.rotate_left((-rotation) as usize);
                } else {
                    password.rotate_right(rotation as usize);
                }
            }
            "reverse" => {
                let x = words[2].parse::<usize>().map_err(error_mapper)?;
                let y = words[4].parse::<usize>().map_err(error_mapper)?;
                password[x..(y + 1)].reverse();
            }
            "move" => {
                let x = words[2].parse::<usize>().map_err(error_mapper)?;
                let y = words[5].parse::<usize>().map_err(error_mapper)?;
                let mut buffer: Vec<u8> = password.to_vec();
                let removed_letter = buffer.remove(x);
                buffer.insert(y, removed_letter);
                password.clone_from_slice(&buffer);
            }
            _ => {
                return Err("Invalid input".to_string());
            }
        }
    }
    Ok(())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day21_input.txt");
    test_part_one!(real_input => "gcedfahb".to_string());
    test_part_two!(real_input => "hegbdcfa".to_string());
}
