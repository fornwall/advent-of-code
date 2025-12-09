use crate::{
    common::array_stack::ArrayStack,
    input::{Input, on_error},
};

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAX_POINTS: usize = 1000;
    let mut points = ArrayStack::<MAX_POINTS, Point>::new();
    let mut boundary = ArrayStack::<MAX_POINTS, Line>::new();
    let mut last_point = None;

    for line in input.text.lines() {
        let mut parts = line.split(',');
        let x = parts
            .next()
            .ok_or_else(on_error)?
            .parse::<u32>()
            .map_err(|_| on_error())?;
        let y = parts
            .next()
            .ok_or_else(on_error)?
            .parse::<u32>()
            .map_err(|_| on_error())?;
        let point = Point { x, y };
        points.push(point)?;
        if let Some(last_point) = last_point {
            boundary.push(Line {
                start: last_point,
                end: point,
            })?;
        }
        last_point = Some(point);
    }
    if points.len() < 4 {
        return Err("Not enough points to form a rectangle".to_string());
    }
    // "The list wraps, so the first red tile is also connected to the last red tile":
    boundary.push(Line {
        start: points.elements[points.len() - 1],
        end: points.elements[0],
    })?;

    if input.is_part_one() {
        points
            .slice()
            .iter()
            .enumerate()
            .flat_map(|(lower_idx, lower_point)| {
                points
                    .slice()
                    .iter()
                    .skip(lower_idx + 1)
                    .map(|higher_point| lower_point.rectangle_size(higher_point))
            })
            .max()
            .ok_or_else(|| "No rectangle found".to_string())
    } else {
        let mut areas = Vec::new();
        for (lower_idx, lower_point) in points.slice().iter().enumerate() {
            areas.extend(
                points
                    .slice()
                    .iter()
                    .skip(lower_idx + 1)
                    .map(|higher_point| {
                        (
                            lower_point.rectangle_size(higher_point),
                            Rectangle::from_exclusive(*lower_point, *higher_point),
                        )
                    }),
            );
        }
        areas.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        areas
            .iter()
            .find_map(|(area, rect)| rect.any_line_touches(boundary.slice()).then_some(*area))
            .ok_or_else(|| "No rectangle found inside perimeter".to_string())
    }
}

#[derive(Clone, Copy, Default)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn rectangle_size(&self, opposing: &Self) -> u64 {
        (u64::from(self.x).abs_diff(u64::from(opposing.x)) + 1)
            * (u64::from(self.y).abs_diff(u64::from(opposing.y)) + 1)
    }
}

#[derive(Clone, Copy, Default)]
struct Line {
    #[allow(dead_code)]
    start: Point,
    #[allow(dead_code)]
    end: Point,
}

impl Line {
    // Note: Inclusive ranges.
    fn is_inside_rect(&self, rect: &Rectangle) -> bool {
        if self.start.x == self.end.x {
            // vertical line
            if !(rect.upper_left.x..=rect.lower_right.x).contains(&self.start.x) {
                return false;
            }
            let line_low_y = self.start.y.min(self.end.y) + 1;
            let line_high_y = self.start.y.max(self.end.y);
            line_low_y <= rect.lower_right.y && line_high_y >= rect.upper_left.y
        } else {
            // horizontal line
            if !(rect.upper_left.y..=rect.lower_right.y).contains(&self.start.y) {
                return false;
            }
            let line_low_x = self.start.x.min(self.end.x) + 1;
            let line_high_x = self.start.x.max(self.end.x);
            line_low_x <= rect.lower_right.x && line_high_x >= rect.upper_left.x
        }
    }
}

struct Rectangle {
    upper_left: Point,
    lower_right: Point,
}

impl Rectangle {
    fn from_exclusive(p1: Point, p2: Point) -> Self {
        Self {
            upper_left: Point {
                x: p1.x.min(p2.x) + 1,
                y: p1.y.min(p2.y) + 1,
            },
            lower_right: Point {
                x: p1.x.max(p2.x) - 1,
                y: p1.y.max(p2.y) - 1,
            },
        }
    }
    fn any_line_touches(&self, lines: &[Line]) -> bool {
        !lines.iter().any(|line| line.is_inside_rect(self))
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    test_part_one_no_allocations!(test_input => 50);
    test_part_two_no_allocations!(test_input => 24);

    let real_input = include_str!("day09_input.txt");
    test_part_one_no_allocations!(real_input => 4_763_040_296);
    test_part_two_no_allocations!(real_input => 1_396_494_456);
}
