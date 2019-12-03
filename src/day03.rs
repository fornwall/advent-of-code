use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn direction(specifier: char) -> Point {
        match specifier {
            'U' => Point::new(0, 1),
            'R' => Point::new(1, 0),
            'D' => Point::new(0, -1),
            'L' => Point::new(-1, 0),
            _ => {
                panic!("Invalid direction specifier: {}", specifier);
            }
        }
    }

    fn distance_from_origin(self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
}

impl ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn parse_wire_points<F>(string: &str, mut on_visit: F)
where
    F: FnMut(Point, u32),
{
    let mut current_position = Point::new(0, 0);
    let mut current_step: u32 = 0;

    for word in string.split(',') {
        let direction = Point::direction(word.chars().next().unwrap());
        let steps = word[1..].parse::<i32>().unwrap();

        for _i in 0..steps {
            current_step += 1;
            current_position += direction;
            on_visit(current_position, current_step);
        }
    }
}

pub fn part1(input_string: &str) -> String {
    let lines: Vec<&str> = input_string.lines().collect();
    let mut first_wire_points = HashSet::new();

    parse_wire_points(lines[0], |point, _| {
        first_wire_points.insert(point);
    });

    let mut best_distance = std::u32::MAX;
    parse_wire_points(lines[1], |point, _| {
        if first_wire_points.contains(&point) {
            best_distance = cmp::min(best_distance, point.distance_from_origin());
        }
    });

    best_distance.to_string()
}

pub fn part2(input_string: &str) -> String {
    let lines: Vec<&str> = input_string.lines().collect();
    let mut first_wire_points = HashMap::new();

    parse_wire_points(lines[0], |point, step| {
        first_wire_points.insert(point, step);
    });

    let mut best_steps = std::u32::MAX;
    parse_wire_points(lines[1], |point, step| {
        if let Some(&value) = first_wire_points.get(&point) {
            best_steps = cmp::min(best_steps, step + value);
        }
    });

    best_steps.to_string()
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1("R8,U5,L5,D3\nU7,R6,D4,L4"), "6");
    assert_eq!(
        part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
        "159"
    );
    assert_eq!(
        part1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        "135"
    );

    assert_eq!(part1(include_str!("day03_input.txt")), "375");
}

#[test]
fn tests_part2() {
    assert_eq!(part2("R8,U5,L5,D3\nU7,R6,D4,L4"), "30");
    assert_eq!(
        part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
        "610"
    );
    assert_eq!(
        part2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        "410"
    );

    assert_eq!(part2(include_str!("day03_input.txt")), "14746");
}
