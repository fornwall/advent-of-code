use std::io::Write;

use crate::common_attributes::{implement_common_attributes, CommonAttributes};
use crate::escape::escape_xml;
use crate::{Coordinate, SvgColor, SvgElement, SvgId, SvgStrokeLinecap, SvgTransform};

#[derive(Default)]
pub struct SvgPath {
    pub shape: SvgShape,
    pub stroke: Option<SvgColor>,
    pub stroke_width: Option<f64>,
    pub common_attributes: CommonAttributes,
}

implement_common_attributes!(SvgPath);

enum SvgPathElement {
    LineAbsolute((Coordinate, Coordinate)),
    LineRelative((Coordinate, Coordinate)),
    ArcRelative(
        (
            Coordinate,
            Coordinate,
            Coordinate,
            Coordinate,
            Coordinate,
            Coordinate,
            Coordinate,
        ),
    ),
    MoveAbsolute((Coordinate, Coordinate)),
    MoveRelative((Coordinate, Coordinate)),
    /// The "Close Path" command, called with Z. This command draws a straight line from the current
    /// position back to the first point of the path. It is often placed at the end of a path node,
    /// although not always
    ///
    /// The SVG syntax for this is 'z' or 'Z'.
    Close,
}

impl From<SvgPath> for SvgElement {
    fn from(value: SvgPath) -> Self {
        Self::Path(value)
    }
}

impl SvgPath {
    #[allow(clippy::missing_const_for_fn)]
    pub fn shape(mut self, shape: SvgShape) -> Self {
        self.shape = shape;
        self
    }

    pub const fn stroke_width(mut self, width: f64) -> Self {
        self.stroke_width = Some(width);
        self
    }

    pub const fn stroke(mut self, color: SvgColor) -> Self {
        self.stroke = Some(color);
        self
    }

    pub(crate) fn write<W: Write>(&self, id: Option<SvgId>, writer: &mut W) {
        #![allow(clippy::unwrap_used)]
        writer.write_all(b"<path").unwrap();
        if let Some(id) = id {
            id.write(writer);
        }
        if let Some(stroke) = &self.stroke {
            stroke.write_stroke(writer);
        }
        self.common_attributes.write(writer);
        if let Some(stroke_width) = &self.stroke_width {
            writer
                .write_all(format!(" stroke-width=\"{stroke_width}\"").as_bytes())
                .unwrap();
        }
        writer.write_all(b" d=\"").unwrap();
        self.shape.write(writer);
        writer.write_all(b"\"").unwrap();
        if let Some(title) = &self.common_attributes.title {
            writer
                .write_all(format!("><title>{}</title></path>", escape_xml(title)).as_bytes())
                .unwrap();
        } else {
            writer.write_all(b"/>\n").unwrap();
        }
    }
}

#[derive(Default)]
pub struct SvgShape {
    elements: Vec<SvgPathElement>,
}

impl SvgShape {
    pub const fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn at<C: Into<Coordinate>>(x: C, y: C) -> Self {
        Self {
            elements: vec![SvgPathElement::MoveAbsolute((x.into(), y.into()))],
        }
    }

    pub fn is_empty(&self) -> bool {
        // If it contains the single initial move command.
        self.elements.is_empty()
    }

    pub fn line_to_absolute<I: Into<Coordinate>>(mut self, x: I, y: I) -> Self {
        self.elements
            .push(SvgPathElement::LineAbsolute((x.into(), y.into())));
        self
    }

    pub fn line_to_relative<C: Into<Coordinate>>(mut self, x: C, y: C) -> Self {
        self.elements
            .push(SvgPathElement::LineRelative((x.into(), y.into())));
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn arc_to_relative<C: Into<Coordinate>>(
        mut self,
        radius_x: C,
        radius_y: C,
        x_axis_rotation: C,
        large_arc_flag: C,
        sweep_flag: C,
        dx: C,
        dy: C,
    ) -> Self {
        self.elements.push(SvgPathElement::ArcRelative((
            radius_x.into(),
            radius_y.into(),
            x_axis_rotation.into(),
            large_arc_flag.into(),
            sweep_flag.into(),
            dx.into(),
            dy.into(),
        )));
        self
    }

    pub fn move_to_absolute<C: Into<Coordinate>>(mut self, x: C, y: C) -> Self {
        self.elements
            .push(SvgPathElement::MoveAbsolute((x.into(), y.into())));
        self
    }

    pub fn move_to_relative(mut self, x: Coordinate, y: Coordinate) -> Self {
        self.elements.push(SvgPathElement::MoveRelative((x, y)));
        self
    }

    pub fn circle_absolute<C: Into<Coordinate>>(self, center_x: C, center_y: C, radius: C) -> Self {
        let radius = radius.into();
        // https://www.smashingmagazine.com/2019/03/svg-circle-decomposition-paths/
        //       M (CX - R), CY
        //       a R,R 0 1,0 (R * 2),0
        //       a R,R 0 1,0 -(R * 2),0
        self.move_to_absolute(center_x.into() - radius, center_y.into())
            .arc_to_relative(radius, radius, 0., 1., 0., radius * 2., 0.)
            .arc_to_relative(radius, radius, 0., 1., 0., -radius * 2., 0.)
    }

    pub fn close(mut self) -> Self {
        self.elements.push(SvgPathElement::Close);
        self
    }

    pub fn data_string(&self) -> String {
        #![allow(clippy::unwrap_used)]
        let mut buffer = Vec::new();
        self.write(&mut buffer);
        String::from_utf8(buffer).unwrap()
    }

    pub(crate) fn write<W: Write>(&self, writer: &mut W) {
        #![allow(clippy::unwrap_used)]
        for element in &self.elements {
            match element {
                SvgPathElement::MoveAbsolute((x, y)) => {
                    writer.write_all(format!("M {x} {y}").as_bytes()).unwrap();
                }
                SvgPathElement::MoveRelative((x, y)) => {
                    writer.write_all(format!("m {x} {y}").as_bytes()).unwrap();
                }
                SvgPathElement::LineAbsolute((x, y)) => {
                    writer.write_all(format!("L {x} {y}").as_bytes()).unwrap();
                }
                SvgPathElement::LineRelative((x, y)) => {
                    writer.write_all(format!("l {x} {y}").as_bytes()).unwrap();
                }
                SvgPathElement::ArcRelative((rx, ry, x_rot, a_flag, s_flag, dx, dy)) => {
                    // a rx ry x-axis-rotation large-arc-flag sweep-flag dx dy
                    writer
                        .write_all(
                            format!("a {rx} {ry} {x_rot} {a_flag} {s_flag} {dx} {dy}").as_bytes(),
                        )
                        .unwrap();
                }
                SvgPathElement::Close => {
                    writer.write_all(b"Z").unwrap();
                }
            }
        }
    }
}
