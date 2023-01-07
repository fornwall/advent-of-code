use std::io::Write;

use crate::common_attributes::{define_element, CommonAttributes};
use crate::{SvgElement, SvgId};

pub struct SvgGroup {
    pub(crate) elements: Vec<SvgElement>,
    pub(crate) common_attributes: CommonAttributes,
}

impl From<SvgGroup> for SvgElement {
    fn from(value: SvgGroup) -> Self {
        Self::Group(value)
    }
}

define_element!(SvgGroup);

impl SvgGroup {
    pub const fn new() -> Self {
        Self {
            elements: Vec::new(),
            common_attributes: CommonAttributes::new(),
        }
    }

    pub fn with_elements<E: Into<SvgElement>>(elements: Vec<E>) -> Self {
        Self {
            elements: elements.into_iter().map(Into::into).collect(),
            common_attributes: CommonAttributes::new(),
        }
    }

    pub fn add<E: Into<SvgElement>>(&mut self, element: E) -> &mut Self {
        self.elements.push(element.into());
        self
    }

    pub(crate) fn write<W: Write>(&self, id: Option<SvgId>, writer: &mut W) {
        #![allow(clippy::unwrap_used)]
        writer.write_all(b"<g").unwrap();
        if let Some(id) = id {
            id.write(writer);
        }
        self.common_attributes.write(writer);
        writer.write_all(b">\n").unwrap();
        for element in &self.elements {
            element.write(None, writer);
        }
        writer.write_all(b"</g>\n").unwrap();
    }
}
