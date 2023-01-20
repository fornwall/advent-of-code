use crate::{Coordinate, SvgColor, SvgElement, SvgId};
use std::io::Write;

pub struct SvgCircle {
    pub cx: Coordinate,
    pub cy: Coordinate,
    pub r: Coordinate,
    pub fill: Option<SvgColor>,
}

impl From<SvgCircle> for SvgElement {
    fn from(value: SvgCircle) -> Self {
        Self::Circle(value)
    }
}

impl SvgCircle {
    pub(crate) fn write<W: Write>(&self, id: Option<SvgId>, writer: &mut W) {
        #![allow(clippy::unwrap_used)]
        writer
            .write_all(
                format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"{}\"",
                    self.cx, self.cy, self.r
                )
                .as_bytes(),
            )
            .unwrap();
        if let Some(id) = id {
            id.write(writer);
        }
        if let Some(fill) = &self.fill {
            fill.write_fill(writer);
        }
        writer.write_all(b"/>\n").unwrap();
    }
}
