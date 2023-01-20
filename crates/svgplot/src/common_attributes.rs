use std::io::Write;

use crate::escape::escape_xml;

/// Common attributes to all SVG elements.
///
/// https://oreillymedia.github.io/Using_SVG/guide/markup.html#common-attributes
#[derive(Default)]
pub(crate) struct CommonAttributes {
    pub(crate) style: Option<String>,
    pub(crate) classes: Vec<String>,
}

impl CommonAttributes {
    pub(crate) const fn new() -> Self {
        Self {
            style: None,
            classes: Vec::new(),
        }
    }
    pub(crate) fn write<W: Write>(&self, writer: &mut W) {
        #![allow(clippy::unwrap_used)]
        if let Some(style) = &self.style {
            writer
                .write_all(format!(" style=\"{}\"", escape_xml(style)).as_bytes())
                .unwrap();
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

macro_rules! define_element {
    ($element_name:ident) => {
        impl $element_name {
            pub fn style<S: ToString>(mut self, style: S) -> Self {
                self.common_attributes.style = Some(style.to_string());
                self
            }
        }
    };
}

pub(crate) use define_element;
