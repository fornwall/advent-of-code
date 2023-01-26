use crate::input::Input;
use std::cmp::max;
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum State {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Copy, Clone)]
struct Square {
    x: i32,
    y: i32,
    state: State,
}

impl Square {
    fn on_corner(&self) -> bool {
        let current_spiral = max(self.x.abs(), self.y.abs());
        self.x.abs() == current_spiral && self.y.abs() == current_spiral
    }

    const fn iter() -> SquareIterator {
        SquareIterator {
            current: Self {
                x: 0,
                y: 0,
                state: State::Right,
            },
        }
    }
}

struct SquareIterator {
    current: Square,
}

impl Iterator for SquareIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        let orig = self.current;
        let mut result = self.current;
        match result.state {
            State::Up => {
                result.y += 1;
                if result.on_corner() {
                    result.state = State::Left;
                }
            }
            State::Right => {
                if result.y == 0 || (result.on_corner() && result.x > 0) {
                    result.state = State::Up;
                }
                result.x += 1;
            }
            State::Down => {
                result.y -= 1;
                if result.on_corner() {
                    result.state = State::Right;
                }
            }
            State::Left => {
                result.x -= 1;
                if result.on_corner() {
                    result.state = State::Down;
                }
            }
        }
        self.current = result;
        Some(orig)
    }
}

fn parse(input_string: &str) -> Result<usize, String> {
    input_string
        .parse::<usize>()
        .map_err(|e| format!("Invalid input - {e}"))
        .and_then(|value| {
            if value == 0 {
                Err("Invalid input 0".to_string())
            } else {
                Ok(value)
            }
        })
}

pub fn solve(input: &Input) -> Result<usize, String> {
    let puzzle_input = parse(input.text)?;
    if input.is_part_one() {
        Square::iter()
            .nth(puzzle_input - 1)
            .map(|walker| (walker.x.abs() + walker.y.abs()) as usize)
            .ok_or_else(|| "No solution found".to_string())
    } else {
        let mut square_values = HashMap::new();
        square_values.insert((0, 0), 1);

        Square::iter()
            .map(|walker| {
                let new_square_value = (-1..=1)
                    .flat_map(move |dx| (-1..=1).map(move |dy| (dx, dy)))
                    .map(|(dx, dy)| (walker.x + dx, walker.y + dy))
                    .filter_map(|neighbor_square| square_values.get(&neighbor_square))
                    .sum();
                square_values.insert((walker.x, walker.y), new_square_value);
                new_square_value
            })
            .find(|&new_square_value| new_square_value > puzzle_input)
            .ok_or_else(|| "No solution found".to_string())
    }
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};
    test_part_one!("12" => 3);
    test_part_one!("23" => 2);
    test_part_one!("1024" => 31);
    test_part_one!("1024" => 31);

    let input = include_str!("day03_input.txt");
    test_part_one!(input => 480);
    test_part_two!(input => 349_975);
}
