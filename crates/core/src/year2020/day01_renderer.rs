use crate::painter::PainterRef;
use core::cmp::Ordering::{Equal, Greater, Less};

pub fn render_part_one(input: &mut Vec<u32>, painter: &mut PainterRef) {
    let desired_sum = 2020;

    let mut min_value = u32::MAX;
    let mut max_value = u32::MIN;
    for &value in input.iter() {
        min_value = std::cmp::min(min_value, value);
        max_value = std::cmp::max(max_value, value);
    }

    max_value = desired_sum + 100;
    min_value = 0;
    let value_span = max_value - min_value;

    let grid_draw_width = 1. / input.len() as f64;
    let grid_draw_height = 1. / value_span as f64;

    let (mut left, mut right) = (0, input.len() - 1);
    while left != right {
        let (left_value, right_value) = (input[left], input[right]);
        let candidate_sum = left_value + right_value;

        painter.clear();

        // Draw line showing desired sum:
        painter.fill_style_rgb(0x0, 0x255, 0x00);
        let desired_sum_height = (desired_sum - min_value) as f64 * grid_draw_height;
        painter.fill_rect(0., 1. - desired_sum_height, 1., grid_draw_height * 10.);

        for (i, value) in input.iter().enumerate() {
            if i == right || i == left {
                painter.fill_style_rgb(0x255, 0x00, 0x00);
            } else if i % 2 == 0 {
                painter.fill_style_rgb(0x78, 0xC3, 0xFB);
            } else {
                painter.fill_style_rgb(0x89, 0xA6, 0xFB);
            }
            let draw_height = (value - min_value) as f64 * grid_draw_height;
            painter.fill_rect(
                i as f64 * grid_draw_width,
                1. - draw_height,
                grid_draw_width,
                draw_height,
            );
        }
        painter.meta_delay(100);

        match candidate_sum.cmp(&desired_sum) {
            Equal => {
                return;
            }
            Less => {
                left += 1;
            }
            Greater => {
                right -= 1;
            }
        }
    }
}

pub fn render_part_two(input: &mut Vec<u32>, painter: &mut PainterRef) {
    let desired_sum = 2020;

    let mut min_value = u32::MAX;
    let mut max_value = u32::MIN;
    for &value in input.iter() {
        min_value = std::cmp::min(min_value, value);
        max_value = std::cmp::max(max_value, value);
    }

    max_value = desired_sum + 100;
    min_value = 0;
    let value_span = max_value - min_value;

    let start_x = -10;

    let grid_draw_width = 1. / (input.len() as i32 - start_x) as f64;
    let grid_draw_height = 1. / value_span as f64;

    const COLOR_LEFTMOST: i32 = 0xFFFFFF;
    const COLOR_LEFT: i32 = 0x00FF00;
    const COLOR_RIGHT: i32 = 0xFF0000;

    let mut iteration = 0;
    for (leftmost_index, leftmost_value) in input.iter().enumerate() {
        let desired_sub_sum = desired_sum - leftmost_value;
        let (mut left, mut right) = (leftmost_index + 1, input.len() - 1);

        while left != right {
            let (left_value, right_value) = (input[left], input[right]);
            let candidate_sum = left_value + right_value;

            painter.clear();
            painter.status_text(&format!("Size: {}   Iteration: {}", input.len(), iteration));
            iteration += 1;

            // Draw line showing desired sum:
            painter.fill_style_rgba(0xFF, 0xFF, 0xFF, 0.4);
            let desired_sum_height = (desired_sum - min_value) as f64 * grid_draw_height;
            painter.fill_rect(0., 1. - desired_sum_height, 1., grid_draw_height * 10.);

            for (i, value) in input.iter().enumerate() {
                if i == left {
                    painter.fill_style_rgb_packed(COLOR_LEFT);
                } else if i == right {
                    painter.fill_style_rgb_packed(COLOR_RIGHT);
                } else if i == leftmost_index {
                    painter.fill_style_rgb_packed(COLOR_LEFTMOST);
                } else if i % 2 == 0 {
                    painter.fill_style_rgb(0x78, 0xC3, 0xFB);
                } else {
                    painter.fill_style_rgb(0x89, 0xA6, 0xFB);
                }
                let draw_height = (value - min_value) as f64 * grid_draw_height;
                painter.fill_rect(
                    (i as i32 - start_x) as f64 * grid_draw_width,
                    1. - draw_height,
                    grid_draw_width,
                    draw_height,
                );

                if i == left || i == right || i == leftmost_index {
                    painter.draw_text(
                        (i as i32 - start_x) as f64 * grid_draw_width,
                        1. - draw_height,
                        0.02,
                        &format!("{}", value),
                    );
                }
            }

            {
                // Draw stacked.
                let draw_x = 0.;
                let draw_y = 1. - ((leftmost_value - min_value) as f64 * grid_draw_height);
                let draw_height = *leftmost_value as f64 * grid_draw_height;
                painter.fill_style_rgb_packed(COLOR_LEFTMOST);
                painter.fill_rect(draw_x, draw_y, grid_draw_width, draw_height);

                let draw_y =
                    1. - ((left_value + leftmost_value - min_value) as f64 * grid_draw_height);
                let draw_height = (left_value) as f64 * grid_draw_height;
                painter.fill_style_rgb_packed(COLOR_LEFT);
                painter.fill_rect(draw_x, draw_y, grid_draw_width, draw_height);

                let draw_y = 1.
                    - ((right_value + left_value + leftmost_value - min_value) as f64
                        * grid_draw_height);
                let draw_height = (right_value) as f64 * grid_draw_height;
                painter.fill_style_rgb_packed(COLOR_RIGHT);
                painter.fill_rect(draw_x, draw_y, grid_draw_width, draw_height);
            }
            painter.end_frame();

            match candidate_sum.cmp(&desired_sub_sum) {
                Equal => {
                    return;
                }
                Less => {
                    left += 1;
                }
                Greater => {
                    right -= 1;
                }
            }
        }
    }
}
