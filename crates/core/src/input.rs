#[cfg(all(feature = "visualization", test))]
use crate::painter::MockPainter;
#[cfg(feature = "visualization")]
use crate::painter::PainterRef;

#[derive(Copy, Clone)]
pub enum Part {
    One,
    Two,
}

pub struct Input {
    pub part: Part,
    pub text: String,
    #[cfg(feature = "visualization")]
    pub painter: PainterRef,
}

impl Input {
    pub const fn is_part_one(&self) -> bool {
        matches!(self.part, Part::One)
    }

    pub fn part_values<T>(&self, if_part_one: T, if_part_two: T) -> T {
        // See https://github.com/rust-lang/rust/issues/66753 for missing_const_for_fn.
        #![allow(clippy::missing_const_for_fn)]
        match self.part {
            Part::One => if_part_one,
            Part::Two => if_part_two,
        }
    }

    #[cfg(test)]
    pub fn part_one(text: &str) -> Self {
        Self {
            part: Part::One,
            text: text.to_string(),
            #[cfg(feature = "visualization")]
            painter: Box::new(MockPainter {}),
        }
    }

    #[cfg(test)]
    pub fn part_two(text: &str) -> Self {
        Self {
            part: Part::Two,
            text: text.to_string(),
            #[cfg(feature = "visualization")]
            painter: Box::new(MockPainter {}),
        }
    }
}

#[macro_export]
macro_rules! test_part_one {
    ($input:tt => $expected:tt) => {
        assert_eq!(Ok($expected), solve(&mut Input::part_one($input)));
    };
}

#[macro_export]
macro_rules! test_part_two {
    ($input:tt => $expected:tt) => {
        assert_eq!(Ok($expected), solve(&mut Input::part_two($input)));
    };
}

#[macro_export]
macro_rules! test_part_one_error {
    ($input:tt => $expected:tt) => {
        assert_eq!(Err($expected.into()), solve(&mut Input::part_one($input)));
    };
}

#[macro_export]
macro_rules! test_part_two_error {
    ($input:tt => $expected:tt) => {
        assert_eq!(Err($expected.into()), solve(&mut Input::part_two($input)));
    };
}
