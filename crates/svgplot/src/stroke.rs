use std::io::Write;

pub enum SvgStrokeLinecap {
    Butt,
    Round,
    Square,
}

impl SvgStrokeLinecap {
    pub(crate) fn write(&self, writer: &mut dyn Write) {
        #![allow(clippy::unwrap_used)]
        writer.write_all(b" stroke-linecap=\"").unwrap();
        writer
            .write_all(match self {
                Self::Butt => b"butt",
                Self::Round => b"round",
                Self::Square => b"square",
            })
            .unwrap();
        writer.write_all(b"\"").unwrap();
    }
}
