use crate::buffer::CircularOutputBuffer;

enum Command {
    COMMAND_BEGIN_PATH = 1,
    COMMAND_CLEAR,
    COMMAND_CLOSE_PATH,
    COMMAND_FILL_RECT,
    COMMAND_FILL_SQUARE,
    COMMAND_END_FRAME,
    COMMAND_FILL_STYLE_RGB,
    COMMAND_LINE_WIDTH,
    COMMAND_STROKE_SQUARE,
    COMMAND_STROKE_STYLE_RGB,
    COMMAND_FILL_TEXT,
    COMMAND_SHADOW_BLUR,
    COMMAND_SHADOW_COLOR,
}

const COMMAND_CLEAR_EVERYTHING: i32 = 0;
const COMMAND_END_PATH: i32 = 1;

pub struct ToBufferDrawer {
    pub output_buffer: CircularOutputBuffer,
}

impl ToBufferDrawer {
    pub fn new() -> ToBufferDrawer {
        Self {
            output_buffer: CircularOutputBuffer::new(),
        }
    }

    pub fn clear(&mut self) {
        self.output_buffer.write(Command::COMMAND_CLEAR as i32);
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/beginPath
    fn begin_path(&mut self) {
        self.output_buffer.write(Command::COMMAND_BEGIN_PATH as i32);
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/closePath
    fn close_path(&mut self) {
        self.output_buffer.write(Command::COMMAND_CLOSE_PATH as i32);
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect
    pub fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.output_buffer.write(Command::COMMAND_FILL_RECT as i32);
        self.output_buffer
            .write_float4(x as f32, y as f32, w as f32, h as f32);
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect
    pub fn fill_square(&mut self, x: i32, y: i32, size: i32) {
        self.output_buffer
            .write(Command::COMMAND_FILL_SQUARE as i32);
        self.output_buffer.write(x);
        self.output_buffer.write(y);
        self.output_buffer.write(size);
    }

    pub fn fill_text(&mut self, text: &str, x: i32, y: i32) {
        self.output_buffer.write(Command::COMMAND_FILL_TEXT as i32);
        // TODO:
    }

    pub fn stroke_square(&mut self, x: i32, y: i32, size: i32) {
        self.output_buffer
            .write(Command::COMMAND_STROKE_SQUARE as i32);
        self.output_buffer.write(x);
        self.output_buffer.write(y);
        self.output_buffer.write(size);
    }

    pub fn line_width(&mut self, width: i32) {
        self.output_buffer.write(Command::COMMAND_LINE_WIDTH as i32);
        self.output_buffer.write(width);
    }

    pub fn fill_style_rgb(&mut self, r: i32, g: i32, b: i32) {
        self.output_buffer
            .write(Command::COMMAND_FILL_STYLE_RGB as i32);
        //self.output_buffer.write_float3(x as f32, y as f32, size as f32);
        self.output_buffer.write(r);
        self.output_buffer.write(g);
        self.output_buffer.write(b);
    }

    pub fn stroke_style_rgb(&mut self, r: i32, g: i32, b: i32) {
        self.output_buffer
            .write(Command::COMMAND_STROKE_STYLE_RGB as i32);
        self.output_buffer.write(r);
        self.output_buffer.write(g);
        self.output_buffer.write(b);
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor
    pub fn shadow_color(&mut self, r: i32, g: i32, b: i32) {
        self.output_buffer
            .write(Command::COMMAND_SHADOW_COLOR as i32);
        self.output_buffer.write(r);
        self.output_buffer.write(g);
        self.output_buffer.write(b);
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur
    pub fn shadow_blur(&mut self, level: i32) {
        self.output_buffer
            .write(Command::COMMAND_SHADOW_BLUR as i32);
        self.output_buffer.write(level);
    }

    pub fn end_frame(&mut self) {
        self.output_buffer.write(Command::COMMAND_END_FRAME as i32);
        self.output_buffer.flush();
        self.output_buffer.wait();
    }
}
