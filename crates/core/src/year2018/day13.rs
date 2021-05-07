use std::collections::{HashMap, HashSet};

enum TrackPiece {
    // ⌜
    TopLeft,
    // ⌝
    TopRight,
    // ⌞
    BottomLeft,
    // ⌟
    BottomRight,
    // |
    Vertical,
    // +
    Intersection,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
struct Vector {
    y: i32,
    x: i32,
}

impl Vector {
    fn add(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
    fn turn_left(&mut self) {
        if self.x == 0 {
            self.x = self.y;
            self.y = 0;
        } else {
            self.y = -self.x;
            self.x = 0;
        }
    }
    fn turn_right(&mut self) {
        if self.x == 0 {
            self.x = -self.y;
            self.y = 0;
        } else {
            self.y = self.x;
            self.x = 0;
        }
    }
}

struct Cart {
    direction: Vector,
    turns: i32,
    position: Vector,
}

impl Cart {
    const fn new(position: Vector, direction_x: i32, direction_y: i32) -> Self {
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
            _ => {
                // Do nothing.
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
        for (y, line) in input_string.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = Vector {
                    x: x as i32,
                    y: y as i32,
                };
                match c {
                    '^' => {
                        carts.push(Cart::new(position, 0, -1));
                    }
                    'v' => {
                        carts.push(Cart::new(position, 0, 1));
                    }
                    '<' => {
                        carts.push(Cart::new(position, -1, 0));
                    }
                    '>' => {
                        carts.push(Cart::new(position, 1, 0));
                    }
                    '+' => {
                        track.insert(position, TrackPiece::Intersection);
                    }
                    '|' => {
                        track.insert(position, TrackPiece::Vertical);
                    }
                    '/' => {
                        if y == 0 {
                            track.insert(position, TrackPiece::TopLeft);
                        } else {
                            track.insert(
                                position,
                                match track.get(&Vector {
                                    x: position.x,
                                    y: position.y - 1,
                                }) {
                                    Some(TrackPiece::Vertical) => TrackPiece::BottomRight,
                                    _ => TrackPiece::TopLeft,
                                },
                            );
                        }
                    }
                    '\\' => {
                        track.insert(position, TrackPiece::Intersection);
                        if y == 0 {
                            track.insert(position, TrackPiece::TopRight);
                        } else {
                            track.insert(
                                position,
                                match track.get(&Vector {
                                    x: position.x,
                                    y: position.y - 1,
                                }) {
                                    Some(TrackPiece::Vertical) => TrackPiece::BottomLeft,
                                    _ => TrackPiece::TopRight,
                                },
                            );
                        }
                    }
                    '-' | ' ' => {
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

                match self.cart_positions.get(&cart.position) {
                    Some(_) => {
                        self.cart_positions.remove(&cart.position);
                        removed_positions.insert(cart.position);
                        continue;
                    }
                    None => {
                        self.cart_positions.insert(cart.position);
                    }
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

pub fn part1(input_string: &str) -> Result<String, String> {
    let mut track = Track::parse(input_string)?;
    let crash_position = track.find_crash();
    Ok(format!("{},{}", crash_position.x, crash_position.y))
}

pub fn part2(input_string: &str) -> Result<String, String> {
    let mut track = Track::parse(input_string)?;
    let remaining_position = track.find_remaining();
    Ok(format!("{},{}", remaining_position.x, remaining_position.y))
}

#[test]
fn tests_part1() {
    assert_eq!(
        Ok("0,3".to_string()),
        part1(
            "|
v
|
|
|
^
|"
        )
    );

    assert_eq!(
        Ok("7,3".to_string()),
        part1(
            r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/"#
        )
    );

    assert_eq!(
        Ok("65,73".to_string()),
        part1(include_str!("day13_input.txt"))
    );
}

#[test]
fn tests_part2() {
    assert_eq!(
        Ok("6,4".to_string()),
        part2(
            r#"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
"#
        )
    );

    assert_eq!(
        Ok("54,66".to_string()),
        part2(include_str!("day13_input.txt"))
    );
}
