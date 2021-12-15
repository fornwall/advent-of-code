use super::day12::{SHIP_POSITION_ENTITY_IDX, WAYPOINT_ENTITY_IDX};
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
            if !part_one {
                let waypoint_relative = entities[WAYPOINT_ENTITY_IDX];
                let waypoint = (waypoint_relative.0 + ship.0, waypoint_relative.1 + ship.1);
                min_x = std::cmp::min(min_x, waypoint.0);
                max_x = std::cmp::max(max_x, waypoint.0);
                min_y = std::cmp::min(min_y, waypoint.1);
                max_y = std::cmp::max(max_y, waypoint.1);
            }
        }

        let grid_width = (max_x - min_x) as i32;
        let grid_height = (max_y - min_y) as i32;
        self.painter.set_aspect_ratio(grid_width, grid_height);
        let grid_display_width = 1.0 / f64::from(grid_width);
        let grid_display_height = (1.0 / f64::from(grid_height)) / self.painter.aspect_ratio();

        self.painter.line_width(0.002);

        // Mark origin:
        self.painter.fill_style_rgb(125, 125, 0);
        self.painter.fill_circle(
            -f64::from(min_x) * grid_display_width,
            -f64::from(min_y) * grid_display_height,
            0.005,
        );

        let mut last_ship_position = self.entities_over_time[0][SHIP_POSITION_ENTITY_IDX];
        for (idx, entities) in self.entities_over_time.iter().enumerate().skip(0) {
            self.painter
                .status_text(&format!("Iteration: {: >3}", idx,));

            let new_ship_position = entities[SHIP_POSITION_ENTITY_IDX];

            let start_x = f64::from(last_ship_position.0 - min_x) * grid_display_width;
            let start_y = f64::from(last_ship_position.1 - min_y) * grid_display_height;
            let end_x = f64::from(new_ship_position.0 - min_x) * grid_display_width;
            let end_y = f64::from(new_ship_position.1 - min_y) * grid_display_height;

            self.painter.stroke_style_rgb(0xff, 0x00, 0x00);
            self.painter.begin_path();
            self.painter.move_to(start_x, start_y);
            self.painter.line_to(end_x, end_y);
            self.painter.stroke();

            if !part_one {
                self.painter.meta_switch_layer(1);
                self.painter.fill_style_rgb_packed(0x0000_FF00);
                let waypoint_position = entities[WAYPOINT_ENTITY_IDX];
                let draw_x = f64::from(new_ship_position.0 + waypoint_position.0 - min_x)
                    * grid_display_width;
                let draw_y = f64::from(new_ship_position.1 + waypoint_position.1 - min_y)
                    * grid_display_height;
                self.painter.fill_circle(draw_x, draw_y, 0.005);
                self.painter.meta_switch_layer(0);
            }

            self.painter.meta_delay(100);

            last_ship_position = new_ship_position;
        }
    }
}
