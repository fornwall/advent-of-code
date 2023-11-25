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
            let moved = row.moving_east_bits.shift_left(width) & !row.bits();
            any_cucumber_moved |= moved.non_zero();
            let stay = row.moving_east_bits & !moved.shift_right(width);
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
