use crate::input::Input;
use std::cmp;
use std::ops;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct LineSegment {
    top_left: Vector,
    length: i32,
    horizontal: bool,
    start_step: u32,
    /// If stepping into this top left point (otherwise stepping away).
    incoming_direction: bool,
}

struct Intersection {
    point: Vector,
    combined_steps: u32,
}

impl LineSegment {
    const fn end_point(self) -> Vector {
        if self.horizontal {
            Vector {
                x: self.top_left.x + self.length,
                y: self.top_left.y,
            }
        } else {
            Vector {
                x: self.top_left.x,
                y: self.top_left.y + self.length,
            }
        }
    }

    fn intersection_with(self, other: Self) -> Option<Intersection> {
        if self.horizontal == other.horizontal {
            None
        } else {
            let (horizontal, vertical) = if self.horizontal {
                (self, other)
            } else {
                (other, self)
            };

            // To check if two line segments intersects:
            //
            // [a..b]
            //
            // [c
            //  .
            //  .
            //  d]
            if (vertical.top_left.y..=vertical.end_point().y).contains(&horizontal.top_left.y)
                && (horizontal.top_left.x..=horizontal.end_point().x).contains(&vertical.top_left.x)
            {
                let intersection_point = Vector {
                    x: vertical.top_left.x,
                    y: horizontal.top_left.y,
                };

                Some(Intersection {
                    point: intersection_point,
                    combined_steps: self.steps_at(intersection_point)
                        + other.steps_at(intersection_point),
                })
            } else {
                None
            }
        }
    }

    /// Assumes that point is on line.
    const fn steps_at(self, point: Vector) -> u32 {
        let self_steps_away = self.top_left.distance_from(point);
        if self.incoming_direction {
            self.start_step - self_steps_away
        } else {
            self.start_step + self_steps_away
        }
    }
}

impl Vector {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn direction(specifier: char) -> Result<Self, String> {
        Ok(match specifier {
            'U' => Self::new(0, -1),
            'R' => Self::new(1, 0),
            'D' => Self::new(0, 1),
            'L' => Self::new(-1, 0),
            _ => {
                return Err(format!("Invalid direction: {}", specifier));
            }
        })
    }

    const fn distance_from(self, other: Self) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
    }

    const fn multiply(self, factor: u32) -> Self {
        Self {
            x: self.x * (factor as i32),
            y: self.y * (factor as i32),
        }
    }
}

