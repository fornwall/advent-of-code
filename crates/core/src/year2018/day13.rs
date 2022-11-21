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
    fn add(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
    fn turn_left(&mut self) {
        let tmp = self.x;
        self.x = self.y;
        self.y = -tmp;
    }
    fn turn_right(&mut self) {
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
    fn advance(&mut self) {
        self.position.add(self.direction);
    }

    fn on_enter(&mut self, piece: &TrackPiece) {
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
    cart_positions: HashSet<Vector>,
}

impl Track {
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
                        return Err(format!("Invalid char: {}", c));
                    }
                }
            }
        }
        Ok(Self {
            track,
            carts,
            cart_positions: HashSet::new(),
        })
    }

    fn find_crash(&mut self) -> Vector {
        loop {
            self.carts.sort_by(|a, b| a.position.cmp(&b.position));

            for cart in self.carts.iter_mut() {
                self.cart_positions.remove(&cart.position);

                cart.advance();

                if !self.cart_positions.insert(cart.position) {
                    return cart.position;
                }

                if let Some(piece) = self.track.get(&cart.position) {
                    cart.on_enter(piece);
                }
            }
        }
    }

    fn find_remaining(&mut self) -> Vector {
        loop {
            self.carts.sort_by(|a, b| a.position.cmp(&b.position));

            let mut removed_positions = HashSet::new();
            for cart in self.carts.iter_mut() {
                if removed_positions.contains(&cart.position) {
                    continue;
                }

                self.cart_positions.remove(&cart.position);

                cart.advance();

                if self.cart_positions.remove(&cart.position) {
                    removed_positions.insert(cart.position);
                    continue;
                } else {
                    self.cart_positions.insert(cart.position);
                };

                if let Some(piece) = self.track.get(&cart.position) {
                    cart.on_enter(piece);
                }
            }

            if !removed_positions.is_empty() {
                self.carts
                    .retain(|cart| !removed_positions.contains(&cart.position));
                if self.carts.len() == 1 {
                    return self.carts[0].position;
                }
            }
        }
    }
}

pub fn solve(input: &mut Input) -> Result<String, String> {
    let mut track = Track::parse(input.text)?;
    let position = if input.is_part_one() {
        track.find_crash()
    } else {
        track.find_remaining()
    };
    Ok(format!("{},{}", position.x, position.y))
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};

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
            r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/"#
        => "7,3".into());

    test_part_two!(
            r#"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
"#
        => "6,4".into());

    let input = include_str!("day13_input.txt");
    test_part_one!(
        input => "65,73".into());
    test_part_two!(
        input => "54,66".into());
}
