use std::collections::{HashSet, VecDeque};

use crate::input::Input;

use super::int_code::Program;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const fn reverse(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::North => "north",
            Self::East => "east",
            Self::South => "south",
            Self::West => "west",
        }
    }

    fn from_str(string: &str) -> Option<Self> {
        match string {
            "north" => Some(Self::North),
            "east" => Some(Self::East),
            "south" => Some(Self::South),
            "west" => Some(Self::West),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
enum Command<'a> {
    Move(Direction),
    Take(&'a str),
    Drop(&'a str),
}

struct Room {
    id: String,
    directions: Vec<Direction>,
    items: Vec<String>,
    solution: ResultFromPressureSensor,
}

type SolutionType = i64;

enum ResultFromPressureSensor {
    None,
    Solution(SolutionType),
    TooLight,
    TooHeavy,
}

fn execute_command(program: &mut Program, command: Command) -> Result<Room, String> {
    match command {
        Command::Move(direction) => {
            program.input_string(&format!("{}\n", direction.as_str()));
        }
        Command::Take(item) => {
            program.input_string(&format!("take {item}\n"));
        }
        Command::Drop(item) => {
            program.input_string(&format!("drop {item}\n"));
        }
    }

    parse_output(program)
}

fn parse_output(program: &mut Program) -> Result<Room, String> {
    let output = program.run_for_output()?;
    let output: Vec<u8> = output.iter().map(|&b| b as u8).collect();
    let output = std::str::from_utf8(&output).map_err(|_| "Invalid input: Not utf-8")?;

    let mut directions = Vec::new();
    let mut items = Vec::new();
    let mut room_id = "";
    let mut solution = ResultFromPressureSensor::None;

    for line in output.lines() {
        if line.starts_with("== ") {
            // This takes the second if bounced from "Pressure-Sensitive Floor".
            room_id = line;
        } else if line.starts_with("A loud") {
            solution = if line.contains("lighter") {
                ResultFromPressureSensor::TooHeavy
            } else {
                ResultFromPressureSensor::TooLight
            };
        } else if let Some(item) = line.strip_prefix("- ") {
            Direction::from_str(item).map_or_else(
                || {
                    items.push(item.to_string());
                },
                |direction| {
                    directions.push(direction);
                },
            );
        } else if line.starts_with("\"Oh, hello! You should be able to get in by typing") {
            let error_message = "Unable to parse typing instruction";
            solution = ResultFromPressureSensor::Solution(
                line.split_whitespace()
                    .nth(11)
                    .ok_or(error_message)?
                    .parse::<SolutionType>()
                    .map_err(|_| error_message)?,
            );
        }
    }

    Ok(Room {
        id: room_id.to_string(),
        directions,
        items,
        solution,
    })
}

pub fn solve(input: &Input) -> Result<SolutionType, String> {
    let mut program = Program::parse(input.text)?;
    let initial_room = parse_output(&mut program)?;

    let mut blacklisted_items = HashSet::new();
    blacklisted_items.insert("infinite loop".to_string());
    blacklisted_items.insert("photons".to_string());
    blacklisted_items.insert("giant electromagnet".to_string());
    blacklisted_items.insert("escape pod".to_string());
    blacklisted_items.insert("molten lava".to_string());

    let mut carried_items = Vec::new();

    let mut visited_rooms = HashSet::new();

    let mut to_visit = VecDeque::new();
    to_visit.push_back((initial_room, Vec::new()));

    let mut directions_to_security_checkpoint = Vec::new();
    let mut direction_to_pressure_sensitive_floor = Direction::North;

    while let Some((from_room, directions_to_reach_here)) = to_visit.pop_front() {
        for &direction in directions_to_reach_here.iter() {
            execute_command(&mut program, Command::Move(direction))?;
        }

        for &direction in from_room.directions.iter() {
            let new_room = execute_command(&mut program, Command::Move(direction))?;

            if new_room.id == from_room.id {
                // Pushed back.
                direction_to_pressure_sensitive_floor = direction;
            } else {
                let mut new_directions = directions_to_reach_here.clone();
                new_directions.push(direction);

                if new_room.id == "== Security Checkpoint ==" {
                    directions_to_security_checkpoint = new_directions.clone();
                }

                let new_room_id = new_room.id.clone();
                if visited_rooms.insert(new_room_id) {
                    for item in new_room
                        .items
                        .iter()
                        .filter(|&item| !blacklisted_items.contains(item))
                    {
                        execute_command(&mut program, Command::Take(item))?;
                        carried_items.push(item.clone());
                    }
                    to_visit.push_back((new_room, new_directions));
                }

                execute_command(&mut program, Command::Move(direction.reverse()))?;
            }
        }

        // Go back to starting point.
        for &direction in directions_to_reach_here.iter().rev() {
            execute_command(&mut program, Command::Move(direction.reverse()))?;
        }
    }

    // Go to security checkpoint:
    for &direction in directions_to_security_checkpoint.iter() {
        execute_command(&mut program, Command::Move(direction))?;
    }

    // Drop all items:
    for item in carried_items.iter() {
        execute_command(&mut program, Command::Drop(item))?;
    }

    // Keep track off too light or too heavy combinations.
    let mut too_light = Vec::new();
    let mut too_heavy = Vec::new();

    // Try all combinations of items using Gray code,
    // https://en.wikipedia.org/wiki/Gray_code#Constructing_an_n-bit_Gray_code,
    // to minimize the number of take and drop commands:
    let mut latest_gray_code = 0;
    for i in 1..=(1 << carried_items.len()) {
        let gray_code = i ^ (i >> 1);

        #[allow(clippy::manual_contains)]
        if too_heavy.iter().any(|&heavy| heavy & gray_code == heavy)
            || too_light.iter().any(|&light| light | gray_code == light)
        {
            continue;
        }

        for (j, item) in carried_items.iter().enumerate() {
            let bit_mask = 1 << j;
            if gray_code & bit_mask != 0 && latest_gray_code & bit_mask == 0 {
                execute_command(&mut program, Command::Take(item))?;
            } else if latest_gray_code & bit_mask != 0 && gray_code & bit_mask == 0 {
                execute_command(&mut program, Command::Drop(item))?;
            }
        }

        latest_gray_code = gray_code;

        let new_room = execute_command(
            &mut program,
            Command::Move(direction_to_pressure_sensitive_floor),
        )?;
        match new_room.solution {
            ResultFromPressureSensor::Solution(solution) => {
                assert!(!too_light.is_empty());
                assert!(!too_heavy.is_empty());
                return Ok(solution);
            }
            ResultFromPressureSensor::TooLight => {
                too_light.push(gray_code);
            }
            ResultFromPressureSensor::TooHeavy => {
                too_heavy.push(gray_code);
            }
            _ => {}
        }
    }

    Err("No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::input::test_part_one;
    let input = include_str!("day25_input.txt");
    test_part_one!(input => 319_815_680);
    let input = include_str!("day25_input_2.txt");
    test_part_one!(input => 2_424_308_736);
}
