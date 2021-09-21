use super::int_code::Program;
use crate::Input;
use std::collections::VecDeque;
use std::slice::Iter;

fn part1_map(map: &str) -> Result<String, String> {
    let map: Vec<&[u8]> = map.trim().lines().map(str::as_bytes).collect();
    if map.len() < 3 {
        return Err("Too small input (less than three lines)".to_string());
    } else if map.iter().filter(|row| row.len() != map[0].len()).count() > 0 {
        return Err("Invalid map - not all rows are of equal length".to_string());
    }

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

    Ok(alignment_parameters_sum.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up = 0,
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

    const fn other(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    const fn advance(self, position: (i32, i32)) -> (i32, i32) {
        match self {
            Self::Up => (position.0, position.1 - 1),
            Self::Right => (position.0 + 1, position.1),
            Self::Down => (position.0, position.1 + 1),
            Self::Left => (position.0 - 1, position.1),
        }
    }

    fn instruction_for_turning_to(self, target: Self) -> Result<char, String> {
        if self.turn_right() == target {
            Ok('R')
        } else if self.turn_left() == target {
            Ok('L')
        } else {
            Err(format!("Cannot turn from {:?} to {:?}", self, target))
        }
    }

    pub fn iterator() -> Iter<'static, Self> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        DIRECTIONS.iter()
    }
}

// Solution taken from https://github.com/emlun/adventofcode-2019/blob/master/src/days/day17.rs
pub fn solve(input: &mut Input) -> Result<String, String> {
    let mut program = Program::parse(input.text)?;

    if input.is_part_one() {
        let output = program.run_for_output()?;
        let map: String = output.iter().map(|&b| (b as u8) as char).collect();
        return part1_map(&map);
    }

    program.write_memory(0, 2);

    let output = program.run_for_output()?;
    let map: String = output.iter().map(|&b| (b as u8) as char).collect();
    let map: Vec<&[u8]> = map.lines().map(str::as_bytes).collect();
    // Strip away last two lines with blank line and "Main:" prompt:
    if map.len() < 5 {
        return Err("Too small input (less than five lines)".to_string());
    }
    let map = &map[0..(map.len() - 2)];

    if map.iter().filter(|row| row.len() != map[0].len()).count() > 0 {
        return Err("Invalid map - not all rows are of equal length".to_string());
    }

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
    let mut movements = Vec::new();

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
                if moves_since_turn != 0 {
                    return Err("Starting with moves already performed".to_string());
                }
                starting = false;
                movements.push(
                    robot_direction
                        .instruction_for_turning_to(possible_directions[0])?
                        .to_string(),
                );
                robot_direction = possible_directions[0];
                moves_since_turn = 0;
            } else {
                // Done.
                if moves_since_turn > 0 {
                    movements.push(moves_since_turn.to_string());
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
                    movements.push(moves_since_turn.to_string());
                    moves_since_turn = 0;
                }
                movements.push(
                    robot_direction
                        .instruction_for_turning_to(new_direction)?
                        .to_string(),
                );
                robot_direction = new_direction;
            }
        } else if possible_directions.len() == 4 {
            robot_position = robot_direction.advance(robot_position);
            moves_since_turn += 1;
        } else {
            return Err(format!(
                "Invalid possible directions: {}",
                possible_directions.len()
            ));
        }
    }

    if let Some((segments, sequence)) = find_covering_subsequences(&movements, 3) {
        let function_a = segments[0];
        let function_b = segments[1];
        let function_c = segments[2];
        let main_routine: Vec<String> = sequence
            .iter()
            .map(|&i| ((b'A' + i as u8) as char).to_string())
            .collect();
        for input in [&main_routine, function_a, function_b, function_c] {
            program.input_string(&input.join(","));
            program.input_string("\n");
        }
        program.input_string("n\n");
        let last_output = program.run_for_output()?;
        return last_output
            .iter()
            .find(|&&value| value > 255)
            .map(i64::to_string)
            .ok_or_else(|| "No output > 255 produced".to_string());
    }

    Err("No output produced".to_string())
}

