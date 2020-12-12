use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<i32, String> {
    let mut position = (0, 0);
    let mut direction = (1, 0);
    let rotation_matrices: [(i32, i32, i32, i32); 3] = [
        (0, -1, 1, 0),  //  R90 / L270
        (-1, 0, 0, -1), // R180 / L180
        (0, 1, -1, 0),  // R270 /  L90
    ];

    for (line_idx, line) in input.text.lines().enumerate() {
        let on_error = || {
            format!("Line {}: Invalid input - expected N|S|E|W followd by u32, or R|L followed by 90|180|270", line_idx + 1)
        };

        if line.len() < 2 {
            return Err(on_error());
        }

        let numeric_parameter = line[1..].parse::<i32>().map_err(|_| on_error())?;

        let operation = line.as_bytes()[0];
        match operation {
            b'N' => {
                position.1 -= numeric_parameter;
            }
            b'S' => {
                position.1 += numeric_parameter;
            }
            b'E' => {
                position.0 += numeric_parameter;
            }
            b'W' => {
                position.0 -= numeric_parameter;
            }
            b'R' | b'L' => {
                if !matches!(numeric_parameter, 90 | 180 | 270) {
                    return Err(on_error());
                }

                let rotation_idx = (if operation == b'L' {
                    360 - numeric_parameter
                } else {
                    numeric_parameter
                } / 90)
                    - 1;

                let rotation = rotation_matrices[rotation_idx as usize];

                direction = (
                    direction.0 * rotation.0 + direction.1 * rotation.1,
                    direction.0 * rotation.2 + direction.1 * rotation.3,
                );
            }
            b'F' => {
                position = (
                    position.0 + numeric_parameter * direction.0,
                    position.1 + numeric_parameter * direction.1,
                );
            }
            _ => {
                return Err(on_error());
            }
        }
    }

    Ok(position.0.abs() + position.1.abs())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!("F10\nN3\nF7\nR90\nF11" => 25);
    //test_part_two!("" => 0);

    let real_input = include_str!("day12_input.txt");
    test_part_one!(real_input => 0);
    //test_part_two!(real_input => 0);
}
