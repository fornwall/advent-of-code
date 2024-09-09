use crate::input::Input;
use std::cmp::{max, min};
use std::collections::HashSet;

struct Point {
    x: i32,
    y: i32,
    x_speed: i32,
    y_speed: i32,
}

pub fn solve(input: &Input) -> Result<String, String> {
    let mut points: Vec<Point> = input
        .text
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            let error = || format!("Invalid input at line {}", line_index + 1);

            let parts: Vec<&str> = line.split(['<', '>', ',']).collect();
            if parts.len() < 6 || !line.starts_with("position=") {
                return Err(error());
            }

            let error_mapper = |_| error();
            let x = parts[1].trim().parse::<i32>().map_err(error_mapper)?;
            let y = parts[2].trim().parse::<i32>().map_err(error_mapper)?;
            let x_speed = parts[4].trim().parse::<i32>().map_err(error_mapper)?;
            let y_speed = parts[5].trim().parse::<i32>().map_err(error_mapper)?;
            Ok(Point {
                x,
                y,
                x_speed,
                y_speed,
            })
        })
        .collect::<Result<_, String>>()?;

    let mut previous_height = i32::MAX;
    let mut seconds = 0;
    loop {
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        for point in &mut points {
            point.x += point.x_speed;
            point.y += point.y_speed;
            min_y = min(min_y, point.y);
            max_y = max(max_y, point.y);
        }

        let this_height = max_y - min_y;
        if this_height > previous_height {
            break;
        };
        previous_height = this_height;
        seconds += 1;
    }

    if input.is_part_two() {
        return Ok(seconds.to_string());
    }

    let mut occupied = HashSet::new();
    let mut borders = (i32::MAX, i32::MIN, i32::MIN, i32::MAX);
    for point in &mut points {
        // Step back after last expanding step.
        point.x -= point.x_speed;
        point.y -= point.y_speed;

        borders.0 = min(borders.0, point.y);
        borders.1 = max(borders.1, point.x);
        borders.2 = max(borders.2, point.y);
        borders.3 = min(borders.3, point.x);

        occupied.insert((point.x, point.y));
    }

    let mut result = String::new();
    for y in borders.0..=borders.2 {
        for x in borders.3..=borders.1 {
            result += if occupied.contains(&(x, y)) { "#" } else { "." }
        }
        result += "\n";
    }

    let identified_chars = identify_chars(&result)?;
    Ok(identified_chars)
}

fn identify_char(input: &str) -> Result<char, String> {
    Ok(match input {
        "..##..\n.#..#.\n#....#\n#....#\n#....#\n######\n#....#\n#....#\n#....#\n#....#\n" => 'A',
        "#####.\n#....#\n#....#\n#....#\n#####.\n#....#\n#....#\n#....#\n#....#\n#####.\n" => 'B',
        ".####.\n#....#\n#.....\n#.....\n#.....\n#.....\n#.....\n#.....\n#....#\n.####.\n" => 'C',
        "######\n#.....\n#.....\n#.....\n#####.\n#.....\n#.....\n#.....\n#.....\n######\n" => 'E',
        "######\n#.....\n#.....\n#.....\n#####.\n#.....\n#.....\n#.....\n#.....\n#.....\n" => 'F',
        ".####.\n#....#\n#.....\n#.....\n#.....\n#..###\n#....#\n#....#\n#...##\n.###.#\n" => 'G',
        "#....#\n#....#\n#....#\n#....#\n######\n#....#\n#....#\n#....#\n#....#\n#....#\n" => 'H',
        "...###\n....#.\n....#.\n....#.\n....#.\n....#.\n....#.\n#...#.\n#...#.\n.###..\n" => 'J',
        "#....#\n#...#.\n#..#..\n#.#...\n##....\n##....\n#.#...\n#..#..\n#...#.\n#....#\n" => 'K',
        "#.....\n#.....\n#.....\n#.....\n#.....\n#.....\n#.....\n#.....\n#.....\n######\n" => 'L',
        "#....#\n##...#\n##...#\n#.#..#\n#.#..#\n#..#.#\n#..#.#\n#...##\n#...##\n#....#\n" => 'N',
        "#####.\n#....#\n#....#\n#....#\n#####.\n#.....\n#.....\n#.....\n#.....\n#.....\n" => 'P',
        "#####.\n#....#\n#....#\n#....#\n#####.\n#..#..\n#...#.\n#...#.\n#....#\n#....#\n" => 'R',
        "#....#\n#....#\n.#..#.\n.#..#.\n..##..\n..##..\n.#..#.\n.#..#.\n#....#\n#....#\n" => 'X',
        "######\n.....#\n.....#\n....#.\n...#..\n..#...\n.#....\n#.....\n#.....\n######\n" => 'Z',
        _ => {
            return Err(format!("Unrecognized char: {input}"));
        }
    })
}

fn identify_chars(input: &str) -> Result<String, String> {
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    let mut result = String::new();
    while i + 5 < lines[0].len() {
        let mut this_char_input = String::new();
        for line in lines.iter() {
            this_char_input.push_str(&line[i..i + 6]);
            this_char_input.push('\n');
        }
        result.push(identify_char(&this_char_input)?);
        i += 8;
    }

    Ok(result)
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let input = include_str!("day10_input.txt");
    test_part_one!(input => "HKJFAKAF".to_string());
    test_part_two!(input => "10888".to_string());
}
