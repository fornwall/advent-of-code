use std::io::Write;

use crate::common_attributes::{implement_common_attributes, CommonAttributes};
use crate::{Coordinate, SvgColor, SvgElement, SvgId, SvgStrokeLinecap, SvgTransform};

pub struct SvgUse {
    id: SvgId,
    x: Option<Coordinate>,
    y: Option<Coordinate>,
    common_attributes: CommonAttributes,
}

implement_common_attributes!(SvgUse);

impl From<SvgUse> for SvgElement {
    fn from(value: SvgUse) -> Self {
        Self::Use(value)
    }
}

impl SvgUse {
    pub const fn new(id: SvgId) -> Self {
        Self {
            id,
            x: None,
            y: None,
            common_attributes: CommonAttributes::new(),
        }
    }

    pub const fn x(mut self, new_x: Coordinate) -> Self {
        self.x = Some(new_x);
        self
    }

    pub const fn y(mut self, new_y: Coordinate) -> Self {
        self.y = Some(new_y);
        self
    }

    pub(crate) fn write<W: Write>(&self, id: Option<SvgId>, writer: &mut W) {
        #![allow(clippy::unwrap_used)]
        writer.write_all(b"<use").unwrap();
        if let Some(id) = id {
            id.write(writer);
        }
        writer
            .write_all(format!(" href=\"#{}\"", self.id).as_bytes())
            .unwrap();
        if let Some(x) = self.x {
            writer.write_all(format!(" x=\"{x}\"").as_bytes()).unwrap();
        }
        if let Some(y) = self.y {
            writer.write_all(format!(" y=\"{y}\"").as_bytes()).unwrap();
        }

        self.common_attributes.write(writer);

        writer.write_all(b"/>\n").unwrap();
    }
}
