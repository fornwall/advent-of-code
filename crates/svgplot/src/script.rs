use crate::escape::escape_text;
use crate::{SvgElement, SvgId};
use std::io::Write;

pub struct SvgScript {
    script: String,
}

impl From<SvgScript> for SvgElement {
    fn from(value: SvgScript) -> Self {
        Self::Script(value)
    }
}

impl SvgScript {
    pub const fn new(script: String) -> Self {
        Self { script }
    }

    pub(crate) fn write<W: Write>(&self, id: Option<SvgId>, writer: &mut W) {
        #![allow(clippy::unwrap_used)]
        writer.write_all(b"<script").unwrap();
        if let Some(id) = id {
            id.write(writer);
        }
        writer
            .write_all(format!(">{}</script>\n", escape_text(&self.script)).as_bytes())
            .unwrap();
    }
}
