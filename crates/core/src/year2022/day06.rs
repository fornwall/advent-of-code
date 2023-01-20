use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    let transmission = input.text.as_bytes();
    let marker_len = input.part_values(4, 14);

    let mut last_idx_of_char = [0_usize; 256];
    let mut distinct_start_idx = 0;

    for (i, &char_at_offset_i) in transmission.iter().enumerate() {
        distinct_start_idx = distinct_start_idx.max(last_idx_of_char[char_at_offset_i as usize]);

        last_idx_of_char[char_at_offset_i as usize] = i;

        if i - distinct_start_idx == marker_len {
            return Ok(i + 1);
        }
    }

    Err("No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};

    let test_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    test_part_one!(test_input => 7);
    test_part_two!(test_input => 19);

    let real_input = include_str!("day06_input.txt");
    test_part_one!(real_input => 1109);
    test_part_two!(real_input => 3965);

    test_part_one_error!("abc" => "No solution found");
    test_part_one_error!("abcc" => "No solution found");

    #[cfg(feature = "count-allocations")]
    {
        let allocations = allocation_counter::count(|| {
            test_part_one!(real_input => 1109);
            test_part_two!(real_input => 3965);
        });
        assert_eq!(allocations, 0);
    }
}
