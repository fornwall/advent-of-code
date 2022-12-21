use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Rational {
    pub x: i64,
    pub y: i64,
}

impl Rational {
    fn gcd<T: Copy + Clone + PartialEq + Rem<Output = T> + From<u8>>(x: T, y: T) -> T {
        if y == 0.into() {
            x
        } else {
            Self::gcd(y, x % y)
        }
    }

    pub const fn integer(x: i64) -> Self {
        Self { x, y: 1 }
    }

    pub fn new(x: i64, y: i64) -> Self {
        assert_ne!(y, 0);
        let g = Self::gcd(i64::abs(x), i64::abs(y));

        Self { x: x / g, y: y / g }
    }

    pub fn new_i128(x: i128, y: i128) -> Self {
        assert_ne!(y, 0);
        let g = Self::gcd(i128::abs(x), i128::abs(y));

        Self {
            x: (x / g) as i64,
            y: (y / g) as i64,
        }
    }

    pub fn int_value(self) -> Option<i64> {
        (self.y.abs() == 1).then_some(self.x / self.y)
    }
}

impl Add for Rational {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x * other.y + self.y * other.x, self.y * other.y)
    }
}

impl Add<i64> for Rational {
    type Output = Self;
    fn add(self, other: i64) -> Self {
        Self::add(self, Self::new(other, 1))
    }
}

impl Add<Rational> for i64 {
    type Output = Rational;
    fn add(self, other: Rational) -> Rational {
        Rational::add(Rational::new(self, 1), other)
    }
}

impl Sub for Rational {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x * other.y - self.y * other.x, self.y * other.y)
    }
}

impl Sub<i64> for Rational {
    type Output = Self;

    fn sub(self, other: i64) -> Self {
        Self::sub(self, Self::new(other, 1))
    }
}

impl Sub<Rational> for i64 {
    type Output = Rational;

    fn sub(self, other: Rational) -> Rational {
        Rational::sub(Rational::new(self, 1), other)
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new_i128(
            i128::from(self.x) * i128::from(other.x),
            i128::from(self.y * other.y),
        )
    }
}

impl Mul<i64> for Rational {
    type Output = Self;

    fn mul(self, other: i64) -> Self {
        Self::mul(self, Self::new(other, 1))
    }
}

impl Mul<Rational> for i64 {
    type Output = Rational;

    fn mul(self, other: Rational) -> Rational {
        Rational::mul(Rational::new(self, 1), other)
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new_i128(
            i128::from(self.x) * i128::from(other.y),
            i128::from(self.y * other.x),
        )
    }
}

impl Div<i64> for Rational {
    type Output = Self;

    fn div(self, other: i64) -> Self {
        Self::div(self, Self::new(other, 1))
    }
}

impl Div<Rational> for i64 {
    type Output = Rational;

    fn div(self, other: Rational) -> Rational {
        Rational::div(Rational::new(self, 1), other)
    }
}
