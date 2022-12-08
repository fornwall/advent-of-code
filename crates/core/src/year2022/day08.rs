use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    let grid = input
        .text
        .bytes()
        .filter(u8::is_ascii_digit)
        .map(|b| b - b'0')
        .collect::<Vec<_>>();

    let grid_size = (grid.len() as f64).sqrt() as usize;
    if grid_size * grid_size != grid.len() {
        return Err("Non-rectangular grid".to_string());
    }

    let mut num_visible = 0;
    let mut best_scenic_score = 0;

    'start: for start in 0..grid.len() {
        let mut scenic_score = 1;
        'stride: for (stride, steps) in [
            (-1, start % grid_size),
            (1, grid_size - start % grid_size - 1),
            (-(grid_size as i32), start / grid_size),
            ((grid_size as i32), grid_size - start / grid_size - 1),
        ] {
            let mut position = start as i32;
            let start_height = grid[start];

            for step in 1..=steps {
                position += stride;
                let height_at_position = grid[position as usize];
                if height_at_position >= start_height {
                    scenic_score *= step;
                    continue 'stride;
                }
            }

            if input.is_part_one() {
                num_visible += 1;
                continue 'start;
            }
            scenic_score *= steps;
        }

        best_scenic_score = best_scenic_score.max(scenic_score);
    }

    Ok(input.part_values(num_visible, best_scenic_score))
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "30373
25512
65332
33549
35390";
    test_part_one!(test_input => 21);
    test_part_two!(test_input => 8);

    let real_input = include_str!("day08_input.txt");
    test_part_one!(real_input => 1_672);
    test_part_two!(real_input => 327_180);
}
