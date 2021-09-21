use crate::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut row = input
        .text
        .as_bytes()
        .iter()
        .map(|&b| b == b'^')
        .collect::<Vec<_>>();

    let mut safe_count = row.iter().filter(|&&b| !b).count();
    let mut next_row = vec![false; row.len()];

    for _count in 1..input.part_values(40, 400_000) {
        for i in 0..next_row.len() {
            let left_is_trap = if i == 0 { false } else { row[i - 1] };
            let center_is_trap = row[i];
            let right_is_trap = if i == next_row.len() - 1 {
                false
            } else {
                row[i + 1]
            };

            // "Then, a new tile is a trap only in one of the following situations":
            // - Its left and center tiles are traps, but its right tile is not.
            // - Its center and right tiles are traps, but its left tile is not.
            // - Only its left tile is a trap.
            // - Only its right tile is a trap."
            next_row[i] = matches!(
                (left_is_trap, center_is_trap, right_is_trap),
                (true, true, false)
                    | (false, true, true)
                    | (true, false, false)
                    | (false, false, true)
            );
            if !next_row[i] {
                safe_count += 1;
            }
        }

        std::mem::swap(&mut row, &mut next_row);
    }

    Ok(safe_count as u32)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day18_input.txt");
    test_part_one!(real_input => 2035);
    test_part_two!(real_input => 20_000_577);
}
