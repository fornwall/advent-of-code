use std::io::Write;

/// - Named colors — orange
/// - Hex colors — #FF9E2C
/// - RGB and RGBa colors — Rgb(255, 158, 44) and Rgba(255, 158, 44, .5)
/// - HSL and HSLa colors — hsl(32, 100%, 59%) and hsla(32, 100%, 59%, .5)
/// - References to SVG patterns: url(#pattern-id)
#[derive(Copy, Clone)]
pub enum SvgColor {
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, f64),
    RgbPercentage(f64, f64, f64),
    RgbaPercentage(f64, f64, f64, f64),
    // https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/hsl
    Hsl(f64, usize, usize),
    // TODO: hue
    // TODO: pattern reference
}

impl SvgColor {
    pub(crate) fn write_fill(&self, writer: &mut dyn Write) {
        self.write_internal("fill", writer);
    }

    pub(crate) fn write_stroke(&self, writer: &mut dyn Write) {
        self.write_internal("stroke", writer);
    }

    fn write_internal(&self, what: &str, writer: &mut dyn Write) {
        #![allow(clippy::panic)]
        #![allow(clippy::unwrap_used)]
        match self {
            Self::Rgb(r, g, b) => {
                writer
                    .write_all(format!(" {what}=\"rgb({r}, {g}, {b})\"").as_bytes())
                    .unwrap();
            }
            Self::RgbPercentage(r, g, b) => {
                writer
                    .write_all(format!(" {what}=\"rgb({r}%, {g}%, {b}%)\"").as_bytes())
                    .unwrap();
            }
            Self::Hsl(hue, saturation, lightness) => {
                writer
                    .write_all(
                        format!(" {what}=\"hsl({hue} {saturation}% {lightness}%)\"").as_bytes(),
                    )
                    .unwrap();
            }
            _ => {
                panic!("Unhandled fill");
            }
        }
    }
}