fn subsequence_exists<T>(seq: &[T], subseq: &[T]) -> bool
where
    T: PartialEq,
{
    if subseq.len() > seq.len() {
        return false;
    }
    (0..(seq.len() - subseq.len())).any(|i| seq[i..].starts_with(subseq))
}

fn find_longest_repeated_subsequence(sequence: &[String]) -> Option<&[String]> {
    let mut end_min = 0;
    let mut end_max = sequence.len();

    while end_max > end_min {
        let end = (end_max + end_min) / 2;
        if end == end_min {
            break;
        } else if subsequence_exists(&sequence[end..], &sequence[0..end]) {
            end_min = end;
        } else {
            end_max = end;
        }
    }

    if end_min > 0 {
        Some(&sequence[0..end_min])
    } else {
        None
    }
}

fn find_subseq_covering(seq: &[String], subseqs: &[&[String]]) -> Option<VecDeque<usize>> {
    if seq.is_empty() {
        Some(VecDeque::new())
    } else {
        for (i, subseq) in subseqs.iter().enumerate() {
            if seq.starts_with(subseq) {
                if let Some(mut subfind) = find_subseq_covering(&seq[subseq.len()..], subseqs) {
                    subfind.push_front(i);
                    return Some(subfind);
                }
            }
        }

        None
    }
}

