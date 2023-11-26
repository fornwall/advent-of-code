use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct U256 {
    pub low: u128,
    pub high: u128,
}

impl U256 {
    pub fn set_bit(&mut self, offset: usize) {
        if offset < 128 {
            self.low |= 1 << offset;
        } else {
            self.high |= 1 << (offset - 128);
        }
    }

    pub const fn non_zero(&self) -> bool {
        self.low != 0 || self.high != 0
    }

    pub const fn is_bit_set(&self, offset: usize) -> bool {
        if offset < 128 {
            (self.low & 1 << offset) != 0
        } else {
            (self.high & 1 << (offset - 128)) != 0
        }
    }

    pub const fn count_ones(&self) -> u32 {
        self.low.count_ones() + self.high.count_ones()
    }

    /*
    pub const fn leading_zeros(&self) -> u32 {
        if self.high != 0 {
            self.high.leading_zeros()
        } else {
            128 + self.low.leading_zeros()
        }
    }

    pub const fn trailing_zeros(&self) -> u32 {
        if self.low != 0 {
            self.low.trailing_zeros()
        } else {
            self.high.trailing_zeros() + 128
        }
    }

    pub const fn left_shift(&self) -> Self {
        Self {
            low: (self.low << 1) | (self.high >> 127),
            high: (self.high << 1),
        }
    }

    pub const fn right_shift(&self) -> Self {
        Self {
            low: (self.low >> 1),
            high: (self.low << 127) | (self.high >> 1),
        }
    }
     */

    pub const fn shift_left(self, width: usize) -> Self {
        if width <= 128 {
            let mask = if width == 128 { !0 } else { !(1 << width) };
            let low = ((self.low << 1) & mask) | (self.low >> (width - 1));
            Self {
                high: self.high,
                low,
            }
        } else {
            // abcd efgh -> bcde efga
            let mask = !(1 << (width - 128));
            let high = ((self.high << 1) & mask) | (self.low >> 127);
            let low = (self.low << 1) | (self.high >> (width - 129));
            Self { high, low }
        }
    }

    pub const fn shift_right(self, width: usize) -> Self {
        if width <= 128 {
            let low = (self.low >> 1) | ((self.low & 1) << (width - 1));
            Self {
                high: self.high,
                low,
            }
        } else {
            let high = (self.high >> 1) | ((self.low & 1) << (width - 129));
            let low = (self.low >> 1) | ((self.high & 1) << 127);
            Self { high, low }
        }
    }
}

impl BitAnd for U256 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self {
            low: self.low & rhs.low,
            high: self.high & rhs.high,
        }
    }
}

impl BitOr for U256 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self {
            low: self.low | rhs.low,
            high: self.high | rhs.high,
        }
    }
}

impl Not for U256 {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            low: !self.low,
            high: !self.high,
        }
    }
}

impl BitOrAssign for U256 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.low |= rhs.low;
        self.high |= rhs.high;
    }
}

impl BitAndAssign for U256 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.low &= rhs.low;
        self.high &= rhs.high;
    }
}

#[test]
fn basics() {
    let mut val = U256::default();
    val.set_bit(0);
    assert_eq!(val.high, 0);
    assert_eq!(val.low, 1);

    let mut val = U256::default();
    val.set_bit(127);
    assert_eq!(val.high, 0);
    assert_eq!(val.low, 1 << 127);

    let mut val = U256::default();
    val.set_bit(128);
    assert_eq!(val.high, 1);
    assert_eq!(val.low, 0);

    let mut val = U256::default();
    val.set_bit(255);
    assert_eq!(val.high, 1 << 127);
    assert_eq!(val.low, 0);
}

#[test]
pub fn test_shift_left() {
    let mut val = U256 { low: 0, high: 0 };
    val.set_bit(0);
    assert_eq!(val.low, 1);
    assert_eq!(val.high, 0);
    val = val.shift_left(10);
    assert_eq!(val.low, 2);
    assert_eq!(val.high, 0);
    val = val.shift_left(10);
    assert_eq!(val.low, 4);
    assert_eq!(val.high, 0);
    val = val.shift_left(4);
    assert_eq!(val.low, 8);
    assert_eq!(val.high, 0);
    val = val.shift_left(4);
    assert_eq!(val.low, 1);
    assert_eq!(val.high, 0);

    val.low = 1 << 127;
    val = val.shift_left(128);
    assert_eq!(val.low, 1);
    assert_eq!(val.high, 0);

    val.low = 1 << 127;
    val = val.shift_left(129);
    assert_eq!(val.low, 0);
    assert_eq!(val.high, 1);
    val = val.shift_left(129);
    assert_eq!(val.low, 1);
    assert_eq!(val.high, 0);
}

#[test]
pub fn test_shift_right() {
    let mut val = U256 { low: 0, high: 0 };
    val.set_bit(0);
    assert_eq!(val.low, 1);
    assert_eq!(val.high, 0);
    val = val.shift_right(4);
    assert_eq!(val.low, 8);
    assert_eq!(val.high, 0);
    val = val.shift_right(4);
    assert_eq!(val.low, 4);
    assert_eq!(val.high, 0);
    val.low = 1;
    val = val.shift_right(128);
    assert_eq!(val.low, 1 << 127);
    assert_eq!(val.high, 0);
    val.low = 1;
    val.high = 0;
    val = val.shift_right(129);
    assert_eq!(val.low, 0);
    assert_eq!(val.high, 1);
    val = val.shift_right(129);
    assert_eq!(val.low, 1 << 127);
    assert_eq!(val.high, 0);
}
