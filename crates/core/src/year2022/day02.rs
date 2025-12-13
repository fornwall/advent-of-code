use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    let scores_indexed_by_input =
        input.part_values([4, 8, 3, 1, 5, 9, 7, 2, 6], [3, 4, 8, 1, 5, 9, 2, 6, 7]);

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

            let other_shape = (line[0] - b'A') as usize;
            let second_value = (line[2] - b'X') as usize;
            let score_index = 3 * other_shape + second_value;
            Ok(scores_indexed_by_input[score_index])
        })
        .sum()
}

#[test]
pub fn tests() {
    let test_input = "A Y\nB X\nC Z";
    test_part_one_no_allocations!(test_input => 15);

    let real_input = include_str!("day02_input.txt");
    test_part_one_no_allocations!(real_input => 11063);
    test_part_two_no_allocations!(real_input => 10349);
}
