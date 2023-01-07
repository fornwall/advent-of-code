use crate::{Circle, Rect, SvgGroup, SvgId, SvgPath, SvgScript};
use std::io::Write;

pub enum SvgElement {
    Rect(Rect),
    Circle(Circle),
    Script(SvgScript),
    Group(SvgGroup),
    Path(SvgPath),
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
        }
    }
}
