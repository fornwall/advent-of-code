use crate::int_code::Program;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    fn reverse(self) -> Direction {
        match self {
            Direction::NORTH => Direction::SOUTH,
            Direction::EAST => Direction::WEST,
            Direction::SOUTH => Direction::NORTH,
            Direction::WEST => Direction::EAST,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Direction::NORTH => "north",
            Direction::EAST => "east",
            Direction::SOUTH => "south",
            Direction::WEST => "west",
        }
    }

    fn from_str(string: &str) -> Direction {
        match string {
            "north" => Direction::NORTH,
            "east" => Direction::EAST,
            "south" => Direction::SOUTH,
            "west" => Direction::WEST,
            _ => {
                panic!("Invalid direction: {}", string);
            }
        }
    }
}

enum Command {
    Move(Direction),
    Take(String),
    Drop(String),
}

struct Room {
    id: String,
    directions: Vec<Direction>,
    items: Vec<String>,
    solution: i32,
}

fn execute_command(program: &mut Program, command: Command) -> Room {
    match command {
        Command::Move(direction) => {
            program.input_string(&format!("{}\n", direction.as_str()));
        }
        Command::Take(item) => {
            program.input_string(&format!("take {}\n", item));
        }
        Command::Drop(item) => {
            program.input_string(&format!("drop {}\n", item));
        }
    }

    parse_output(program)
}

fn parse_output(program: &mut Program) -> Room {
    let output: Vec<u8> = program.run_for_output().iter().map(|&b| b as u8).collect();
    let output = std::str::from_utf8(&output).unwrap();

    let mut directions = Vec::new();
    let mut items = Vec::new();
    let mut room_id = "";
    let mut solution = -1;
    for line in output.lines() {
        if line.starts_with("== ") {
            // This takes the second if bounced from "Pressure-Sensitive Floor".
            room_id = line;
        } else if line.starts_with("- ") {
            match &line[2..] {
                "north" | "east" | "south" | "west" => {
                    directions.push(Direction::from_str(&line[2..]));
                }
                _ => {
                    items.push((&line[2..]).to_string());
                }
            }
        } else if line.starts_with("\"Oh, hello! You should be able to get in by typing") {
            solution = line
                .split_whitespace()
                .nth(11)
                .unwrap()
                .parse::<i32>()
                .unwrap();
        }
    }

    Room {
        id: room_id.to_string(),
        directions,
        items,
        solution,
    }
}

pub fn part1(input_string: &str) -> String {
    let mut program = Program::parse(input_string);
    let initial_room = parse_output(&mut program);

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
    let mut direction_to_pressure_sensitive_floor = Direction::NORTH;

    while let Some((from_room, directions_to_reach_here)) = to_visit.pop_front() {
        for &direction in directions_to_reach_here.iter() {
            execute_command(&mut program, Command::Move(direction));
        }

        for &direction in from_room.directions.iter() {
            let new_room = execute_command(&mut program, Command::Move(direction));

            if new_room.id == from_room.id {
                // Pushed back.
                direction_to_pressure_sensitive_floor = direction;
            } else {
                let mut new_directions = directions_to_reach_here.clone();
                new_directions.push(direction);

                if new_room.id.as_str() == "== Security Checkpoint ==" {
                    directions_to_security_checkpoint = new_directions.clone();
                }

                let new_room_id = new_room.id.clone();
                if visited_rooms.insert(new_room_id) {
                    for item in new_room
                        .items
                        .iter()
                        .filter(|&item| !blacklisted_items.contains(item))
                    {
                        execute_command(&mut program, Command::Take(item.clone()));
                        carried_items.push(item.clone());
                    }

                    to_visit.push_back((new_room, new_directions.clone()));
                }

                execute_command(&mut program, Command::Move(direction.reverse()));
            }
        }

        // Go back to starting point.
        for &direction in directions_to_reach_here.iter().rev() {
            execute_command(&mut program, Command::Move(direction.reverse()));
        }
    }

    // Go to security checkpoint:
    for &direction in directions_to_security_checkpoint.iter() {
        execute_command(&mut program, Command::Move(direction));
    }

    //let all_items = carried_items.clone();
    // Try all combinations of items:
    for i in 0..=(1 << carried_items.len()) {
        let mut items_to_use = Vec::new();
        for (j, item) in carried_items.iter().enumerate() {
            if i & (1 << j) != 0 {
                items_to_use.push(item.clone());
            }
        }
        for item in carried_items.iter() {
            execute_command(&mut program, Command::Drop(item.clone()));
        }
        for item in items_to_use.iter() {
            execute_command(&mut program, Command::Take(item.clone()));
        }

        let new_room = execute_command(
            &mut program,
            Command::Move(direction_to_pressure_sensitive_floor),
        );
        if new_room.solution != -1 {
            return new_room.solution.to_string();
        }
    }

    String::from("No solution found")
}

pub fn part2(_input_string: &str) -> String {
    String::from("")
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day25_input.txt")), "319815680");
}
