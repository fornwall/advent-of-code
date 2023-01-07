use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    input
        .text
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            if !(line.len() == 3
                && (b'A'..=b'C').contains(&line[0])
                && (b'X'..=b'Z').contains(&line[2]))
            {
                return Err("Invalid input".to_string());
            }

            let other_shape = (line[0] - b'A') as i8;
            let second_value = (line[2] - b'X') as i8;

            let my_shape =
                input.part_values(second_value, (other_shape - 1 + second_value).rem_euclid(3));

            let my_shape_score = 1 + my_shape as u32;
            let my_outcome_score = match (other_shape - my_shape).rem_euclid(3) {
                2 => 6,
                1 => 0,
                _ => 3_u32,
            };
            Ok(my_shape_score + my_outcome_score)
        })
        .sum()
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "A Y\nB X\nC Z";
    test_part_one!(test_input => 15);

    let real_input = include_str!("day02_input.txt");
    test_part_one!(real_input => 11063);
    test_part_two!(real_input => 10349);
}

#[cfg(feature = "count-allocations")]
#[test]
pub fn no_memory_allocations() {
    use crate::input::{test_part_one, test_part_two};
    let real_input = include_str!("day02_input.txt");
    let allocations = allocation_counter::count(|| {
        test_part_one!(real_input => 11063);
        test_part_two!(real_input => 10349);
    });
    assert_eq!(allocations, 0);
}
