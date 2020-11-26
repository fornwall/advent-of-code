use super::int_code::Word;
use crate::Painter;
use std::collections::HashMap;

type PainterRef = Box<dyn Painter>;

pub struct Renderer<'a> {
    tiles: HashMap<(Word, Word), Word>,
    painter: &'a mut PainterRef,
    aspect_ratio_sent: bool,
}

impl<'a> Renderer<'a> {
    pub fn new(painter: &'a mut PainterRef) -> Self {
        Self {
            tiles: HashMap::new(),
            painter,
            aspect_ratio_sent: false,
        }
    }

    pub fn add_tile(&mut self, location: (Word, Word), value: Word) {
        self.tiles.insert(location, value);
    }

    pub fn render(&mut self, current_score: Word) {
        self.painter.clear();

        let mut min_x = Word::MAX;
        let mut max_x = Word::MIN;
        let mut min_y = Word::MAX;
        let mut max_y = Word::MIN;
        for &(x, y) in self.tiles.keys() {
            min_x = std::cmp::min(min_x, x);
            max_x = std::cmp::max(max_x, x);
            min_y = std::cmp::min(min_y, y);
            max_y = std::cmp::max(max_y, y);
        }

        let grid_width = (max_x - min_x + 1) as i32;
        let grid_height = (max_y - min_y + 1) as i32;

        if !self.aspect_ratio_sent {
            self.aspect_ratio_sent = true;
            self.painter.set_aspect_ratio(grid_width, grid_height);
        }

        let grid_display_width = 1.0 / grid_width as f64;
        let grid_display_height = (1.0 / grid_height as f64) / self.painter.aspect_ratio();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let draw_rect = |r, g, b, painter: &mut PainterRef| {
                    let draw_x = (x - min_x) as f64 * grid_display_width;
                    let draw_y = (y - min_y) as f64 * grid_display_height;
                    painter.fill_style_rgb(r, g, b);
                    if r == 255 && g == 255 {
                        painter.begin_path();
                        painter.arc(
                            draw_x + grid_display_width / 2.,
                            draw_y + grid_display_width / 2.,
                            grid_display_width / 3.,
                            0.,
                            std::f64::consts::PI * 2.,
                        );
                        painter.fill();
                    } else {
                        painter.fill_rect(
                            draw_x,
                            draw_y,
                            grid_display_width * 0.95,
                            grid_display_height * 0.95,
                        );
                    }
                };
                match self.tiles.get(&(x, y)) {
                    Some(1) => {
                        // Wall.
                        draw_rect(255, 0, 0, &mut self.painter);
                    }
                    Some(2) => {
                        let r = (y * 11) % 256;
                        let g = 255 - (y * 3) % 256;
                        let b = 255 - (y * 9) % 256;
                        // Thing to blow up.
                        draw_rect(r as i32, g as i32, b as i32, &mut self.painter);
                    }
                    Some(3) => {
                        // Paddle.
                        self.painter.shadow_color(255, 0, 255);
                        self.painter.shadow_blur(85);
                        draw_rect(0, 0, 255, &mut self.painter);
                        self.painter.shadow_blur(0);
                    }
                    Some(4) => {
                        // Ball.
                        draw_rect(255, 255, 255, &mut self.painter);
                    }
                    _ => {}
                };
            }
        }
        self.painter.end_frame();
    }
}