fn find_covering_subsequences(
    seq: &[String],
    num_subsequences: usize,
) -> Option<(Vec<&[String]>, VecDeque<usize>)> {
    fn fill_subsequences<'a>(
        seq: &'a [String],
        num_subsequences: usize,
        mut subsequences: Vec<&'a [String]>,
    ) -> Option<Vec<&'a [String]>> {
        if seq.is_empty() || subsequences.len() == num_subsequences {
            Some(subsequences)
        } else if let Some(prefix) = subsequences.iter().find(|subseq| seq.starts_with(subseq)) {
            fill_subsequences(&seq[prefix.len()..], num_subsequences, subsequences)
        } else {
            let next = find_longest_repeated_subsequence(seq)?;
            subsequences.push(next);
            fill_subsequences(&seq[next.len()..], num_subsequences, subsequences)
        }
    }

    let mut subsequences: Vec<&[String]> = fill_subsequences(seq, num_subsequences, Vec::new())?;

    while !subsequences[0].is_empty() {
        if let Some(covering) = find_subseq_covering(seq, &subsequences) {
            if !subsequences.iter().any(|s| s.join(",").len() > 20) {
                return Some((subsequences, covering));
            }
        }

        while !subsequences.is_empty() {
            let i = subsequences.len() - 1;
            subsequences[i] = subsequences[i].split_last()?.1;
            if subsequences[subsequences.len() - 1].is_empty() {
                subsequences.pop();
            } else {
                subsequences = fill_subsequences(seq, num_subsequences, subsequences)?;
                break;
            }
        }

        if subsequences.is_empty() {
            return None;
        }
    }
    None
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

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
        Ok("76".to_string())
    );

    test_part_two!("1,330,331,332,109,3546,1101,0,1182,15,1101,1481,0,24,1001,0,0,570,1006,570,36,102,1,571,0,1001,570,-1,570,1001,24,1,24,1105,1,18,1008,571,0,571,1001,15,1,15,1008,15,1481,570,1006,570,14,21102,58,1,0,1106,0,786,1006,332,62,99,21101,0,333,1,21101,0,73,0,1106,0,579,1101,0,0,572,1101,0,0,573,3,574,101,1,573,573,1007,574,65,570,1005,570,151,107,67,574,570,1005,570,151,1001,574,-64,574,1002,574,-1,574,1001,572,1,572,1007,572,11,570,1006,570,165,101,1182,572,127,1002,574,1,0,3,574,101,1,573,573,1008,574,10,570,1005,570,189,1008,574,44,570,1006,570,158,1105,1,81,21102,1,340,1,1106,0,177,21102,1,477,1,1106,0,177,21101,0,514,1,21102,1,176,0,1105,1,579,99,21102,1,184,0,1106,0,579,4,574,104,10,99,1007,573,22,570,1006,570,165,102,1,572,1182,21102,375,1,1,21101,211,0,0,1106,0,579,21101,1182,11,1,21101,0,222,0,1106,0,979,21102,388,1,1,21102,1,233,0,1106,0,579,21101,1182,22,1,21102,1,244,0,1106,0,979,21101,0,401,1,21102,255,1,0,1106,0,579,21101,1182,33,1,21102,266,1,0,1105,1,979,21102,414,1,1,21102,1,277,0,1105,1,579,3,575,1008,575,89,570,1008,575,121,575,1,575,570,575,3,574,1008,574,10,570,1006,570,291,104,10,21102,1,1182,1,21102,1,313,0,1105,1,622,1005,575,327,1102,1,1,575,21101,0,327,0,1106,0,786,4,438,99,0,1,1,6,77,97,105,110,58,10,33,10,69,120,112,101,99,116,101,100,32,102,117,110,99,116,105,111,110,32,110,97,109,101,32,98,117,116,32,103,111,116,58,32,0,12,70,117,110,99,116,105,111,110,32,65,58,10,12,70,117,110,99,116,105,111,110,32,66,58,10,12,70,117,110,99,116,105,111,110,32,67,58,10,23,67,111,110,116,105,110,117,111,117,115,32,118,105,100,101,111,32,102,101,101,100,63,10,0,37,10,69,120,112,101,99,116,101,100,32,82,44,32,76,44,32,111,114,32,100,105,115,116,97,110,99,101,32,98,117,116,32,103,111,116,58,32,36,10,69,120,112,101,99,116,101,100,32,99,111,109,109,97,32,111,114,32,110,101,119,108,105,110,101,32,98,117,116,32,103,111,116,58,32,43,10,68,101,102,105,110,105,116,105,111,110,115,32,109,97,121,32,98,101,32,97,116,32,109,111,115,116,32,50,48,32,99,104,97,114,97,99,116,101,114,115,33,10,94,62,118,60,0,1,0,-1,-1,0,1,0,0,0,0,0,0,1,12,18,0,109,4,2102,1,-3,587,20101,0,0,-1,22101,1,-3,-3,21101,0,0,-2,2208,-2,-1,570,1005,570,617,2201,-3,-2,609,4,0,21201,-2,1,-2,1106,0,597,109,-4,2106,0,0,109,5,2102,1,-4,630,20102,1,0,-2,22101,1,-4,-4,21101,0,0,-3,2208,-3,-2,570,1005,570,781,2201,-4,-3,653,20102,1,0,-1,1208,-1,-4,570,1005,570,709,1208,-1,-5,570,1005,570,734,1207,-1,0,570,1005,570,759,1206,-1,774,1001,578,562,684,1,0,576,576,1001,578,566,692,1,0,577,577,21101,0,702,0,1105,1,786,21201,-1,-1,-1,1106,0,676,1001,578,1,578,1008,578,4,570,1006,570,724,1001,578,-4,578,21102,731,1,0,1105,1,786,1106,0,774,1001,578,-1,578,1008,578,-1,570,1006,570,749,1001,578,4,578,21102,1,756,0,1105,1,786,1105,1,774,21202,-1,-11,1,22101,1182,1,1,21101,0,774,0,1106,0,622,21201,-3,1,-3,1106,0,640,109,-5,2106,0,0,109,7,1005,575,802,21001,576,0,-6,20102,1,577,-5,1106,0,814,21102,1,0,-1,21102,0,1,-5,21102,0,1,-6,20208,-6,576,-2,208,-5,577,570,22002,570,-2,-2,21202,-5,59,-3,22201,-6,-3,-3,22101,1481,-3,-3,2101,0,-3,843,1005,0,863,21202,-2,42,-4,22101,46,-4,-4,1206,-2,924,21102,1,1,-1,1105,1,924,1205,-2,873,21102,35,1,-4,1105,1,924,2101,0,-3,878,1008,0,1,570,1006,570,916,1001,374,1,374,1202,-3,1,895,1101,0,2,0,2101,0,-3,902,1001,438,0,438,2202,-6,-5,570,1,570,374,570,1,570,438,438,1001,578,558,921,21002,0,1,-4,1006,575,959,204,-4,22101,1,-6,-6,1208,-6,59,570,1006,570,814,104,10,22101,1,-5,-5,1208,-5,35,570,1006,570,810,104,10,1206,-1,974,99,1206,-1,974,1101,0,1,575,21102,973,1,0,1105,1,786,99,109,-7,2105,1,0,109,6,21101,0,0,-4,21102,0,1,-3,203,-2,22101,1,-3,-3,21208,-2,82,-1,1205,-1,1030,21208,-2,76,-1,1205,-1,1037,21207,-2,48,-1,1205,-1,1124,22107,57,-2,-1,1205,-1,1124,21201,-2,-48,-2,1106,0,1041,21102,1,-4,-2,1106,0,1041,21101,0,-5,-2,21201,-4,1,-4,21207,-4,11,-1,1206,-1,1138,2201,-5,-4,1059,1202,-2,1,0,203,-2,22101,1,-3,-3,21207,-2,48,-1,1205,-1,1107,22107,57,-2,-1,1205,-1,1107,21201,-2,-48,-2,2201,-5,-4,1090,20102,10,0,-1,22201,-2,-1,-2,2201,-5,-4,1103,1202,-2,1,0,1105,1,1060,21208,-2,10,-1,1205,-1,1162,21208,-2,44,-1,1206,-1,1131,1105,1,989,21101,0,439,1,1106,0,1150,21102,477,1,1,1106,0,1150,21101,0,514,1,21102,1,1149,0,1105,1,579,99,21101,0,1157,0,1106,0,579,204,-2,104,10,99,21207,-3,22,-1,1206,-1,1138,2101,0,-5,1176,1201,-4,0,0,109,-6,2105,1,0,6,13,27,13,6,1,11,1,27,1,11,1,6,1,11,1,27,1,11,1,6,1,11,1,27,1,11,1,6,1,11,1,27,1,11,1,6,1,11,1,27,1,11,1,6,1,11,1,1,9,9,11,9,1,6,1,11,1,1,1,7,1,9,1,7,1,1,1,9,1,6,1,11,13,7,1,7,1,1,1,9,1,6,1,13,1,7,1,1,1,7,1,7,1,1,1,9,1,6,1,13,1,7,1,1,1,5,11,1,1,9,1,6,1,13,1,7,1,1,1,5,1,1,1,9,1,9,1,6,11,3,1,7,1,1,1,5,1,1,1,9,1,1,9,16,1,3,1,7,1,1,1,5,1,1,1,9,1,1,1,24,1,3,1,7,13,7,1,1,1,24,1,3,1,9,1,5,1,1,1,1,1,7,1,1,1,24,1,3,1,9,9,1,1,7,11,16,1,3,1,15,1,3,1,9,1,7,1,12,9,15,1,3,1,9,1,7,1,16,1,19,1,3,1,9,1,7,1,16,1,19,11,3,1,7,1,16,1,23,1,5,1,3,1,7,1,8,9,23,11,7,1,8,1,37,1,11,1,8,1,37,1,11,1,8,1,37,1,11,1,8,1,37,1,11,1,8,1,37,1,11,1,8,1,37,13,8,1,58,1,58,1,58,1,58,1,50,9,50" => "933214".into());
    test_part_two!("1,330,331,332,109,3080,1101,0,1182,15,1101,0,1403,24,1001,0,0,570,1006,570,36,1002,571,1,0,1001,570,-1,570,1001,24,1,24,1105,1,18,1008,571,0,571,1001,15,1,15,1008,15,1403,570,1006,570,14,21101,58,0,0,1105,1,786,1006,332,62,99,21102,333,1,1,21101,0,73,0,1105,1,579,1102,0,1,572,1101,0,0,573,3,574,101,1,573,573,1007,574,65,570,1005,570,151,107,67,574,570,1005,570,151,1001,574,-64,574,1002,574,-1,574,1001,572,1,572,1007,572,11,570,1006,570,165,101,1182,572,127,101,0,574,0,3,574,101,1,573,573,1008,574,10,570,1005,570,189,1008,574,44,570,1006,570,158,1106,0,81,21102,1,340,1,1105,1,177,21102,477,1,1,1105,1,177,21102,1,514,1,21102,1,176,0,1106,0,579,99,21101,184,0,0,1105,1,579,4,574,104,10,99,1007,573,22,570,1006,570,165,1002,572,1,1182,21101,0,375,1,21102,211,1,0,1105,1,579,21101,1182,11,1,21102,1,222,0,1106,0,979,21101,0,388,1,21102,233,1,0,1105,1,579,21101,1182,22,1,21101,244,0,0,1105,1,979,21101,0,401,1,21101,255,0,0,1106,0,579,21101,1182,33,1,21101,266,0,0,1106,0,979,21101,0,414,1,21102,1,277,0,1105,1,579,3,575,1008,575,89,570,1008,575,121,575,1,575,570,575,3,574,1008,574,10,570,1006,570,291,104,10,21101,1182,0,1,21101,313,0,0,1105,1,622,1005,575,327,1101,1,0,575,21101,327,0,0,1106,0,786,4,438,99,0,1,1,6,77,97,105,110,58,10,33,10,69,120,112,101,99,116,101,100,32,102,117,110,99,116,105,111,110,32,110,97,109,101,32,98,117,116,32,103,111,116,58,32,0,12,70,117,110,99,116,105,111,110,32,65,58,10,12,70,117,110,99,116,105,111,110,32,66,58,10,12,70,117,110,99,116,105,111,110,32,67,58,10,23,67,111,110,116,105,110,117,111,117,115,32,118,105,100,101,111,32,102,101,101,100,63,10,0,37,10,69,120,112,101,99,116,101,100,32,82,44,32,76,44,32,111,114,32,100,105,115,116,97,110,99,101,32,98,117,116,32,103,111,116,58,32,36,10,69,120,112,101,99,116,101,100,32,99,111,109,109,97,32,111,114,32,110,101,119,108,105,110,101,32,98,117,116,32,103,111,116,58,32,43,10,68,101,102,105,110,105,116,105,111,110,115,32,109,97,121,32,98,101,32,97,116,32,109,111,115,116,32,50,48,32,99,104,97,114,97,99,116,101,114,115,33,10,94,62,118,60,0,1,0,-1,-1,0,1,0,0,0,0,0,0,1,20,26,0,109,4,1202,-3,1,587,20102,1,0,-1,22101,1,-3,-3,21101,0,0,-2,2208,-2,-1,570,1005,570,617,2201,-3,-2,609,4,0,21201,-2,1,-2,1106,0,597,109,-4,2106,0,0,109,5,1202,-4,1,630,20101,0,0,-2,22101,1,-4,-4,21102,0,1,-3,2208,-3,-2,570,1005,570,781,2201,-4,-3,653,20102,1,0,-1,1208,-1,-4,570,1005,570,709,1208,-1,-5,570,1005,570,734,1207,-1,0,570,1005,570,759,1206,-1,774,1001,578,562,684,1,0,576,576,1001,578,566,692,1,0,577,577,21101,0,702,0,1105,1,786,21201,-1,-1,-1,1106,0,676,1001,578,1,578,1008,578,4,570,1006,570,724,1001,578,-4,578,21102,1,731,0,1106,0,786,1105,1,774,1001,578,-1,578,1008,578,-1,570,1006,570,749,1001,578,4,578,21101,756,0,0,1105,1,786,1106,0,774,21202,-1,-11,1,22101,1182,1,1,21101,774,0,0,1105,1,622,21201,-3,1,-3,1106,0,640,109,-5,2105,1,0,109,7,1005,575,802,20101,0,576,-6,21002,577,1,-5,1106,0,814,21102,0,1,-1,21101,0,0,-5,21101,0,0,-6,20208,-6,576,-2,208,-5,577,570,22002,570,-2,-2,21202,-5,43,-3,22201,-6,-3,-3,22101,1403,-3,-3,2102,1,-3,843,1005,0,863,21202,-2,42,-4,22101,46,-4,-4,1206,-2,924,21101,0,1,-1,1106,0,924,1205,-2,873,21101,35,0,-4,1106,0,924,1201,-3,0,878,1008,0,1,570,1006,570,916,1001,374,1,374,1201,-3,0,895,1101,2,0,0,1201,-3,0,902,1001,438,0,438,2202,-6,-5,570,1,570,374,570,1,570,438,438,1001,578,558,922,20102,1,0,-4,1006,575,959,204,-4,22101,1,-6,-6,1208,-6,43,570,1006,570,814,104,10,22101,1,-5,-5,1208,-5,39,570,1006,570,810,104,10,1206,-1,974,99,1206,-1,974,1101,0,1,575,21101,0,973,0,1105,1,786,99,109,-7,2105,1,0,109,6,21101,0,0,-4,21101,0,0,-3,203,-2,22101,1,-3,-3,21208,-2,82,-1,1205,-1,1030,21208,-2,76,-1,1205,-1,1037,21207,-2,48,-1,1205,-1,1124,22107,57,-2,-1,1205,-1,1124,21201,-2,-48,-2,1106,0,1041,21102,1,-4,-2,1106,0,1041,21102,1,-5,-2,21201,-4,1,-4,21207,-4,11,-1,1206,-1,1138,2201,-5,-4,1059,1201,-2,0,0,203,-2,22101,1,-3,-3,21207,-2,48,-1,1205,-1,1107,22107,57,-2,-1,1205,-1,1107,21201,-2,-48,-2,2201,-5,-4,1090,20102,10,0,-1,22201,-2,-1,-2,2201,-5,-4,1103,2101,0,-2,0,1106,0,1060,21208,-2,10,-1,1205,-1,1162,21208,-2,44,-1,1206,-1,1131,1105,1,989,21101,0,439,1,1106,0,1150,21102,1,477,1,1106,0,1150,21101,514,0,1,21101,1149,0,0,1106,0,579,99,21101,1157,0,0,1105,1,579,204,-2,104,10,99,21207,-3,22,-1,1206,-1,1138,1201,-5,0,1176,2101,0,-4,0,109,-6,2105,1,0,28,5,38,1,3,1,38,1,3,1,38,1,3,1,38,1,3,1,38,1,3,1,34,9,34,1,3,1,38,1,3,1,38,1,3,1,34,9,34,1,3,1,38,1,3,1,38,1,3,1,34,5,3,5,30,1,11,1,30,1,11,1,30,1,11,1,22,9,11,5,18,1,23,1,18,1,23,1,18,1,23,1,10,7,1,1,19,9,6,1,5,1,1,1,19,1,3,1,3,1,6,1,5,1,1,5,15,1,3,1,3,1,6,1,5,1,5,1,15,1,3,1,3,1,6,11,1,1,1,7,3,5,3,11,6,1,3,1,1,1,1,1,9,1,11,1,5,1,6,1,1,9,7,1,11,1,5,1,6,1,1,1,1,1,1,1,1,1,9,1,11,1,5,1,6,9,1,9,11,7,8,1,1,1,1,1,3,1,30,9,3,1,30,1,3,1,1,1,5,1,30,1,3,1,1,7,30,1,3,1,38,1,3,1,38,1,3,1,38,5,34" => "714866".into());

    let input = include_str!("day17_input.txt");
    test_part_one!(input => "11140".into());
    test_part_two!(input => "1113108".into());
}
