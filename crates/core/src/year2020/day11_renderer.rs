use super::day11::Grid;
use crate::painter::PainterRef;
use std::collections::HashMap;

pub struct Renderer<'a> {
    painter: &'a mut PainterRef,
    aspect_ratio_sent: bool,
    iteration: u32,
    idx_to_coordinates: HashMap<usize, (i32, i32)>,
}

impl<'a> Renderer<'a> {
    pub fn new(painter: &'a mut PainterRef) -> Self {
        Self {
            painter,
            aspect_ratio_sent: false,
            iteration: 0,
            idx_to_coordinates: HashMap::new(),
        }
    }

    pub fn add_idx_mapping(&mut self, idx: usize, x: i32, y: i32) {
        self.idx_to_coordinates.insert(idx, (x, y));
    }

    pub fn init(&mut self) {
        // TODO!
    }

    pub fn render(&mut self, grid: &Grid) {
        self.painter.clear();
        self.iteration += 1;
        self.painter.end_frame();
    }
}
