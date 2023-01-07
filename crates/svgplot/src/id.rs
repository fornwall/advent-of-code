use std::fmt::{Display, Formatter};
use std::io::Write;

#[derive(Copy, Clone)]
pub struct SvgId {
    pub(crate) value: u32,
}

impl SvgId {
    pub(crate) fn write<W: Write>(self, writer: &mut W) {
        #![allow(clippy::unwrap_used)]
        writer
            .write_all(format!(" id=\"i{}\"", self.value).as_bytes())
            .unwrap();
    }
}

impl Display for SvgId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "i{}", self.value)
    }
}
