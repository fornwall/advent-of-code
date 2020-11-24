use crate::buffer::CircularOutputBuffer;

enum Command {
    BeginPath = 1,
    Clear,
    ClosePath,
    FillRect,
    FillSquare,
    EndFrame,
    FillStyleRGB,
    LineWidth,
    StrokeSquare,
    StrokeStyleRGB,
    FillText,
    ShadowBlur,
    ShadowColor,
    Done,
    Delay,
    SwitchLayer,
    FillStyleRGBA,
    SetAspectRatio,
}

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
        self.output_buffer.write(Command::Clear as i32);
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/beginPath
    pub fn begin_path(&mut self) {
        self.output_buffer.write(Command::BeginPath as i32);
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/closePath
    pub fn close_path(&mut self) {
        self.output_buffer.write(Command::ClosePath as i32);
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect
    pub fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.output_buffer.write(Command::FillRect as i32);
        self.output_buffer.write_float4(x, y, w, h);
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect
    pub fn fill_square(&mut self, x: f64, y: f64, size: f64) {
        self.output_buffer.write(Command::FillSquare as i32);
        self.output_buffer.write_float3(x, y, size);
    }

    pub fn fill_text(&mut self, text: &str, x: f64, y: f64) {
        self.output_buffer.write(Command::FillText as i32);
        self.output_buffer.write_text(text);
        self.output_buffer.write_float(x);
        self.output_buffer.write_float(y);
    }

    pub fn stroke_square(&mut self, x: i32, y: i32, size: i32) {
        self.output_buffer.write(Command::StrokeSquare as i32);
        self.output_buffer.write(x);
        self.output_buffer.write(y);
        self.output_buffer.write(size);
    }

    pub fn line_width(&mut self, width: i32) {
        self.output_buffer.write(Command::LineWidth as i32);
        self.output_buffer.write(width);
    }

    pub fn fill_style_rgb(&mut self, r: i32, g: i32, b: i32) {
        self.output_buffer.write(Command::FillStyleRGB as i32);
        self.output_buffer.write(r);
        self.output_buffer.write(g);
        self.output_buffer.write(b);
    }

    pub fn fill_style_rgba(&mut self, r: i32, g: i32, b: i32, a: f64) {
        self.output_buffer.write(Command::FillStyleRGBA as i32);
        self.output_buffer.write(r);
        self.output_buffer.write(g);
        self.output_buffer.write(b);
        self.output_buffer.write_float(a);
    }

    pub fn stroke_style_rgb(&mut self, r: i32, g: i32, b: i32) {
        self.output_buffer.write(Command::StrokeStyleRGB as i32);
        self.output_buffer.write(r);
        self.output_buffer.write(g);
        self.output_buffer.write(b);
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor
    pub fn shadow_color(&mut self, r: i32, g: i32, b: i32) {
        self.output_buffer.write(Command::ShadowColor as i32);
        self.output_buffer.write(r);
        self.output_buffer.write(g);
        self.output_buffer.write(b);
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur
    pub fn shadow_blur(&mut self, level: i32) {
        self.output_buffer.write(Command::ShadowBlur as i32);
        self.output_buffer.write(level);
    }

    pub fn end_frame(&mut self) {
        self.output_buffer.write(Command::EndFrame as i32);
        self.output_buffer.flush();
        self.output_buffer.perhaps_wait();
    }

    pub fn meta_delay(&mut self, delay_ms: u16) {
        self.output_buffer.write(Command::Delay as i32);
        self.output_buffer.write(delay_ms as i32);
        // FIXME: flush? end_frame?
        self.output_buffer.flush();
    }

    pub fn meta_switch_layer(&mut self, to_layer: u16) {
        self.output_buffer.write(Command::SwitchLayer as i32);
        self.output_buffer.write(to_layer as i32);
    }

    pub fn set_aspect_ratio(&mut self, width: i32, height: i32) {
        self.output_buffer.write(Command::SetAspectRatio as i32);
        let aspect_ratio = width as f64 / height as f64;
        self.output_buffer.write_float(aspect_ratio);
        self.output_buffer.flush();
    }
}

impl Drop for ToBufferDrawer {
    fn drop(&mut self) {
        self.output_buffer.write(Command::Done as i32);
        self.output_buffer.flush();
        self.output_buffer.report_stats();
        self.output_buffer.wait_forever();
    }
}
