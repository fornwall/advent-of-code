use super::int_code::Program;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    Black = 0,
    White = 1,
}

impl Color {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Black,
            1 => Self::White,
            _ => panic!("Invalid value: {}", value),
        }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    const fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }
}

fn run(input_string: &str, initial_color: Color) -> Result<HashMap<(i32, i32), Color>, String> {
    let mut program = Program::parse(input_string)?;
    let mut painted: HashMap<(i32, i32), Color> = HashMap::new();
    let mut position = (0, 0);
    let mut current_direction = Direction::Up;

    if initial_color == Color::White {
        painted.insert(position, initial_color);
    }

    loop {
        program.input(*painted.get(&position).unwrap_or(&Color::Black) as i64);
        let output = program.run_for_output();

        if program.is_halted() {
            break;
        }

        let painted_color = Color::from(output[0]);
        let turn_direction = output[1];

        painted.insert(position, painted_color);

        current_direction = match turn_direction {
            0 => current_direction.turn_left(),
            1 => current_direction.turn_right(),
            _ => panic!("Invalid direction: {}", turn_direction),
        };

        match current_direction {
            Direction::Up => position = (position.0, position.1 + 1),
            Direction::Right => position = (position.0 + 1, position.1),
            Direction::Down => position = (position.0, position.1 - 1),
            Direction::Left => position = (position.0 - 1, position.1),
        }
    }

    Ok(painted)
}

pub fn part1(input_string: &str) -> Result<String, String> {
    let map = run(input_string, Color::Black)?;
    Ok(map.len().to_string())
}

pub fn part2(input_string: &str) -> Result<String, String> {
    let painted = run(input_string, Color::White)?;
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;
    painted.keys().for_each(|&(x, y)| {
        min_x = std::cmp::min(min_x, x);
        max_x = std::cmp::max(max_x, x);
        min_y = std::cmp::min(min_y, y);
        max_y = std::cmp::max(max_y, y);
    });

    let mut result = String::new();
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            result.push(if let Some(&Color::White) = painted.get(&(x, y)) {
                '█'
            } else {
                ' '
            });
        }
        if y != min_y {
            result.push('\n');
        }
    }

    Ok(result)
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day11_input.txt")).unwrap(), "1686");
}

#[test]
fn tests_part2() {
    assert_eq!(
        part2(include_str!("day11_input.txt")).unwrap(),
        include_str!("day11_part2_output.txt").trim_end_matches('\n')
    );
}
