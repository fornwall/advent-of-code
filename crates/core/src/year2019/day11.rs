use super::int_code::{Program, Word};
use crate::common::character_recognition::recognize;
use crate::input::Input;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    Black = 0,
    White = 1,
}

impl Color {
    fn from(value: Word) -> Result<Self, String> {
        Ok(match value {
            0 => Self::Black,
            1 => Self::White,
            _ => {
                return Err(format!("Invalid color value: {}", value));
            }
        })
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
        program.input(*painted.get(&position).unwrap_or(&Color::Black) as Word);
        let output = program.run_for_output()?;

        if program.is_halted() {
            break;
        }

        if output.len() != 2 {
            return Err("Invalid output length".to_string());
        }

        let painted_color = Color::from(output[0])?;
        let turn_direction = output[1];

        painted.insert(position, painted_color);

        current_direction = match turn_direction {
            0 => current_direction.turn_left(),
            1 => current_direction.turn_right(),
            _ => {
                return Err(format!("Invalid direction: {}", turn_direction));
            }
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

pub fn solve(input: &mut Input) -> Result<String, String> {
    let painted = run(input.text, input.part_values(Color::Black, Color::White))?;

    if input.is_part_one() {
        Ok(painted.len().to_string())
    } else {
        let mut min_x = std::i32::MAX;
        let mut max_x = std::i32::MIN;
        let mut min_y = std::i32::MAX;
        let mut max_y = std::i32::MIN;
        painted.iter().for_each(|(&(x, y), color)| {
            if *color == Color::White {
                min_x = std::cmp::min(min_x, x);
                max_x = std::cmp::max(max_x, x);
                min_y = std::cmp::min(min_y, y);
                max_y = std::cmp::max(max_y, y);
            }
        });

        let mut result = String::new();
        for x in (min_x..=max_x).step_by(5) {
            let mut this_char_string = String::new();
            for y in (min_y..=max_y).rev() {
                for char_x in x..(x + 5) {
                    this_char_string.push(if Some(&Color::White) == painted.get(&(char_x, y)) {
                        '█'
                    } else {
                        ' '
                    });
                }
                if y != min_y {
                    this_char_string.push('\n');
                }
            }
            result.push(recognize(&this_char_string)?);
        }

        Ok(result)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_two!("3,8,1005,8,291,1106,0,11,0,0,0,104,1,104,0,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,1002,8,1,28,1,1003,20,10,2,1103,19,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1001,8,0,59,1,1004,3,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,84,1006,0,3,1,1102,12,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,101,0,8,114,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,101,0,8,135,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,158,2,9,9,10,2,2,10,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,188,1006,0,56,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,1001,8,0,212,1006,0,76,2,1005,8,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1001,8,0,241,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1002,8,1,264,1006,0,95,1,1001,12,10,101,1,9,9,1007,9,933,10,1005,10,15,99,109,613,104,0,104,1,21102,838484206484,1,1,21102,1,308,0,1106,0,412,21102,1,937267929116,1,21101,0,319,0,1105,1,412,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,206312598619,1,1,21102,366,1,0,1105,1,412,21101,179410332867,0,1,21102,377,1,0,1105,1,412,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,709580595968,1,21102,1,400,0,1106,0,412,21102,868389384552,1,1,21101,411,0,0,1106,0,412,99,109,2,21202,-1,1,1,21102,1,40,2,21102,1,443,3,21101,0,433,0,1106,0,476,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,438,439,454,4,0,1001,438,1,438,108,4,438,10,1006,10,470,1102,0,1,438,109,-2,2106,0,0,0,109,4,1202,-1,1,475,1207,-3,0,10,1006,10,493,21102,0,1,-3,21202,-3,1,1,21201,-2,0,2,21101,0,1,3,21102,1,512,0,1106,0,517,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,540,2207,-4,-2,10,1006,10,540,22101,0,-4,-4,1106,0,608,21201,-4,0,1,21201,-3,-1,2,21202,-2,2,3,21101,0,559,0,1106,0,517,21201,1,0,-4,21102,1,1,-1,2207,-4,-2,10,1006,10,578,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,600,21201,-1,0,1,21102,600,1,0,106,0,475,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2106,0,0
" => "UERPRFGJ".into());

    let input = include_str!("day11_input.txt");
    test_part_one!(input => "1686".into());
    test_part_two!(input => "GARPKZUL".into());
}
