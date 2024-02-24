use std::io::Write;

use crate::escape::escape_xml;
pub use circle::*;
pub use color::*;
use common_attributes::*;
pub use element::*;
pub use group::*;
pub use id::*;
pub use path::*;
pub use rect::*;
pub use script::*;
pub use stroke::*;
pub use style::*;
pub use svg_use::*;
pub use transform::*;
pub use view_box::*;

pub mod circle;
pub mod color;
pub mod common_attributes;
pub mod element;
pub(crate) mod escape;
pub mod group;
pub mod id;
pub mod path;
pub mod rect;
pub mod script;
pub mod stroke;
pub mod style;
pub mod svg_use;
pub mod transform;
pub mod view_box;

pub type Coordinate = f64;

pub type SvgInteger = i64;

enum OptionalSvgId {
    None,
    Some(SvgId),
    Def(SvgId),
}

pub struct SvgImage {
    id_sequence: u32,
    dimensions: Option<(SvgInteger, SvgInteger)>,
    view_box: Option<ViewBox>,
    elements: Vec<(OptionalSvgId, SvgElement)>,
    data_attributes: Vec<(String, String)>,
    common_attributes: CommonAttributes,
}

implement_common_attributes!(SvgImage);

impl SvgImage {
    pub const fn new() -> Self {
        Self {
            id_sequence: 0,
            dimensions: None,
            view_box: None,
            elements: Vec::new(),
            data_attributes: Vec::new(),
            common_attributes: CommonAttributes::new(),
        }
    }

    pub const fn dimensions(mut self, width: SvgInteger, height: SvgInteger) -> Self {
        self.dimensions = Some((width, height));
        self
    }

    pub fn data_attribute(mut self, name: String, value: String) -> Self {
        self.data_attributes.push((name, value));
        self
    }

    pub fn view_box<V: Into<ViewBox>>(mut self, view_box: V) -> Self {
        self.view_box = Some(view_box.into());
        self
    }

    pub fn add<E: Into<SvgElement>>(&mut self, element: E) -> &mut Self {
        self.elements.push((OptionalSvgId::None, element.into()));
        self
    }

    pub fn add_with_id<E: Into<SvgElement>>(&mut self, element: E) -> SvgId {
        let new_id = SvgId {
            value: self.id_sequence,
        };
        self.id_sequence += 1;
        self.elements
            .push((OptionalSvgId::Some(new_id), element.into()));
        new_id
    }

    pub fn define<E: Into<SvgElement>>(&mut self, element: E) -> SvgId {
        let new_id = SvgId {
            value: self.id_sequence,
        };
        self.id_sequence += 1;
        self.elements
            .push((OptionalSvgId::Def(new_id), element.into()));
        new_id
    }

    pub fn to_svg_string(&self) -> String {
        #![allow(clippy::unwrap_used)]
        let mut buffer = Vec::new();
        buffer
            .write_all(b"<svg xmlns=\"http://www.w3.org/2000/svg\"")
            .unwrap();
        if let Some((x, y)) = &self.dimensions {
            buffer
                .write_all(format!(" x=\"{x}\" y=\"{y}\"").as_bytes())
                .unwrap();
        }
        if let Some(view_box) = &self.view_box {
            let s = format!(
                " viewBox=\"{} {} {} {}\" preserveAspectRatio=\"xMidYMid\"",
                view_box.min_x, view_box.min_y, view_box.width, view_box.height
            );
            buffer.write_all(s.as_bytes()).unwrap();
        }
        for (name, value) in &self.data_attributes {
            buffer
                .write_all(
                    format!("data-{}=\"{}\"", escape_xml(name), escape_xml(value)).as_bytes(),
                )
                .unwrap();
        }
        self.common_attributes.write(&mut buffer);
        buffer.write_all(&[b'>', b'\n']).unwrap();

        let mut first = true;
        for (id, element) in &self.elements {
            if let OptionalSvgId::Def(id) = id {
                if first {
                    first = false;
                    buffer.write_all(b"<defs>").unwrap();
                }
                element.write(Some(*id), &mut buffer);
            }
        }
        if !first {
            buffer.write_all(b"</defs>").unwrap();
        }

        for (id, element) in &self.elements {
            match id {
                OptionalSvgId::None => {
                    element.write(None, &mut buffer);
                }
                OptionalSvgId::Some(id) => {
                    element.write(Some(*id), &mut buffer);
                }
                OptionalSvgId::Def(_) => {}
            }
        }

        buffer.write_all(b"</svg>").unwrap();
        String::from_utf8(buffer).unwrap()
    }
}

#[test]
fn test() {
    let mut image = SvgImage::new()
        .dimensions(200, 00)
        .view_box((-100, -100, 200, 200))
        .style("--step: 0");
    for (offset_x, offset_y, color) in [
        (-100, -100, (0xFF, 0, 0)),
        (0, -100, (0, 0xFF, 0)),
        (-100, 0, (0, 0, 0xFF)),
        (0, 0, (0xFF, 0xFF, 0xFF)),
    ] {
        let id = image.add_with_id(
            SvgGroup::with_elements(vec![SvgRect::default()
                .x(offset_x)
                .y(offset_y)
                .width(100)
                .height(100)
                .fill(SvgColor::Rgb(color.0, color.1, color.2))])
            .style("opacity: 0"),
        );
        if color.0 == 0xff {
            image.add(SvgScript::new(format!(
                "setTimeout(() => {{ document.getElementById('{id}').remove(); }}, 1000);"
            )));
        }
    }
    image.add(SvgPath {
        stroke: Some(SvgColor::Rgb(0xFF, 0xFF, 0)),
        shape: SvgShape::at(10.6, 10.).close(),
        ..Default::default()
    });
}
