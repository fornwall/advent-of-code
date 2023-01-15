use std::io::Write;

/// Common attributes to all SVG elements.
///
/// https://oreillymedia.github.io/Using_SVG/guide/markup.html#common-attributes
pub(crate) struct CommonAttributes {
    pub(crate) style: Option<String>,
}

impl CommonAttributes {
    pub(crate) const fn new() -> Self {
        Self { style: None }
    }
    pub(crate) fn write<W: Write>(&self, writer: &mut W) {
        #![allow(clippy::unwrap_used)]
        if let Some(style) = &self.style {
            writer
                .write_all(format!(" style=\"{}\"", escape_xml(style)).as_bytes())
                .unwrap();
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

use crate::escape::escape_xml;
pub(crate) use define_element;
