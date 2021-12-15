use super::day15::Graph;
use crate::painter::PainterRef;
use std::collections::HashSet;

pub struct Renderer<'a> {
    visited_positions: HashSet<(u16, u16)>,
    painter: &'a mut PainterRef,
    iteration: u32,
}

impl<'a> Renderer<'a> {
    pub fn new(painter: &'a mut PainterRef) -> Self {
        Self {
            visited_positions: HashSet::new(),
            painter,
            iteration: 0,
        }
    }

    pub fn render_initial(&mut self, graph: &Graph) {
        self.painter.clear();

        let min_x = 0;
        let max_x = graph.width;
        let min_y = 0;
        let max_y = graph.height;

        let grid_width = i32::from(max_x - min_x + 1);
        let grid_height = i32::from(max_y - min_y + 1);

        self.painter.set_aspect_ratio(grid_width, grid_height);

        let grid_display_width = 1.0 / f64::from(grid_width);
        let grid_display_height = (1.0 / f64::from(grid_height)) / self.painter.aspect_ratio();

        for y in min_y..max_y {
            for x in min_x..max_x {
                let draw_rect = |r, g, b, painter: &mut PainterRef| {
                    let draw_x = f64::from(x - min_x) * grid_display_width;
                    let draw_y = f64::from(y - min_y) * grid_display_height;
                    painter.fill_style_rgb(r, g, b);
                    painter.fill_rect(
                        draw_x,
                        draw_y,
                        grid_display_width * 0.95,
                        grid_display_height * 0.95,
                    );
                };

                let r = graph.risk_level_at(x as usize, y as usize) * 28;
                let g = 0;
                let b = 0;
                draw_rect(i32::from(r), g, b, self.painter);
            }
        }
        self.painter.end_frame();
    }

    pub fn render(&mut self, graph: &Graph, visited: (u16, u16), distance: u32) {
        self.visited_positions.insert(visited);
        self.iteration += 1;
        self.painter.status_text(&format!(
            "Nodes: {: >6}   Risk: {: >4}",
            self.visited_positions.len(),
            distance
        ));

        let min_x = 0;
        let max_x = graph.width;
        let min_y = 0;
        let max_y = graph.height;

        let grid_width = i32::from(max_x - min_x + 1);
        let grid_height = i32::from(max_y - min_y + 1);

        let grid_display_width = 1.0 / f64::from(grid_width);
        let grid_display_height = (1.0 / f64::from(grid_height)) / self.painter.aspect_ratio();

        let draw_rect = |x, y, r, g, b, painter: &mut PainterRef| {
            let draw_x = f64::from(x - min_x) * grid_display_width;
            let draw_y = f64::from(y - min_y) * grid_display_height;
            painter.fill_style_rgb(r, g, b);
            painter.fill_rect(
                draw_x,
                draw_y,
                grid_display_width * 0.95,
                grid_display_height * 0.95,
            );
        };

        let r = 100;
        let g = 255;
        let b = 100;
        draw_rect(
            visited.0,
            visited.1,
            r as i32,
            g as i32,
            b as i32,
            self.painter,
        );
        if self.iteration % 240 == 0 {
            self.painter.end_frame();
        }
    }
}
