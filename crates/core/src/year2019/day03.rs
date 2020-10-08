use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn direction(specifier: char) -> Result<Self, String> {
        Ok(match specifier {
            'U' => Self::new(0, 1),
            'R' => Self::new(1, 0),
            'D' => Self::new(0, -1),
            'L' => Self::new(-1, 0),
            _ => {
                return Err(format!("Invalid direction: {}", specifier));
            }
        })
    }

    const fn distance_from_origin(self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
}

impl ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn parse_wire_points<F>(string: &str, mut on_visit: F) -> Result<(), String>
where
    F: FnMut(Point, u32),
{
    let mut current_position = Point::new(0, 0);
    let mut current_step: u32 = 0;

    for word in string.split(',') {
        let first_char = word
            .chars()
            .next()
            .ok_or("Invalid input - too small word")?;
        let direction = Point::direction(first_char)?;
        let steps = word[1..].parse::<i32>().map_err(|error| {
            format!(
                "Invalid input - could not parse steps: {}",
                error.to_string()
            )
        })?;

        for _ in 0..steps {
            current_step += 1;
            current_position += direction;
            on_visit(current_position, current_step);
        }
    }
    Ok(())
}

fn input_lines(input_string: &str) -> Result<(&str, &str), String> {
    let lines: Vec<&str> = input_string.lines().collect();
    if lines.len() != 2 {
        return Err(format!(
            "Invalid number of input lines - expected 2, was {}",
            lines.len(),
        ));
    }
    Ok((lines[0], lines[1]))
}

pub fn part1(input_string: &str) -> Result<u32, String> {
    let (first_line, second_line) = input_lines(input_string)?;
    let mut first_wire_points = HashSet::new();

    parse_wire_points(first_line, |point, _| {
        first_wire_points.insert(point);
    })?;

    let mut closest_distance = std::u32::MAX;
    parse_wire_points(second_line, |point, _| {
        if first_wire_points.contains(&point) {
            closest_distance = cmp::min(closest_distance, point.distance_from_origin());
        }
    })?;

    Ok(closest_distance)
}

pub fn part2(input_string: &str) -> Result<u32, String> {
    let (first_line, second_line) = input_lines(input_string)?;
    let mut first_wire_points = HashMap::new();

    parse_wire_points(first_line, |point, step| {
        first_wire_points.entry(point).or_insert(step);
    })?;

    let mut fewest_steps = std::u32::MAX;
    parse_wire_points(second_line, |point, step| {
        if let Some(&value) = first_wire_points.get(&point) {
            fewest_steps = cmp::min(fewest_steps, step + value);
        }
    })?;

    Ok(fewest_steps)
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1("R8,U5,L5,D3\nU7,R6,D4,L4"), Ok(6));
    assert_eq!(
        part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
        Ok(159)
    );
    assert_eq!(
        part1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        Ok(135)
    );

    assert_eq!(part1(include_str!("day03_input.txt")), Ok(375));
}

#[test]
fn tests_part2() {
    assert_eq!(part2("R8,U5,L5,D3\nU7,R6,D4,L4"), Ok(30));
    assert_eq!(
        part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
        Ok(610)
    );
    assert_eq!(
        part2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        Ok(410)
    );

    assert_eq!(part2(include_str!("day03_input.txt")), Ok(14746));
}
