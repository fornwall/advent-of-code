use advent_of_code::solve;
use std::fs::read_to_string;


fn problem_2020_5_1() {
            #![allow(clippy::unwrap_used)]
  let input = read_to_string("src/year2020/day05_input.txt").unwrap();
  solve(2020, 5, 1, &input).unwrap();
}

fn problem_2020_5_2() {
            #![allow(clippy::unwrap_used)]
  let input = read_to_string("src/year2020/day05_input.txt").unwrap();
  solve(2020, 5, 2, &input).unwrap();
}

iai::main!(problem_2020_5_1, problem_2020_5_2);
