use super::computer::{Computer, Instruction};
use crate::painter::PainterRef;

pub fn start_rendering(painter: &mut PainterRef) {
    painter.set_aspect_ratio(16, 9);
}

pub fn render(
    painter: &mut PainterRef,
    computer: &Computer,
    executed_instructions: &Vec<bool>,
    switched_instruction_idx: Option<usize>,
) {
    painter.clear();

    let cols = 14;
    // +1 for exit:
    let instructions_per_col = ((computer.instructions.len() + 1) + cols - 1) / cols;
    let font_size = 1. / (instructions_per_col + 5) as f64;

    let found_loop =
        executed_instructions.get(computer.instruction_pointer as usize) == Some(&true);

    for idx in 0..=computer.instructions.len() {
        let mut str = if idx == computer.instructions.len() {
            "exit".to_string()
        } else {
            match computer.instructions[idx] {
                Instruction::Jmp(parameter) => {
                    format!("jmp {}", parameter)
                }
                Instruction::Nop(parameter) => {
                    format!("nop {}", parameter)
                }
                Instruction::Acc(parameter) => {
                    format!("acc {}", parameter)
                }
            }
        };

        let col = idx / instructions_per_col;
        let row = idx % instructions_per_col;

        let x_draw = (col as f64) / (cols as f64);
        let y_draw = (row as f64) / (instructions_per_col as f64);

        let color = if idx == computer.instruction_pointer as usize {
            str = format!("{}⚠️️", str);
            "yellow"
        } else if switched_instruction_idx == Some(idx) {
            str = format!("{}⚙️", str);
            "chartreuse"
        } else if idx < executed_instructions.len() && executed_instructions[idx] {
            "red"
        } else {
            "white"
        };

        painter.fill_text_style(color);
        painter.draw_text_top_left(x_draw, y_draw, font_size, &str);
    }

    if found_loop {
        painter.meta_delay(500);
    } else {
        painter.end_frame();
    }
}
