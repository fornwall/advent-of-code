use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAX_DAYS: usize = 9;

    // Indexed by days left mapping to number of fishes with that many days left:
    let mut count_per_day_left = [0; MAX_DAYS];

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

    for day in 0..input.part_values(80, 256) {
        // Those with 0 days left have given birth to new ones with 8 days
        // left - but we need to also add them back (reset to 6 days left):
        count_per_day_left[(7 + day) % MAX_DAYS] += count_per_day_left[day % MAX_DAYS];
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
