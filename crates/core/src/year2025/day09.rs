use std::collections::BTreeMap;

use crate::{
    common::array_stack::ArrayStack,
    input::{Input, on_error},
};

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAX_POINTS: usize = 1000;
    let mut points = ArrayStack::<MAX_POINTS, Point>::new();

    for line in input.text.lines() {
        let (x_str, y_str) = line.split_once(',').ok_or_else(on_error)?;
        let x = x_str.parse::<u32>().map_err(|_| on_error())?;
        let y = y_str.parse::<u32>().map_err(|_| on_error())?;
        points.push(Point { x, y })?;
    }

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
                    .map(|&higher_point| lower_point.rectangle_size(higher_point))
            })
            .max()
            .ok_or_else(|| "No rectangle found".to_string())
    } else {
        //let mut boundary = ArrayStack::<MAX_POINTS, Line>::new();
        let mut horizontal_lines_by_y = BTreeMap::new();
        let mut vertical_lines_by_x = BTreeMap::new();
        for i in 0..points.len() {
            let p1 = points.elements[i];
            let p2 = points.elements[(i + 1) % points.len()];
            if p1.x == p2.x {
                vertical_lines_by_x
                    .entry(p1.x)
                    .or_insert_with(Vec::new)
                    .push(Line { start: p1, end: p2 });
            } else {
                horizontal_lines_by_y
                    .entry(p1.y)
                    .or_insert_with(Vec::new)
                    .push(Line { start: p1, end: p2 });
            }
        }

        let mut highest_area = 0;
        for (lower_idx, &lower_point) in points.slice().iter().enumerate() {
            'higher: for &higher_point in points.slice().iter().skip(lower_idx + 1) {
                let this_area = lower_point.rectangle_size(higher_point);
                if this_area > highest_area {
                    let rect = Rectangle::from_exclusive(lower_point, higher_point);
                    if rect.upper_left.y > rect.lower_right.y {
                        continue 'higher;
                    }
                    for (_, lines) in
                        horizontal_lines_by_y.range(rect.upper_left.y..=rect.lower_right.y)
                    {
                        for line in lines {
                            if line.is_inside_rect(&rect) {
                                continue 'higher;
                            }
                        }
                    }
                    if rect.upper_left.x > rect.lower_right.x {
                        continue 'higher;
                    }
                    for (_, lines) in
                        vertical_lines_by_x.range(rect.upper_left.x..=rect.lower_right.x)
                    {
                        for line in lines {
                            if line.is_inside_rect(&rect) {
                                //println!("  blocked by line {:?}", line);
                                continue 'higher;
                            }
                        }
                    }
                    highest_area = this_area;
                }
            }
        }
        Ok(highest_area)
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn rectangle_size(&self, opposing: Self) -> u64 {
        (u64::from(self.x).abs_diff(u64::from(opposing.x)) + 1)
            * (u64::from(self.y).abs_diff(u64::from(opposing.y)) + 1)
    }
}

#[derive(Clone, Copy, Default, Debug)]
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

#[derive(Debug)]
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
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    test_part_one!(test_input => 50);
    test_part_two!(test_input => 24);

    let real_input = include_str!("day09_input.txt");
    test_part_one!(real_input => 4_763_040_296);
    test_part_two!(real_input => 1_396_494_456);
}
