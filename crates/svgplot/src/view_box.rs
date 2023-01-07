use crate::SvgInteger;

/// The viewBox attribute defines the position and dimension, in user space, of an SVG viewport.
///
/// The value of the viewBox attribute is a list of four numbers: min-x, min-y, width and height.
/// The numbers, which are separated by whitespace and/or a comma, specify a rectangle in user space
/// which is mapped to the bounds of the viewport established for the associated SVG element
/// (not the browser viewport).
///
/// A viewBox can be constructed out of a tuple:
/// ```rust
/// let view_box: svgplot::ViewBox = (1, 2, 3, 4).into();
/// assert_eq!(view_box.min_x, 1);
/// assert_eq!(view_box.min_y, 2);
/// assert_eq!(view_box.width, 3);
/// assert_eq!(view_box.height, 4);
/// ```
pub struct ViewBox {
    pub min_x: SvgInteger,
    pub min_y: SvgInteger,
    pub width: SvgInteger,
    pub height: SvgInteger,
}

impl From<(SvgInteger, SvgInteger, SvgInteger, SvgInteger)> for ViewBox {
    fn from(value: (SvgInteger, SvgInteger, SvgInteger, SvgInteger)) -> Self {
        Self {
            min_x: value.0,
            min_y: value.1,
            width: value.2,
            height: value.3,
        }
    }
}

#[test]
fn test() {
    let view_box: ViewBox = (1, 2, 3, 4).into();
    assert_eq!(view_box.min_x, 1);
    assert_eq!(view_box.min_y, 2);
    assert_eq!(view_box.width, 3);
    assert_eq!(view_box.height, 4);
}
