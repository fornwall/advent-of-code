pub trait Painter {
    fn clear(&mut self);

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/beginPath
    fn begin_path(&mut self);

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/closePath
    fn close_path(&mut self);

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect
    fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64);

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect
    fn fill_square(&mut self, x: f64, y: f64, size: f64);

    /// Show a status of the current situation.
    fn status_text(&mut self, text: &str);

    /// Fills the current path with the current fillStyle.
    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill
    fn fill(&mut self);

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeRect
    fn stroke_square(&mut self, x: i32, y: i32, size: i32);

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle
    fn stroke_style_rgb(&mut self, r: i32, g: i32, b: i32);

    /// Strokes (outlines) the current or given path with the current stroke style.
    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke
    fn stroke(&mut self);

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth
    fn line_width(&mut self, width: f64);

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineTo
    fn line_to(&mut self, x: f64, y: f64);

    ///https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/moveTo
    fn move_to(&mut self, x: f64, y: f64);

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle
    fn fill_style_rgb(&mut self, r: i32, g: i32, b: i32);

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle
    fn fill_style_rgb_packed(&mut self, rgb: i32) {
        let red = (rgb & 0xFF0000) >> 16;
        let green = (rgb & 0x00FF00) >> 8;
        let blue = (rgb & 0x0000FF) >> 0;
        self.fill_style_rgb(red, green, blue);
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle
    fn fill_style_rgba(&mut self, r: i32, g: i32, b: i32, a: f64);

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor
    fn shadow_color(&mut self, r: i32, g: i32, b: i32);

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur
    fn shadow_blur(&mut self, level: i32);

    /// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc
    fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64);

    fn stroke_circle(&mut self, x: f64, y: f64, radius: f64) {
        self.begin_path();
        self.arc(x, y, radius, 0., 2. * std::f64::consts::PI);
        self.stroke();
    }

    fn fill_circle(&mut self, x: f64, y: f64, radius: f64) {
        self.begin_path();
        self.arc(x, y, radius, 0., 2. * std::f64::consts::PI);
        self.fill();
    }

    fn end_frame(&mut self);

    fn meta_delay(&mut self, delay_ms: u16);

    fn meta_switch_layer(&mut self, to_layer: u16);

    fn set_aspect_ratio(&mut self, width: i32, height: i32);

    fn aspect_ratio(&self) -> f64;

    fn await_forever(&mut self);

    fn play_sound(&mut self, sound_id: i32);

    fn draw_text(&mut self, x: f64, y: f64, font_size: f64, text: &str);

    fn log(&mut self, text: &str);
}

pub type PainterRef = Box<dyn Painter>;

pub struct MockPainter;

impl Painter for MockPainter {
    fn clear(&mut self) {}

    fn begin_path(&mut self) {}

    fn close_path(&mut self) {}

    fn fill_rect(&mut self, _x: f64, _y: f64, _w: f64, _h: f64) {}

    fn fill_square(&mut self, _x: f64, _y: f64, _size: f64) {}

    fn status_text(&mut self, _text: &str) {}

    fn fill(&mut self) {}

    fn stroke_square(&mut self, _x: i32, _y: i32, _size: i32) {}

    fn stroke_style_rgb(&mut self, _r: i32, _g: i32, _b: i32) {}

    fn stroke(&mut self) {}

    fn line_width(&mut self, _width: f64) {}

    fn line_to(&mut self, _x: f64, _y: f64) {}

    fn move_to(&mut self, _x: f64, _y: f64) {}

    fn fill_style_rgb(&mut self, _r: i32, _g: i32, _b: i32) {}

    fn fill_style_rgba(&mut self, _r: i32, _g: i32, _b: i32, _a: f64) {}

    fn shadow_color(&mut self, _r: i32, _g: i32, _b: i32) {}

    fn shadow_blur(&mut self, _level: i32) {}

    fn arc(&mut self, _x: f64, _y: f64, _radius: f64, _start_angle: f64, _end_angle: f64) {}

    fn end_frame(&mut self) {}

    fn meta_delay(&mut self, _delay_ms: u16) {}

    fn meta_switch_layer(&mut self, _to_layer: u16) {}

    fn set_aspect_ratio(&mut self, _width: i32, _height: i32) {}

    fn aspect_ratio(&self) -> f64 {
        1.
    }

    fn await_forever(&mut self) {}

    fn play_sound(&mut self, _sound_id: i32) {}

    fn draw_text(&mut self, _x: f64, _y: f64, _font_size: f64, _text: &str) {}

    fn log(&mut self, _text: &str) {}
}
