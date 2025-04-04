use std::collections::{HashMap, HashSet};

use crate::input::Input;

#[derive(Clone)]
enum TrackPiece {
    // ⌜
    TopLeft,
    // ⌝
    TopRight,
    // ⌞
    BottomLeft,
    // ⌟
    BottomRight,
    // +
    Intersection,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
struct Vector {
    y: i16,
    x: i16,
}

impl Vector {
    const fn is_outside_track(self) -> bool {
        self.x < 0 || self.y < 0 || self.x > 1000 || self.y > 1000
    }

    const fn add(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
    const fn turn_left(&mut self) {
        let tmp = self.x;
        self.x = self.y;
        self.y = -tmp;
    }
    const fn turn_right(&mut self) {
        let tmp = self.x;
        self.x = -self.y;
        self.y = tmp;
    }
}

struct Cart {
    direction: Vector,
    turns: u8,
    position: Vector,
}

impl Cart {
    const fn new(position: Vector, direction_x: i16, direction_y: i16) -> Self {
        Self {
            direction: Vector {
                x: direction_x,
                y: direction_y,
            },
            turns: 0,
            position,
        }
    }

    const fn advance(&mut self) -> bool {
        self.position.add(self.direction);
        self.position.is_outside_track()
    }

    const fn on_enter(&mut self, piece: &TrackPiece) {
        match *piece {
            TrackPiece::TopRight | TrackPiece::BottomLeft => {
                self.direction = Vector {
                    x: self.direction.y,
                    y: self.direction.x,
                };
            }
            TrackPiece::TopLeft | TrackPiece::BottomRight => {
                self.direction = Vector {
                    x: -self.direction.y,
                    y: -self.direction.x,
                };
            }
            TrackPiece::Intersection => {
                match self.turns {
                    0 => {
                        self.direction.turn_left();
                    }
                    2 => {
                        self.direction.turn_right();
                    }
                    _ => {}
                };
                self.turns = (self.turns + 1) % 3;
            }
        }
    }
}

struct Track {
    track: HashMap<Vector, TrackPiece>,
    carts: Vec<Cart>,
}

impl Track {
    const MAX_TICKS: u32 = 100_000;

    fn parse(input_string: &str) -> Result<Self, String> {
        let mut carts = Vec::new();
        let mut track = HashMap::new();
        let mut verticals = HashSet::new();
        for (y, line) in input_string.lines().enumerate() {
            for (x, c) in line.as_bytes().iter().enumerate() {
                let position = Vector {
                    x: x as i16,
                    y: y as i16,
                };
                match c {
                    b'^' => {
                        carts.push(Cart::new(position, 0, -1));
                        verticals.insert(position);
                    }
                    b'v' => {
                        carts.push(Cart::new(position, 0, 1));
                        verticals.insert(position);
                    }
                    b'<' => {
                        carts.push(Cart::new(position, -1, 0));
                    }
                    b'>' => {
                        carts.push(Cart::new(position, 1, 0));
                    }
                    b'+' => {
                        track.insert(position, TrackPiece::Intersection);
                    }
                    b'|' => {
                        verticals.insert(position);
                    }
                    b'/' => {
                        let piece = if verticals.contains(&Vector {
                            x: position.x,
                            y: position.y - 1,
                        }) {
                            TrackPiece::BottomRight
                        } else {
                            TrackPiece::TopLeft
                        };
                        track.insert(position, piece);
                    }
                    b'\\' => {
                        let piece = if verticals.contains(&Vector {
                            x: position.x,
                            y: position.y - 1,
                        }) {
                            TrackPiece::BottomLeft
                        } else {
                            TrackPiece::TopRight
                        };
                        track.insert(position, piece);
                    }
                    b'-' | b' ' => {
                        // Ignore
                    }
                    _ => {
                        return Err(format!("Invalid char: {c}"));
                    }
                }

                if carts.len() > 32 {
                    return Err("Too many carts".to_string());
                }
            }
        }
        Ok(Self { track, carts })
    }

    fn find_position(&mut self, part1: bool) -> Result<Vector, String> {
        for _ in 0..Self::MAX_TICKS {
            self.carts
                .sort_unstable_by(|a, b| a.position.cmp(&b.position));

            let mut cart_idx = 0;
            'outer: while cart_idx < self.carts.len() {
                if self.carts[cart_idx].advance() {
                    return Err("Cart ends up outside track".to_string());
                }

                for other_cart_idx in 0..self.carts.len() {
                    if cart_idx == other_cart_idx {
                        continue;
                    }
                    if self.carts[cart_idx].position == self.carts[other_cart_idx].position {
                        if part1 {
                            return Ok(self.carts[cart_idx].position);
                        }
                        self.carts.remove(std::cmp::max(cart_idx, other_cart_idx));
                        self.carts.remove(std::cmp::min(cart_idx, other_cart_idx));
                        if other_cart_idx < cart_idx {
                            cart_idx -= 1;
                        }
                        continue 'outer;
                    }
                }

                if let Some(piece) = self.track.get(&self.carts[cart_idx].position) {
                    self.carts[cart_idx].on_enter(piece);
                }

                cart_idx += 1;
            }

            if self.carts.len() == 1 {
                return Ok(self.carts[0].position);
            }
        }

        Err(format!("No solution found in {} ticks", Self::MAX_TICKS))
    }
}

pub fn solve(input: &Input) -> Result<String, String> {
    let mut track = Track::parse(input.text)?;
    let position = track.find_position(input.is_part_one())?;
    Ok(format!("{},{}", position.x, position.y))
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};

    test_part_one!(
            "|
v
|
|
|
^
|"
        => "0,3".into());

    test_part_one!(
            r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/"
        => "7,3".into());

    test_part_two!(
            r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
"
        => "6,4".into());

    let input = include_str!("day13_input.txt");
    test_part_one!(
        input => "65,73".into());
    test_part_two!(
        input => "54,66".into());

    test_part_one_error!(
            "|
^
v
|"
        => "Cart ends up outside track");
}
