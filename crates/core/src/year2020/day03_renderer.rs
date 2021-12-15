use super::day03::Map;
use crate::painter::PainterRef;

pub fn render(map: &Map, slopes: &[(usize, usize)], painter: &mut PainterRef) {
    let mut num_horizontal_repeats = 1;
    for slope in slopes {
        let num_iterations = map.rows / slope.1;
        num_horizontal_repeats = std::cmp::max(
            num_horizontal_repeats,
            1 + (slope.0 * num_iterations) / map.cols,
        );
    }

    let min_x = 0;
    let max_x = map.cols * num_horizontal_repeats;
    let min_y = 0;
    let max_y = map.rows;

    let grid_width = (max_x - min_x + 1) as i32;
    let grid_height = (max_y - min_y + 1) as i32;

    painter.set_aspect_ratio(16, 9);
    painter.end_frame();
    painter.meta_delay(100);

    let grid_display_width = 1.0 / f64::from(grid_width);
    let grid_display_height = (1.0 / f64::from(grid_height)) / painter.aspect_ratio();

    let draw_rect = |x, y, rgb, circle, painter: &mut PainterRef| {
        let draw_width = grid_display_width;
        let draw_height = grid_display_height;
        let draw_x = (x - min_x) as f64 * grid_display_width;
        let draw_y = (y - min_y) as f64 * draw_height;
        painter.fill_style_rgb_packed(rgb);
        if circle {
            painter.fill_circle(
                draw_x + draw_width / 2.,
                draw_y + draw_height / 2.,
                grid_display_width * 3.,
            );
        } else {
            painter.fill_rect(draw_x, draw_y, draw_width, draw_height);
        }
    };

    for y in 0..map.rows {
        for x in 0..(map.cols * num_horizontal_repeats) {
            if map.tree_at(x, y) {
                draw_rect(x, y, 0x00_FF00, false, painter);
            }
        }
    }

    painter.end_frame();

    let mut current_product: usize = 1;

    for slope in slopes {
        let initial_position = (0, 0);
        let mut trees_seen_now = 0;

        let set_status =
            |slope: &(usize, usize), trees_seen_now, current_product, painter: &mut PainterRef| {
                if slopes.len() == 1 {
                    painter.status_text(&format!(
                        "Slope: {},{} Trees: {: >3}",
                        slope.0, slope.1, trees_seen_now
                    ));
                } else {
                    painter.status_text(&format!(
                        "Slope: {},{} Trees: {: >3} Product: {: >8}",
                        slope.0, slope.1, trees_seen_now, current_product
                    ));
                }
            };

        for position in std::iter::successors(Some(initial_position), |pos| {
            let new_pos = (pos.0 + slope.0, pos.1 + slope.1);
            if new_pos.1 < map.rows {
                Some(new_pos)
            } else {
                None
            }
        }) {
            if map.tree_at(position.0, position.1) {
                painter.play_sound(1);
                trees_seen_now += 1;
            }
            draw_rect(position.0, position.1, 0xFF_0000, true, painter);

            set_status(slope, trees_seen_now, current_product, painter);
            painter.end_frame();
        }

        current_product *= trees_seen_now;

        set_status(slope, trees_seen_now, current_product, painter);
        painter.end_frame();
    }
}
