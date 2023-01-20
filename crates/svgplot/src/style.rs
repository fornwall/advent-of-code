use crate::escape::escape_xml;
use crate::{SvgElement, SvgId};
use std::io::Write;

#[derive(Default)]
pub struct SvgStyle {
    pub content: String,
}

impl From<SvgStyle> for SvgElement {
    fn from(value: SvgStyle) -> Self {
        Self::Style(value)
    }
}

impl SvgStyle {
    pub const fn new(content: String) -> Self {
        Self { content }
    }
    pub(crate) fn write<W: Write>(&self, id: Option<SvgId>, buffer: &mut W) {
        #![allow(clippy::unwrap_used)]
        buffer.write_all(b"<style").unwrap();
        if let Some(id) = id {
            id.write(buffer);
        }
        buffer.write_all(b">").unwrap();
        buffer
            .write_all(escape_xml(&self.content).as_bytes())
            .unwrap();
        buffer.write_all(b"</style>").unwrap();
    }
}
