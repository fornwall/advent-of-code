use crate::common_attributes::{implement_common_attributes, CommonAttributes};
use crate::escape::escape_xml;
use crate::{Coordinate, SvgColor, SvgElement, SvgId, SvgStrokeLinecap, SvgTransform};
use std::io::Write;

#[derive(Default)]
pub struct SvgRect {
    /// The left edge of the rectangle.
    pub x: Coordinate,
    /// The top edge of the rectangle.
    pub y: Coordinate,
    pub width: Coordinate,
    pub height: Coordinate,
    common_attributes: CommonAttributes,
}

implement_common_attributes!(SvgRect);

impl From<SvgRect> for SvgElement {
    fn from(value: SvgRect) -> Self {
        Self::Rect(value)
    }
}

impl SvgRect {
    pub fn x<C: Into<Coordinate>>(mut self, x: C) -> Self {
        self.x = x.into();
        self
    }
    pub fn y<C: Into<Coordinate>>(mut self, y: C) -> Self {
        self.y = y.into();
        self
    }
    pub fn width<C: Into<Coordinate>>(mut self, width: C) -> Self {
        self.width = width.into();
        self
    }
    pub fn height<C: Into<Coordinate>>(mut self, height: C) -> Self {
        self.height = height.into();
        self
    }

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
        self.common_attributes.write(buffer);
        if let Some(title) = &self.common_attributes.title {
            buffer
                .write_all(format!("><title>{}</title></rect>", escape_xml(title)).as_bytes())
                .unwrap();
        } else {
            buffer.write_all(b"/>\n").unwrap();
        }
    }
}