impl ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn parse_wire_points<'a>(
    string: &'a str,
) -> impl Iterator<Item = Result<LineSegment, String>> + Clone + 'a {
    let initial_position = Vector::new(0, 0);
    let initial_step = 0_u32;

    string.split(',').scan(
        (initial_position, initial_step),
        |(current_position, current_step), word| {
            if let (Some(first_char), Some(Ok(steps))) =
                (word.chars().next(), word.get(1..).map(|n| n.parse::<u32>()))
            {
                let start_position = *current_position;

                let direction = match Vector::direction(first_char as char) {
                    Ok(direction) => direction,
                    Err(description) => {
                        return Some(Err(description));
                    }
                };
                *current_position += direction.multiply(steps);

                let top_left = Vector {
                    x: std::cmp::min(start_position.x, current_position.x),
                    y: std::cmp::min(start_position.y, current_position.y),
                };

                let incoming_direction = top_left != start_position;

                let line_segment = LineSegment {
                    top_left,
                    length: steps as i32,
                    horizontal: direction.x.abs() != 0,
                    start_step: if incoming_direction {
                        *current_step + steps
                    } else {
                        *current_step
                    },
                    incoming_direction,
                };

                *current_step += steps;

                Some(Ok(line_segment))
            } else {
                Some(Err(
                    "Invalid word - not 'U', 'R', 'D' or 'L' followed by an integer".to_string(),
                ))
            }
        },
    )
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

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let (first_line, second_line) = input_lines(&input.text)?;
    let first_wire_segments: Vec<LineSegment> =
        parse_wire_points(first_line).collect::<Result<_, _>>()?;

    #[cfg(feature = "visualization")]
    {
        use crate::painter::PainterRef;
        let mut min_x = std::i32::MAX;
        let mut max_x = std::i32::MIN;
        let mut min_y = std::i32::MAX;
        let mut max_y = std::i32::MIN;

        let second_wire_segments: Vec<LineSegment> =
            parse_wire_points(second_line).collect::<Result<_, _>>()?;

        for line_segment in first_wire_segments
            .iter()
            .chain(second_wire_segments.iter())
        {
            min_x = std::cmp::min(min_x, line_segment.top_left.x);
            max_x = std::cmp::max(max_x, line_segment.end_point().x);
            min_y = std::cmp::min(min_y, line_segment.top_left.y);
            max_y = std::cmp::max(max_y, line_segment.end_point().y);
        }

        let grid_width = (max_x - min_x) as i32;
        let grid_height = (max_y - min_y) as i32;
        input.painter.set_aspect_ratio(grid_width, grid_height);

        let grid_display_width = 1.0 / grid_width as f64;
        let grid_display_height = (1.0 / grid_height as f64) / input.painter.aspect_ratio();

        let mut drawn_lines1: Vec<LineSegment> = Vec::new();
        let mut drawn_lines2: Vec<LineSegment> = Vec::new();

        let draw_line = |painter: &mut PainterRef, line_segment: &LineSegment, r, g, b| {
            let start_x = (line_segment.top_left.x - min_x) as f64 * grid_display_width;
            let start_y = (line_segment.top_left.y - min_y) as f64 * grid_display_height;
            let mut end_x = start_x;
            let mut end_y = start_y;
            if line_segment.horizontal {
                end_x += line_segment.length as f64 * grid_display_width
            } else {
                end_y += line_segment.length as f64 * grid_display_height
            }

            painter.line_width(grid_display_width * 30.);
            painter.stroke_style_rgb(r, g, b);
            painter.begin_path();
            painter.move_to(start_x, start_y);
            painter.line_to(end_x, end_y);
            painter.stroke();
        };

        let mut first = true;
        for (l1, l2) in first_wire_segments.iter().zip(second_wire_segments.iter()) {
            draw_line(&mut input.painter, &l1, 255, 0, 0);
            draw_line(&mut input.painter, &l2, 0, 255, 0);

            drawn_lines1.push(*l1);
            drawn_lines2.push(*l2);

            let mut intersection_count = 0;
            for d1 in &drawn_lines1 {
                for d2 in &drawn_lines2 {
                    if let Some(intersection) = d1.intersection_with(*d2) {
                        intersection_count += 1;
                        input.painter.line_width(grid_display_width * 10.);
                        input.painter.stroke_style_rgb(255, 255, 255);
                        input.painter.stroke_circle(
                            (intersection.point.x - min_x) as f64 * grid_display_width,
                            (intersection.point.y - min_y) as f64 * grid_display_height,
                            grid_display_width * 100.,
                        );
                    }
                }
            }

            input.painter.status_text(&format!(
                "Line segments: {} Intersections: {}",
                drawn_lines1.len(),
                intersection_count
            ));

            if first {
                first = false;
                input.painter.end_frame();
            } else {
                input.painter.meta_delay(100);
            }
        }
    }

    let mut best = std::u32::MAX;
    let origin = Vector { x: 0, y: 0 };

    for line_segment in parse_wire_points(second_line) {
        let line_segment = line_segment?;
        for first_line_segment in &first_wire_segments {
            if let Some(intersection) = first_line_segment.intersection_with(line_segment) {
                // "While the wires do technically cross right at the central port
                // where they both start, this point does not count":
                if intersection.point != origin {
                    let intersection_value = if input.is_part_one() {
                        intersection.point.distance_from(origin)
                    } else {
                        intersection.combined_steps
                    };
                    best = cmp::min(best, intersection_value);
                }
            }
        }
    }

    Ok(best)
}

#[test]
pub fn tests_line_segment() {
    let l1 = LineSegment {
        top_left: Vector { x: 0, y: 0 },
        length: 10,
        horizontal: true,
        start_step: 99,
        incoming_direction: false,
    };
    let l2 = LineSegment {
        top_left: Vector { x: 3, y: -4 },
        length: 5,
        horizontal: false,
        start_step: 10,
        incoming_direction: false,
    };
    if let Some(intersection) = l1.intersection_with(l2) {
        assert_eq!(Vector { x: 3, y: 0 }, intersection.point);
        assert_eq!(99 + 3 + 10 + 4, intersection.combined_steps);
    } else {
        panic!("Incorrect");
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!("R8,U5,L5,D3\nU7,R6,D4,L4" => 6);
    test_part_one!("R8,U5,L5,D3\nU7,R6,D4,L4" => 6);
    test_part_one!("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83" => 159);
    test_part_one!(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7" => 135);

    test_part_two!("R8,U5,L5,D3\nU7,R6,D4,L4" => 30);
    test_part_two!("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83" => 610);
    test_part_two!("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7" => 410);

    let real_input = include_str!("day03_input.txt");
    test_part_one!(real_input => 375);
    test_part_two!(real_input => 14746);
}
