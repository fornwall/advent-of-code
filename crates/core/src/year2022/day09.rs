use crate::input::Input;
use std::collections::HashSet;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    let mut snake = vec![(0_i32, 0_i32); input.part_values(2, 10)];
    let mut visited = HashSet::new();
    visited.insert((0_i32, 0_i32));

    for line in input.text.lines() {
        let steps = line[2..]
            .parse::<i32>()
            .map_err(|_| "Not an integer for steps".to_string())?;

        for _ in 0..steps {
            match line.as_bytes()[0] {
                b'U' => snake[0].1 -= 1,
                b'R' => snake[0].0 += 1,
                b'D' => snake[0].1 += 1,
                _ => snake[0].0 -= 1,
            };

            for i in 1..snake.len() {
                if snake[i - 1].0.abs_diff(snake[i].0) > 1
                    || snake[i - 1].1.abs_diff(snake[i].1) > 1
                {
                    snake[i].0 += (snake[i - 1].0 - snake[i].0).signum();
                    snake[i].1 += (snake[i - 1].1 - snake[i].1).signum();
                    if i + 1 == snake.len() {
                        visited.insert(snake[i]);
                    }
                }
            }
        }
    }

    Ok(visited.len())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    test_part_one!(test_input => 13);
    test_part_two!(test_input => 1);

    let test_input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
    test_part_two!(test_input => 36);

    let real_input = include_str!("day09_input.txt");
    test_part_one!(real_input => 6081);
    test_part_two!(real_input => 2487);
}
