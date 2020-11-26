use super::int_code::{Program, Word};
#[cfg(feature = "visualization")]
use crate::painter::Painter;
use crate::Input;
#[cfg(feature = "visualization")]
use std::collections::HashMap;

#[cfg(feature = "visualization")]
fn render(
    mut painter: &mut Box<dyn Painter>,
    current_score: Word,
    tiles: &HashMap<(Word, Word), Word>,
) {
    painter.clear();

    let mut min_x = Word::MAX;
    let mut max_x = Word::MIN;
    let mut min_y = Word::MAX;
    let mut max_y = Word::MIN;
    for &(x, y) in tiles.keys() {
        min_x = std::cmp::min(min_x, x);
        max_x = std::cmp::max(max_x, x);
        min_y = std::cmp::min(min_y, y);
        max_y = std::cmp::max(max_y, y);
    }

    let grid_width = (max_x - min_x + 1) as i32;
    let grid_height = (max_y - min_y + 1) as i32;

    if current_score == 0 {
        painter.set_aspect_ratio(grid_width, grid_height);
    }

    let grid_display_width = 1.0 / grid_width as f64;
    let grid_display_height = 1.0 / grid_height as f64;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let draw_rect = |the_drawer: &mut Box<dyn Painter>| {
                let draw_x = (x - min_x) as f64 * grid_display_width;
                let draw_y = (y - min_y) as f64 * grid_display_height;
                the_drawer.fill_rect(
                    draw_x,
                    draw_y,
                    grid_display_width * 0.95,
                    grid_display_height * 0.95,
                );
            };
            match tiles.get(&(x, y)) {
                Some(1) => {
                    // Wall.
                    painter.fill_style_rgb(255, 0, 0);
                    draw_rect(&mut painter);
                }
                Some(2) => {
                    // Thing to blow up.
                    painter.fill_style_rgb(0, 255, 255);
                    draw_rect(&mut painter);
                }
                Some(3) => {
                    // Paddle.
                    painter.fill_style_rgb(0, 0, 255);
                    draw_rect(&mut painter);
                }
                Some(4) => {
                    // Ball.
                    painter.fill_style_rgb(255, 255, 255);
                    draw_rect(&mut painter);
                }
                _ => {}
            };
        }
    }
    painter.end_frame();
}

pub fn solve(input: &mut Input) -> Result<Word, String> {
    let mut program = Program::parse(&input.text)?;

    if input.is_part_one() {
        let output = program.run_for_output()?;
        return Ok(output
            .iter()
            .skip(2)
            .step_by(3)
            .filter(|&&t| t == 2)
            .count() as Word);
    }

    // "Memory address 0 represents the number of quarters that
    // have been inserted; set it to 2 to play for free."
    program.write_memory(0, 2);

    #[cfg(feature = "visualization")]
    let mut tiles = HashMap::new();

    let mut current_score = 0;
    let mut ball_x = -1;
    let mut paddle_x = -1;

    loop {
        let output = program.run_for_output()?;
        output.chunks_exact(3).for_each(|chunk| {
            let (x, y, third) = (chunk[0], chunk[1], chunk[2]);
            if x == -1 && y == 0 {
                current_score = third;
            } else {
                #[cfg(feature = "visualization")]
                tiles.insert((x, y), third);

                match third {
                    3 => paddle_x = x,
                    4 => ball_x = x,
                    _ => {}
                }
            }
        });

        #[cfg(feature = "visualization")]
        render(&mut input.painter, current_score, &tiles);

        if program.is_halted() {
            break;
        }

        program.input(ball_x.cmp(&paddle_x) as Word);
    }

    Ok(current_score)
}

#[test]
pub fn tests_part1() {
    assert_eq!(
        solve(&mut Input::part_one(include_str!("day13_input.txt"))),
        Ok(462)
    );
}

#[test]
fn tests_part2() {
    assert_eq!(
        solve(&mut Input::part_two(include_str!("day13_input.txt"))),
        Ok(23981)
    );
}
