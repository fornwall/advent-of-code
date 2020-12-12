use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<i32, String> {
    const SHIP_POSITION_ENTITY_IDX: usize = 0;
    const SHIP_DIRECTION_ENTITY_IDX: usize = 1;
    const WAYPOINT_ENTITY_IDX: usize = 2;

    let ship_position = (0, 0);
    let ship_direction = (1, 0);
    let waypoint = (10, -1);

    let mut entities = [ship_position, ship_direction, waypoint];
    let moved_entity_idx = input.part_values(SHIP_POSITION_ENTITY_IDX, WAYPOINT_ENTITY_IDX);
    let rotated_entity_idx = input.part_values(SHIP_DIRECTION_ENTITY_IDX, WAYPOINT_ENTITY_IDX);

    let rotation_matrices: [(i32, i32, i32, i32); 3] = [
        (0, -1, 1, 0),  //  R90 / L270
        (-1, 0, 0, -1), // R180 / L180
        (0, 1, -1, 0),  // R270 /  L90
    ];

    for (line_idx, line) in input.text.lines().enumerate() {
        let on_error = || {
            format!("Line {}: Invalid input - expected N|S|E|W followd by i32, or R|L followed by 90|180|270", line_idx + 1)
        };

        if line.len() < 2 {
            return Err(on_error());
        }

        let numeric_parameter = line[1..].parse::<i32>().map_err(|_| on_error())?;

        let operation = line.as_bytes()[0];
        match operation {
            b'N' => {
                entities[moved_entity_idx].1 -= numeric_parameter;
            }
            b'S' => {
                entities[moved_entity_idx].1 += numeric_parameter;
            }
            b'E' => {
                entities[moved_entity_idx].0 += numeric_parameter;
            }
            b'W' => {
                entities[moved_entity_idx].0 -= numeric_parameter;
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
                let rotated = entities[rotated_entity_idx];
                entities[rotated_entity_idx] = (
                    rotated.0 * rotation.0 + rotated.1 * rotation.1,
                    rotated.0 * rotation.2 + rotated.1 * rotation.3,
                );
            }
            b'F' => {
                let direction = entities[rotated_entity_idx];
                let position = entities[SHIP_POSITION_ENTITY_IDX];
                entities[SHIP_POSITION_ENTITY_IDX] = (
                    position.0 + numeric_parameter * direction.0,
                    position.1 + numeric_parameter * direction.1,
                );
            }
            _ => {
                return Err(on_error());
            }
        }
    }

    let position = entities[SHIP_POSITION_ENTITY_IDX];
    Ok(position.0.abs() + position.1.abs())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "F10\nN3\nF7\nR90\nF11";
    test_part_one!(example => 25);
    test_part_two!(example => 286);

    let real_input = include_str!("day12_input.txt");
    test_part_one!(real_input => 1007);
    test_part_two!(real_input => 41212);
}
