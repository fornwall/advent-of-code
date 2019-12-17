use crate::int_code::Program;
use std::slice::Iter;

pub fn part1(input_string: &str) -> String {
    let mut program = Program::parse(input_string);
    program.run();
    let map: String = program
        .output_values
        .iter()
        .map(|&b| (b as u8) as char)
        .collect();
    part1_map(&map)
}

fn part1_map(map: &str) -> String {
    let map: Vec<&[u8]> = map.lines().map(|line| line.as_bytes()).collect();

    let mut alignment_parameters_sum = 0;
    for y in 1..(map.len() - 1) {
        for x in 1..(map[0].len() - 1) {
            if map[y][x] == b'#'
                && map[y][x - 1] == b'#'
                && map[y][x + 1] == b'#'
                && map[y - 1][x] == b'#'
                && map[y + 1][x] == b'#'
            {
                alignment_parameters_sum += x * y;
            }
        }
    }

    alignment_parameters_sum.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn other(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    fn advance(self, position: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Right => (position.0 + 1, position.1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
        }
    }

    fn instruction_for_turning_to(self, target: Direction) -> char {
        if self.turn_right() == target {
            'R'
        } else if self.turn_left() == target {
            'L'
        } else {
            panic!("From {:?} to {:?}", self, target);
        }
    }

    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        DIRECTIONS.iter()
    }
}

pub fn part2(input_string: &str) -> String {
    let mut program = Program::parse(input_string);

    program.write_memory(0, 2);
    program.run();

    let map: String = program
        .output_values
        .iter()
        .map(|&b| (b as u8) as char)
        .collect();
    let map: Vec<&[u8]> = map.lines().map(|line| line.as_bytes()).collect();
    // Strip away last two lines with blank line and "Main:" prompt:
    let map = &map[0..(map.len() - 2)];

    let mut robot_direction = Direction::Up;
    let mut robot_position: (i32, i32) = (0, 0);
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == b'^' {
                robot_direction = Direction::Up;
                robot_position = (x as i32, y as i32);
            } else if map[y][x] == b'v' {
                robot_direction = Direction::Down;
                robot_position = (x as i32, y as i32);
            } else if map[y][x] == b'<' {
                robot_direction = Direction::Left;
                robot_position = (x as i32, y as i32);
            } else if map[y][x] == b'>' {
                robot_direction = Direction::Right;
                robot_position = (x as i32, y as i32);
            }
        }
    }

    let mut starting = true;
    let mut moves_since_turn = 0;
    let mut movements = String::new();

    loop {
        let continuing_position = robot_direction.advance(robot_position);
        if continuing_position.0 >= 0
            && continuing_position.0 < map[0].len() as i32
            && continuing_position.1 >= 0
            && continuing_position.1 < map.len() as i32
            && map[continuing_position.1 as usize][continuing_position.0 as usize] == b'#'
        {
            robot_position = continuing_position;
            moves_since_turn += 1;
            continue;
        }

        let mut possible_directions = Vec::new();
        for &direction in Direction::iterator() {
            let new_location = direction.advance(robot_position);
            if new_location.0 >= 0
                && new_location.0 < map[0].len() as i32
                && new_location.1 >= 0
                && new_location.1 < map.len() as i32
                && map[new_location.1 as usize][new_location.0 as usize] == b'#'
            {
                possible_directions.push(direction);
            }
        }

        if possible_directions.len() == 1 {
            if starting {
                starting = false;
                movements.push(robot_direction.instruction_for_turning_to(possible_directions[0]));
                robot_direction = possible_directions[0];
                moves_since_turn = 0;
            } else {
                // Done.
                if moves_since_turn > 0 {
                    movements.push(',');
                    movements.push_str(&moves_since_turn.to_string());
                }
                break;
            }
        } else if possible_directions.len() == 2 {
            let new_direction = if possible_directions[0] == robot_direction.other() {
                possible_directions[1]
            } else {
                possible_directions[0]
            };

            if new_direction == robot_direction {
                robot_position = robot_direction.advance(robot_position);
                moves_since_turn += 1;
            } else {
                if moves_since_turn > 0 {
                    movements.push(',');
                    movements.push_str(&moves_since_turn.to_string());
                    moves_since_turn = 0;
                }
                movements.push(',');
                movements.push(robot_direction.instruction_for_turning_to(new_direction));
                robot_direction = new_direction;
            }
        } else if possible_directions.len() == 4 {
            robot_position = robot_direction.advance(robot_position);
            moves_since_turn += 1;
        } else {
            panic!("Possible directions={}", possible_directions.len());
        }
    }

    movements.push(',');
    for length_of_a in 1..20 {
        let substring_a = &movements[0..length_of_a];
        let remaining_string = movements.replace(substring_a, "");
        for length_of_b in 1..20 {
            let substring_b = &remaining_string[0..length_of_b];
            let remaining_string = remaining_string.replace(substring_b, "");
            for length_of_c in 1..20 {
                let substring_c = &remaining_string[0..length_of_c];
                let remaining_string = remaining_string.replace(substring_c, "");
                if remaining_string.is_empty() {
                    let function_a = &substring_a[0..substring_a.len() - 1];
                    let function_b = &substring_b[0..substring_b.len() - 1];
                    let function_c = &substring_c[0..substring_c.len() - 1];
                    let main_routine = movements
                        .replace(function_a, "A")
                        .replace(function_b, "B")
                        .replace(function_c, "C");
                    let main_routine = &main_routine[0..main_routine.len() - 1];

                    for &mut input in
                        vec![main_routine, function_a, function_b, function_c, "n"].iter_mut()
                    {
                        program.input_string(input);
                        program.input_string("\n");
                        program.output_values.clear();
                        program.run();
                    }
                    let result = program
                        .output_values
                        .iter()
                        .find(|&&value| value > 255)
                        .unwrap();
                    return result.to_string();
                }
            }
        }
    }

    String::from("")
}

#[test]
pub fn tests_part1() {
    assert_eq!(
        part1_map(
            "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..",
        ),
        "76"
    );

    assert_eq!(part1(include_str!("day17_input.txt")), "11140");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day17_input.txt")), "1113108");
}
