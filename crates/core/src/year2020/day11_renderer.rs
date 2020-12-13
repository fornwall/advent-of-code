use crate::painter::PainterRef;
use std::collections::HashMap;

pub struct Renderer<'a> {
    painter: &'a mut PainterRef,
    idx_to_coordinates: HashMap<usize, (i32, i32)>,
    cols: i32,
    rows: i32,
}

impl<'a> Renderer<'a> {
    pub fn new(painter: &'a mut PainterRef, cols: i32, rows: i32) -> Self {
        Self {
            painter,
            idx_to_coordinates: HashMap::new(),
            cols,
            rows,
        }
    }

    pub fn add_idx_mapping(&mut self, idx: usize, x: i32, y: i32) {
        self.idx_to_coordinates.insert(idx, (x, y));
    }

    pub fn render(&mut self, iteration: u32, seats: &[bool]) {
        self.painter.status_text(&format!(
            "Iteration: {: >3}   Occupied: {: >4}",
            iteration,
            seats.iter().filter(|&&b| b).count()
        ));

        let grid_width = (self.cols) as i32;
        let grid_height = (self.rows) as i32;
        let grid_display_width = 1.0 / grid_width as f64;
        let grid_display_height = (1.0 / grid_height as f64) / self.painter.aspect_ratio();

        self.painter.clear();
        for (idx, &occupied) in seats.iter().enumerate() {
            let &(x, y) = self.idx_to_coordinates.get(&idx).unwrap();
            let draw_x = x as f64 * grid_display_width;
            let draw_y = y as f64 * grid_display_height;
            if occupied {
                self.painter.fill_style_rgb_packed(0xFF0000);
            } else {
                self.painter.fill_style_rgb_packed(0x444444);
            }
            self.painter.fill_square(draw_x, draw_y, grid_display_width);
        }

        self.painter.meta_delay(100);
    }
}
