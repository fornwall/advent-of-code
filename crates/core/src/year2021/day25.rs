/// Categories: Bit manipulation
use crate::common::u256::U256;
use crate::input::Input;

#[derive(Copy, Clone)]
struct CucumberRow {
    moving_east_bits: U256,
    moving_south_bits: U256,
}

impl CucumberRow {
    fn bits(self) -> U256 {
        self.moving_east_bits | self.moving_south_bits
    }
}

pub const fn shift_left(this: U256, width: usize) -> U256 {
    if width <= 128 {
        let mask = if width == 128 { !0 } else { !(1 << width) };
        let low = ((this.low << 1) & mask) | (this.low >> (width - 1));
        U256 {
            high: this.high,
            low,
        }
    } else {
        // abcd efgh -> bcde efga
        let mask = !(1 << (width - 128));
        let high = ((this.high << 1) & mask) | (this.low >> 127);
        let low = (this.low << 1) | (this.high >> (width - 129));
        U256 { high, low }
    }
}

pub const fn shift_right(this: U256, width: usize) -> U256 {
    if width <= 128 {
        let low = (this.low >> 1) | ((this.low & 1) << (width - 1));
        U256 {
            high: this.high,
            low,
        }
    } else {
        let high = (this.high >> 1) | ((this.low & 1) << (width - 129));
        let low = (this.low >> 1) | ((this.high & 1) << 127);
        U256 { high, low }
    }
}

pub fn solve(input: &Input) -> Result<u32, String> {
    const MAX_ITERATIONS: u32 = 1000;

    let mut width = 0;

    let mut cucumber_rows = input
        .text
        .lines()
        .map(|line| {
            let mut moving_east_bits = U256::default();
            let mut moving_south_bits = U256::default();
            for (offset, col) in line.bytes().enumerate() {
                match col {
                    b'>' => {
                        moving_east_bits.set_bit(offset);
                        width = width.max(offset + 1);
                    }
                    b'v' => {
                        moving_south_bits.set_bit(offset);
                        width = width.max(offset + 1);
                    }
                    _ => (),
                }
            }
            CucumberRow {
                moving_east_bits,
                moving_south_bits,
            }
        })
        .collect::<Vec<_>>();

    let height = cucumber_rows.len();

    for step in 1..MAX_ITERATIONS {
        let mut any_cucumber_moved = false;

        // "Every step, the sea cucumbers in the east-facing herd attempt to move forward one location":
        for row in cucumber_rows.iter_mut() {
            let moved = shift_left(row.moving_east_bits, width) & !row.bits();
            any_cucumber_moved |= moved.non_zero();
            let stay = row.moving_east_bits & !shift_right(moved, width);
            row.moving_east_bits = moved | stay;
        }

        // "[..] then the sea cucumbers in the south-facing herd attempt to move forward one location":
        let orig_first_row = cucumber_rows[0];
        let mut moving_south_scratch_bits = U256::default();
        for i in 0..height {
            let row_to_the_south = if i == height - 1 {
                orig_first_row
            } else {
                cucumber_rows[i + 1]
            };

            // Everyone who wants to move south and can:
            let moved = cucumber_rows[i].moving_south_bits & !row_to_the_south.bits();
            any_cucumber_moved |= moved.non_zero();
            cucumber_rows[i].moving_south_bits &= !moved;
            cucumber_rows[i].moving_south_bits |= moving_south_scratch_bits;
            moving_south_scratch_bits = moved;
        }
        cucumber_rows[0].moving_south_bits |= moving_south_scratch_bits;

        if !any_cucumber_moved {
            return Ok(step);
        }
    }

    Err(format!("Did not stabilize in {MAX_ITERATIONS} iterations"))
}

#[test]
pub fn tests() {
    use crate::input::test_part_one;

    let example = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
    test_part_one!(example => 58);

    let real_input = include_str!("day25_input.txt");
    test_part_one!(real_input => 582);
}

#[test]
pub fn test_shift_left() {
    let mut val = U256 { low: 0, high: 0 };
    val.set_bit(0);
    assert_eq!(val.low, 1);
    assert_eq!(val.high, 0);
    val = shift_left(val, 10);
    assert_eq!(val.low, 2);
    assert_eq!(val.high, 0);
    val = shift_left(val, 10);
    assert_eq!(val.low, 4);
    assert_eq!(val.high, 0);
    val = shift_left(val, 4);
    assert_eq!(val.low, 8);
    assert_eq!(val.high, 0);
    val = shift_left(val, 4);
    assert_eq!(val.low, 1);
    assert_eq!(val.high, 0);

    val.low = 1 << 127;
    val = shift_left(val, 128);
    assert_eq!(val.low, 1);
    assert_eq!(val.high, 0);

    val.low = 1 << 127;
    val = shift_left(val, 129);
    assert_eq!(val.low, 0);
    assert_eq!(val.high, 1);
    val = shift_left(val, 129);
    assert_eq!(val.low, 1);
    assert_eq!(val.high, 0);
}

#[test]
pub fn test_shift_right() {
    let mut val = U256 { low: 0, high: 0 };
    val.set_bit(0);
    assert_eq!(val.low, 1);
    assert_eq!(val.high, 0);
    val = shift_right(val, 4);
    assert_eq!(val.low, 8);
    assert_eq!(val.high, 0);
    val = shift_right(val, 4);
    assert_eq!(val.low, 4);
    assert_eq!(val.high, 0);
    val.low = 1;
    val = shift_right(val, 128);
    assert_eq!(val.low, 1 << 127);
    assert_eq!(val.high, 0);
    val.low = 1;
    val.high = 0;
    val = shift_right(val, 129);
    assert_eq!(val.low, 0);
    assert_eq!(val.high, 1);
    val = shift_right(val, 129);
    assert_eq!(val.low, 1 << 127);
    assert_eq!(val.high, 0);
}
