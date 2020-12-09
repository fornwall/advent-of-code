use super::computer::{Computer, Instruction};
use crate::painter::PainterRef;

pub fn start_rendering(painter: &mut PainterRef, computer: &Computer) {
    painter.set_aspect_ratio(16, 9);
    // painter.status_text(&format!("Instructions: {}", computer.instructions.len()));
}

pub fn render(painter: &mut PainterRef, computer: &Computer, executed_instructions: &Vec<bool>) {
    painter.clear();

    let cols = 14;
    let instructions_per_col = (computer.instructions.len() + cols - 1) / cols;
    let max_string_len = 10;
    let font_size = 1. / (instructions_per_col + 5) as f64;

    for (idx, instruction) in computer.instructions.iter().enumerate() {
        let str = match instruction {
            Instruction::Jmp(parameter) => {
                format!("jmp {}", parameter)
            }
            Instruction::Nop(parameter) => {
                format!("nop {}", parameter)
            }
            Instruction::Acc(parameter) => {
                format!("acc {}", parameter)
            }
        };

        let col = idx / instructions_per_col;
        let row = idx % instructions_per_col;

        let x_draw = (col as f64) / (cols as f64);
        let y_draw = (row as f64) / (instructions_per_col as f64);

        let color = if executed_instructions[idx] {
            "white"
        } else {
            "red"
        };
        painter.fill_text_style(color);
        painter.draw_text_top_left(x_draw, y_draw, font_size, &str);
    }
    painter.end_frame();
}
