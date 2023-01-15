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
pub use symbol::*;
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
pub mod symbol;
pub mod view_box;

pub type Coordinate = f64;

pub type SvgInteger = i64;

pub struct SvgImage {
    id_sequence: u32,
    dimensions: Option<(SvgInteger, SvgInteger)>,
    view_box: Option<ViewBox>,
    elements: Vec<(Option<SvgId>, SvgElement)>,
    data_attributes: Vec<(String, String)>,
    common_attributes: CommonAttributes,
}

define_element!(SvgImage);

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
        self.elements.push((None, element.into()));
        self
    }

    pub fn add_with_id<E: Into<SvgElement>>(&mut self, element: E) -> SvgId {
        let new_id = SvgId {
            value: self.id_sequence,
        };
        self.id_sequence += 1;
        self.elements.push((Some(new_id), element.into()));
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
                .write_all(format!(" x=\"{}\" y=\"{}\"", x, y).as_bytes())
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

        for (id, element) in &self.elements {
            element.write(*id, &mut buffer);
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
            SvgGroup::with_elements(vec![Rect {
                x: Coordinate::from(offset_x),
                y: Coordinate::from(offset_y),
                width: 100.,
                height: 100.,
                fill: Some(SvgColor::Rgb(color.0, color.1, color.2)),
                title: None,
                class: None,
            }])
            .style("opacity: 0"),
        );
        if color.0 == 0xff {
            image.add(SvgScript::new(format!(
                "setTimeout(() => {{ document.getElementById('{}').remove(); }}, 1000);",
                id
            )));
        }
    }
    image.add(SvgPath {
        stroke: Some(SvgColor::Rgb(0xFF, 0xFF, 0)),
        shape: SvgPathShape::at(10.6, 10.).close(),
        ..Default::default()
    });
    println!("###:\n{}\n###", image.to_svg_string());
    /*

    */
}
