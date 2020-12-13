use super::day12::{SHIP_DIRECTION_ENTITY_IDX, SHIP_POSITION_ENTITY_IDX, WAYPOINT_ENTITY_IDX};
use crate::painter::PainterRef;

pub struct Renderer<'a> {
    painter: &'a mut PainterRef,
    entities_over_time: Vec<[(i32, i32); 3]>,
}

impl<'a> Renderer<'a> {
    pub fn new(painter: &'a mut PainterRef) -> Self {
        Self {
            painter,
            entities_over_time: Vec::new(),
        }
    }

    pub fn save_entities(&mut self, entities: [(i32, i32); 3]) {
        self.entities_over_time.push(entities);
    }

    pub fn render(&mut self, part_one: bool) {
        let mut min_x = std::i32::MAX;
        let mut max_x = std::i32::MIN;
        let mut min_y = std::i32::MAX;
        let mut max_y = std::i32::MIN;
        for entities in self.entities_over_time.iter() {
            let ship = entities[SHIP_POSITION_ENTITY_IDX];
            min_x = std::cmp::min(min_x, ship.0);
            max_x = std::cmp::max(max_x, ship.0);
            min_y = std::cmp::min(min_y, ship.1);
            max_y = std::cmp::max(max_y, ship.1);
        }

        let grid_width = (max_x - min_x) as i32;
        let grid_height = (max_y - min_y) as i32;
        self.painter.set_aspect_ratio(grid_width, grid_height);
        let grid_display_width = 1.0 / grid_width as f64;
        let grid_display_height = (1.0 / grid_height as f64) / self.painter.aspect_ratio();

        self.painter.line_width(grid_display_width * 3.);

        // Mark origin:
        self.painter.fill_style_rgb(125, 125, 0);
        self.painter.fill_circle(
            -min_x as f64 * grid_display_width,
            -min_y as f64 * grid_display_height,
            grid_display_width * 10.,
        );

        let mut last_ship_position = self.entities_over_time[0][SHIP_POSITION_ENTITY_IDX];
        for (idx, entities) in self.entities_over_time.iter().enumerate().skip(0) {
            self.painter
                .status_text(&format!("Iteration: {: >3}", idx,));

            let new_ship_position = entities[SHIP_POSITION_ENTITY_IDX];

            let start_x = (last_ship_position.0 - min_x) as f64 * grid_display_width;
            let start_y = (last_ship_position.1 - min_y) as f64 * grid_display_height;
            let end_x = (new_ship_position.0 - min_x) as f64 * grid_display_width;
            let end_y = (new_ship_position.1 - min_y) as f64 * grid_display_height;

            self.painter.stroke_style_rgb(0xff, 0x00, 0x00);
            self.painter.begin_path();
            self.painter.move_to(start_x, start_y);
            self.painter.line_to(end_x, end_y);
            self.painter.stroke();
            self.painter.meta_delay(100);

            last_ship_position = new_ship_position;
        }
    }
}
