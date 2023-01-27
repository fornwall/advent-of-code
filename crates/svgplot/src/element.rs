use crate::{SvgCircle, SvgGroup, SvgId, SvgPath, SvgRect, SvgScript, SvgStyle, SvgUse};
use std::io::Write;

pub enum SvgElement {
    Rect(SvgRect),
    Circle(SvgCircle),
    Script(SvgScript),
    Group(SvgGroup),
    Path(SvgPath),
    Style(SvgStyle),
    Use(SvgUse),
}

impl SvgElement {
    pub(crate) fn write<W: Write>(&self, id: Option<SvgId>, writer: &mut W) {
        match self {
            Self::Circle(circle) => {
                circle.write(id, writer);
            }
            Self::Rect(rect) => {
                rect.write(id, writer);
            }
            Self::Script(script) => {
                script.write(id, writer);
            }
            Self::Group(group) => {
                group.write(id, writer);
            }
            Self::Path(path) => {
                path.write(id, writer);
            }
            Self::Style(style) => {
                style.write(id, writer);
            }
            Self::Use(svg_use) => {
                svg_use.write(id, writer);
            }
        }
    }
}
