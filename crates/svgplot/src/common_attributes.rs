use std::io::Write;

use crate::escape::escape_xml;

/// Common attributes to all SVG elements.
///
/// https://oreillymedia.github.io/Using_SVG/guide/markup.html#common-attributes
#[derive(Default)]
pub struct CommonAttributes {
    pub(crate) style: Option<String>,
    pub(crate) classes: Vec<String>,
    pub(crate) transform: Option<SvgTransform>,
    pub(crate) fill: Option<SvgColor>,
    pub(crate) stroke_linecap: Option<SvgStrokeLinecap>,
    pub(crate) title: Option<String>,
}

impl CommonAttributes {
    pub(crate) const fn new() -> Self {
        Self {
            style: None,
            classes: Vec::new(),
            transform: None,
            fill: None,
            stroke_linecap: None,
            title: None,
        }
    }
    pub(crate) fn write<W: Write>(&self, writer: &mut W) {
        #![allow(clippy::unwrap_used)]
        if let Some(style) = &self.style {
            writer
                .write_all(format!(" style=\"{}\"", escape_xml(style)).as_bytes())
                .unwrap();
        }
        if let Some(fill) = &self.fill {
            fill.write_fill(writer);
        }
        if let Some(transform) = &self.transform {
            transform.write(writer);
        }
        if let Some(stroke_linecap) = &self.stroke_linecap {
            stroke_linecap.write(writer);
        }
        if !self.classes.is_empty() {
            writer.write_all(b" class=\"").unwrap();
            for (idx, class) in self.classes.iter().enumerate() {
                writer
                    .write_all(format!("{}{}", if idx == 0 { "" } else { " " }, class).as_bytes())
                    .unwrap();
            }
            writer.write_all(b"\"").unwrap();
        }
    }
}

macro_rules! implement_common_attributes {
    ($element_name:ident) => {
        impl $element_name {
            pub fn style<S: ToString>(mut self, style: S) -> Self {
                self.common_attributes.style = Some(style.to_string());
                self
            }
            pub fn class<S: ToString>(mut self, class: S) -> Self {
                self.common_attributes.classes.push(class.to_string());
                self
            }
            pub const fn transform(mut self, transform: SvgTransform) -> Self {
                self.common_attributes.transform = Some(transform);
                self
            }
            pub const fn fill(mut self, color: SvgColor) -> Self {
                self.common_attributes.fill = Some(color);
                self
            }
            pub const fn stroke_linecap(mut self, stroke_linecap: SvgStrokeLinecap) -> Self {
                self.common_attributes.stroke_linecap = Some(stroke_linecap);
                self
            }

            #[allow(clippy::missing_const_for_fn)]
            pub fn title(mut self, title: String) -> Self {
                self.common_attributes.title = Some(title);
                self
            }
        }
    };
}

use crate::{SvgColor, SvgStrokeLinecap, SvgTransform};
pub(crate) use implement_common_attributes;
