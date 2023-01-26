use std::io::Write;

use crate::Coordinate;

pub enum SvgTransform {
    Translate(Coordinate, Coordinate),
    Scale(Coordinate, Coordinate),
    // TODO
}

impl SvgTransform {
    pub(crate) fn write<W: Write>(&self, writer: &mut W) {
        #![allow(clippy::unwrap_used)]
        // TODO: Co exist with styling
        writer.write_all(b" style=\"transform:").unwrap();
        match self {
            Self::Translate(x, y) => {
                writer
                    .write_all(format!("translate({x}px,{y}px)\"").as_bytes())
                    .unwrap();
            }
            Self::Scale(x, y) => {
                writer
                    .write_all(format!("scale({x}px,{y}px)\"").as_bytes())
                    .unwrap();
            }
        }
    }
}
