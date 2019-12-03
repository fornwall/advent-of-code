use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
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

fn parse_wire_points(string: &str) -> impl Iterator<Item = (Point, u32)> + '_ {
    fn parse_step(step: &str) -> impl Iterator<Item = Point> {
        let mut chars = step.chars();
        let direction = Point::direction(chars.next().unwrap());
        let length = chars.collect::<String>().parse().unwrap();
        std::iter::repeat(direction).take(length)
    }

    let mut current_position = Point::new(0, 0);
    let mut current_step: u32 = 0;

    string
        .split(',')
        .flat_map(parse_step)
        .map(move |direction| {
            current_position += direction;
            current_step += 1;
            (current_position, current_step)
        })
}

pub fn part1(input_string: &str) -> String {
    let mut lines = input_string.lines();

    let first_wire_points: HashSet<Point> =
        HashSet::from_iter(parse_wire_points(lines.next().unwrap()).map(|(point, _)| point));

    parse_wire_points(lines.next().unwrap())
        .filter_map(|(point, _)| {
            if first_wire_points.contains(&point) {
                Some(point.distance_from_origin())
            } else {
                None
            }
        })
        .min()
        .unwrap()
        .to_string()
}

pub fn part2(input_string: &str) -> String {
    let mut lines = input_string.lines();

    let mut first_wire_points = HashMap::new();
    parse_wire_points(lines.next().unwrap()).for_each(|(point, step)| {
        first_wire_points.entry(point).or_insert(step);
    });

    parse_wire_points(lines.next().unwrap())
        .filter_map(|(point, step)| {
            if let Some(&value) = first_wire_points.get(&point) {
                Some(step + value)
            } else {
                None
            }
        })
        .min()
        .unwrap()
        .to_string()
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

    assert_eq!(part1(include_str!("../src/day03_input.txt")), "375");
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

    assert_eq!(part2(include_str!("../src/day03_input.txt")), "14746");
}
