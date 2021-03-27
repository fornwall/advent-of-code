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
                    result.state = State::Down
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
        .map_err(|e| format!("Invalid input - {}", e.to_string()))
        .and_then(|value| {
            if value == 0 {
                Err("Invalid input 0".to_string())
            } else {
                Ok(value)
            }
        })
}

pub fn part1(input_string: &str) -> Result<i32, String> {
    let input_square = parse(input_string)?;
    Square::iter()
        .nth(input_square - 1)
        .map(|walker| walker.x.abs() + walker.y.abs())
        .ok_or_else(|| "No solution found".to_string())
}

pub fn part2(input_string: &str) -> Result<usize, String> {
    let puzzle_input = parse(input_string)?;

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

#[test]
fn test_part1() {
    assert_eq!(Ok(3), part1("12"));
    assert_eq!(Ok(2), part1("23"));
    assert_eq!(Ok(31), part1("1024"));
    assert_eq!(Ok(480), part1(include_str!("day03_input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(Ok(349_975), part2(include_str!("day03_input.txt")));
}
