use std::collections::VecDeque;

#[cfg(feature = "visualization")]
use svgplot::{Coordinate, SvgColor, SvgImage, SvgPath, SvgScript, SvgShape, SvgStrokeLinecap};

use crate::input::Input;

type Position = (usize, usize);

struct Graph {
    cells: Vec<u8>,
    visited: Vec<bool>,
    height: usize,
    width: usize,
}

impl Graph {
    fn parse(input: &str) -> Result<(Position, Position, Self), String> {
        let height = input.bytes().filter(|b| *b == b'\n').count() + 1;
        let width = input.lines().next().unwrap_or_default().len();
        let mut start_pos = (0, 0);
        let mut destination_pos = (0, 0);
        let mut cells = vec![0_u8; height * width];
        for (y, line) in input.lines().enumerate() {
            if line.len() != width {
                return Err("Not all rows have equal length".to_string());
            }
            for (x, val) in line.bytes().enumerate() {
                cells[y * width + x] = if val == b'S' {
                    // "Your current position (S) has elevation a"
                    start_pos = (x, y);
                    b'a'
                } else if val == b'E' {
                    // "the location that should get the best signal (E) has elevation z"
                    destination_pos = (x, y);
                    b'z'
                } else {
                    if !val.is_ascii_lowercase() {
                        return Err("Strange character in input".to_string());
                    }
                    val
                } - b'a';
            }
        }
        let visited = vec![false; cells.len()];
        Ok((
            start_pos,
            destination_pos,
            Self {
                cells,
                visited,
                height,
                width,
            },
        ))
    }

    fn height_at(&mut self, x: usize, y: usize) -> u8 {
        self.cells[y * self.width + x]
    }

    fn mark_visited(&mut self, x: usize, y: usize) -> bool {
        let old = self.visited[y * self.width + x];
        self.visited[y * self.width + x] = true;
        !old
    }

    fn can_go(&mut self, x: usize, y: usize, dx: i32, dy: i32) -> Option<Position> {
        if (dx < 0 && x == 0)
            || (dy < 0 && y == 0)
            || (dx > 0 && x + 1 == self.width)
            || (dy > 0 && y + 1 == self.height)
        {
            return None;
        }
        let new_x = (x as i32 + dx) as usize;
        let new_y = (y as i32 + dy) as usize;
        let from_height = self.height_at(new_x, new_y);
        let to_height = self.height_at(x, y);
        (to_height <= from_height + 1 && self.mark_visited(new_x, new_y)).then_some((new_x, new_y))
    }
}

pub fn solve(input: &Input) -> Result<u32, String> {
    let (start_pos, destination_pos, mut graph) = Graph::parse(input.text)?;

    #[cfg(feature = "visualization")]
    let mut svg = SvgImage::new().view_box((0, 0, graph.width as i64, graph.height as i64));
    #[cfg(feature = "visualization")]
    let mut current_render_step = 0;
    #[cfg(feature = "visualization")]
    let mut circles_render_script = String::from("const circlesPerStep = ['");
    #[cfg(feature = "visualization")]
    let mut path_render_script = String::from("const pathsPerStep = ['");

    #[cfg(feature = "visualization")]
    {
        for draw_height in 0..26 {
            let mut shape = SvgShape::new();
            let hue = 225. - (draw_height as f64) * 10.;
            for x in 0..graph.width {
                for y in 0..graph.height {
                    let height = graph.height_at(x, y);
                    if height == draw_height {
                        shape = shape
                            .move_to_absolute(x as i32, y as i32)
                            .line_to_relative(1, 0)
                            .line_to_relative(0, 1)
                            .line_to_relative(-1, 0)
                            .close();
                    }
                }
            }
            if !shape.is_empty() {
                svg.add(
                    SvgPath {
                        shape,
                        ..Default::default()
                    }
                    .title(format!("Elevation: {}", draw_height))
                    .fill(SvgColor::Hsl(hue, 70, 40)),
                );
            }
        }

        if input.is_part_one() {
            svg.add(
                svgplot::SvgRect::default()
                    .x(start_pos.0 as Coordinate)
                    .y(start_pos.1 as Coordinate)
                    .width(1)
                    .height(1)
                    .fill(SvgColor::Rgb(255, 255, 255))
                    .title("Starting position - elevation 0".to_string()),
            );
        }
        svg.add(
            svgplot::SvgRect::default()
                .x(destination_pos.0 as Coordinate)
                .y(destination_pos.1 as Coordinate)
                .width(1)
                .height(1)
                .fill(SvgColor::Rgb(255, 255, 255))
                .title(format!(
                    "Destination - elevation {}",
                    graph.height_at(destination_pos.0, destination_pos.1)
                )),
        );
    }

    let mut to_visit = VecDeque::with_capacity(64);
    graph.mark_visited(destination_pos.0, destination_pos.1);
    to_visit.push_back((0, destination_pos));

    while let Some((cost, pos)) = to_visit.pop_front() {
        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if let Some(new_pos) = graph.can_go(pos.0, pos.1, dx, dy) {
                let new_cost = cost + 1;
                let at_goal = if input.is_part_one() {
                    new_pos == start_pos
                } else {
                    graph.height_at(new_pos.0, new_pos.1) == 0
                };

                #[cfg(feature = "visualization")]
                {
                    if new_cost != current_render_step {
                        path_render_script.push_str("', '");
                        circles_render_script.push_str("', '");
                        current_render_step = new_cost;
                    }
                    let circle_radius = 0.4;
                    circles_render_script.push_str(
                        &SvgShape::new()
                            .circle_absolute(
                                new_pos.0 as f64 + 0.5,
                                new_pos.1 as f64 + 0.5,
                                circle_radius,
                            )
                            .data_string(),
                    );
                    path_render_script.push_str(
                        &SvgShape::at(new_pos.0 as f64 + 0.5, new_pos.1 as f64 + 0.5)
                            .line_to_relative(-dx as f64, -dy as f64)
                            .data_string(),
                    );
                }

                if at_goal {
                    #[cfg(feature = "visualization")]
                    {
                        let visited_path_id = svg.add_with_id(
                            SvgPath::default()
                                .stroke(SvgColor::Rgb(255, 255, 255))
                                .stroke_width(0.2)
                                .stroke_linecap(SvgStrokeLinecap::Round),
                        );
                        let circles_path_id =
                            svg.add_with_id(SvgPath::default().fill(SvgColor::Rgb(255, 255, 255)));

                        circles_render_script.push_str("'];");
                        path_render_script.push_str(&format!("'];\n window.onNewStep = (step) => {{\n\
                                                              document.getElementById('{}').setAttribute('d', circlesPerStep[step]);\n\
                                                              const pathData = pathsPerStep.slice(0, step+1).join('');\n\
                                                              document.getElementById('{}').setAttribute('d', pathData);\n\
                                                             }}", circles_path_id, visited_path_id));
                        svg.add(SvgScript::new(format!(
                            "{}{}",
                            circles_render_script, path_render_script
                        )));
                        input.rendered_svg.replace(
                            svg.data_attribute("steps".to_string(), format!("{}", new_cost))
                                .data_attribute("step-duration".to_string(), format!("{}", 100))
                                .to_svg_string(),
                        );
                    }
                    return Ok(new_cost);
                }
                to_visit.push_back((new_cost, new_pos));
            }
        }
    }
    Err("No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
    test_part_one!(test_input => 31);
    test_part_two!(test_input => 29);

    let real_input = include_str!("day12_input.txt");
    test_part_one!(real_input => 528);
    test_part_two!(real_input => 522);
}
