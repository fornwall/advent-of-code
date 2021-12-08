use crate::input::Input;
use std::collections::VecDeque;

pub fn solve(input: &mut Input) -> Result<u64, String> {
    // Indexed by days left mapping to number of fishes with that many days left:
    let mut count_per_day_left = VecDeque::from([0; 9]);

    for day_left_str in input.text.split(',') {
        match day_left_str.parse::<u8>() {
            Ok(day_left) if day_left <= 8 => {
                count_per_day_left[day_left as usize] += 1;
            }
            _ => {
                return Err(
                    "Input is not comma-separated list of integers in the range [0,8].".to_string(),
                );
            }
        }
    }

    for _day in 0..input.part_values(80, 256) {
        count_per_day_left.rotate_left(1);
        // Those with 0 days left have given birth to new ones with 8 days
        // left - but we need to also add them back (reset to 6 days left):
        count_per_day_left[6] += count_per_day_left[8];
    }

    Ok(count_per_day_left.iter().sum())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "3,4,3,1,2";
    test_part_one!(example => 5934);
    test_part_two!(example => 26_984_457_539);

    let real_input = include_str!("day06_input.txt");
    test_part_one!(real_input => 387_413);
    test_part_two!(real_input => 1_738_377_086_345);
}
