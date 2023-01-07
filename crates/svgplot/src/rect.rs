use crate::escape::escape_text;
use crate::{Coordinate, SvgColor, SvgElement, SvgId};
use std::io::Write;

#[derive(Default)]
pub struct Rect {
    /// The left edge of the rectangle.
    pub x: Coordinate,
    /// The top edge of the rectangle.
    pub y: Coordinate,
    pub width: Coordinate,
    pub height: Coordinate,
    pub fill: Option<SvgColor>,
    pub title: Option<String>,
    pub class: Option<String>,
}

impl From<Rect> for SvgElement {
    fn from(value: Rect) -> Self {
        Self::Rect(value)
    }
}

impl Rect {
    pub(crate) fn write<W: Write>(&self, id: Option<SvgId>, buffer: &mut W) {
        #![allow(clippy::unwrap_used)]
        buffer
            .write_all(
                format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"",
                    self.x, self.y, self.width, self.height
                )
                .as_bytes(),
            )
            .unwrap();
        if let Some(id) = id {
            id.write(buffer);
        }
        if let Some(fill) = &self.fill {
            fill.write_fill(buffer);
        }
        if let Some(title) = &self.title {
            buffer
                .write_all(format!("><title>{}</title></rect>", escape_text(title)).as_bytes())
                .unwrap();
        } else {
            buffer.write_all(b"/>\n").unwrap();
        }
    }
}
