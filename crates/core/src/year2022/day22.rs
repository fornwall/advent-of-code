use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    let (direction_str, mut cube) =
        Cube::parse(input.text, input.is_part_two()).ok_or("Invalid input")?;

    let mut direction = Direction::Right;
    let mut steps_forward = 0;
    for c in direction_str.bytes() {
        if c.is_ascii_digit() {
            if steps_forward > 10 {
                return Err("Too many steps in a direction".to_string());
            }
            steps_forward = steps_forward * 10 + (c - b'0');
        } else {
            direction = cube.step_forward(steps_forward, direction);
            match c {
                b'R' => {
                    direction = direction.turn_right();
                }
                b'L' => {
                    direction = direction.turn_left();
                }
                _ => {
                    return Err("Strange direction".to_string());
                }
            }
            steps_forward = 0;
        }
    }
    if steps_forward != 0 {
        direction = cube.step_forward(steps_forward, direction);
    }

    Ok(cube.password(direction))
}

#[derive(Copy, Clone)]
#[allow(clippy::type_complexity)]
struct CubeSide {
    grid: [bool; Self::SIZE * Self::SIZE],
    // (top, right, left, bottom)
    // The direction is the direction one facing the side on, in map view.
    adjacent_sides: (
        (usize, Direction),
        (usize, Direction),
        (usize, Direction),
        (usize, Direction),
    ),
    grid_position: (usize, usize),
}

impl CubeSide {
    const SIZE: usize = 50;

    const fn is_free(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= (Self::SIZE as i32) || y < 0 || y >= (Self::SIZE as i32) {
            return false;
        }
        !self.grid[y as usize * Self::SIZE + x as usize]
    }

    const fn set_wall(&mut self, x: usize, y: usize) {
        self.grid[y * Self::SIZE + x] = true;
    }
}

struct Cube {
    sides: [CubeSide; 6],
    current_cube_idx: usize,
    current_position: (i32, i32),
}

impl Cube {
    fn parse(input: &str, fold_cube: bool) -> Option<(&str, Self)> {
        let mut sides = [CubeSide {
            grid: [false; CubeSide::SIZE * CubeSide::SIZE],
            grid_position: (usize::MAX, usize::MAX),
            adjacent_sides: (
                (usize::MAX, Direction::Right),
                (usize::MAX, Direction::Right),
                (usize::MAX, Direction::Right),
                (usize::MAX, Direction::Right),
            ),
        }; 6];

        let mut parts = input.split("\n\n");
        let cubes_part = parts.next()?;
        let directions_part = parts.next()?;

        let mut seen_sides = 0;
        for (row_idx, line) in cubes_part.lines().enumerate() {
            let line_start = line.find(['.', '#'])?;
            let line_end = line.rfind(['.', '#'])?;
            let line_end_inclusive = line_end + 1;
            if line_start % CubeSide::SIZE != 0 || line_end_inclusive % CubeSide::SIZE != 0 {
                return None;
            }

            let sides_on_line = (line_end_inclusive - line_start) / CubeSide::SIZE;

            for (col_idx, &c) in line.as_bytes()[line_start..line_end_inclusive]
                .iter()
                .enumerate()
            {
                if c == b'#' {
                    let cube_offset = col_idx / CubeSide::SIZE;
                    sides[seen_sides + cube_offset].set_wall(col_idx % 50, row_idx % 50);
                }
            }

            if row_idx % CubeSide::SIZE == 49 {
                for i in 0..sides_on_line {
                    sides[seen_sides + i].grid_position =
                        (line_start / CubeSide::SIZE + i, row_idx / CubeSide::SIZE);
                }
                seen_sides += sides_on_line;
            }
        }

        if sides.iter().any(|side| side.grid_position.0 == usize::MAX) {
            return None;
        }

        if fold_cube {
            // See https://www.reddit.com/r/adventofcode/comments/zsct8w/comment/j17s6l5/
            // for description of programmatic approach.
            //
            //       ┌─────┬─────┐
            //       │  A  │  E  │
            //       │D 0 B│B 1 F│
            //       │  C  │  G  │
            //       ├─────┼─────┘
            //       │  C  │
            //       │I 2 G│
            //       │  H  │
            // ┌─────┼─────┤
            // │  I  │  H  │
            // │D 3 J│J 4 F│
            // │  K  │  L  │
            // ├─────┼─────┘
            // │  K  │
            // │A 5 L│
            // │  E  │
            // └─────┘
            sides[0].adjacent_sides = (
                (5, Direction::Right),
                (1, Direction::Right),
                (2, Direction::Down),
                (3, Direction::Right),
            );
            sides[1].adjacent_sides = (
                (5, Direction::Up),
                (4, Direction::Left),
                (2, Direction::Left),
                (0, Direction::Left),
            );
            sides[2].adjacent_sides = (
                (0, Direction::Up),
                (1, Direction::Up),
                (4, Direction::Down),
                (3, Direction::Down),
            );
            sides[3].adjacent_sides = (
                (2, Direction::Right),
                (4, Direction::Right),
                (5, Direction::Down),
                (0, Direction::Right),
            );
            sides[4].adjacent_sides = (
                (2, Direction::Up),
                (1, Direction::Left),
                (5, Direction::Left),
                (3, Direction::Left),
            );
            sides[5].adjacent_sides = (
                (3, Direction::Up),
                (4, Direction::Up),
                (1, Direction::Down),
                (0, Direction::Down),
            );
        } else {
            sides[0].adjacent_sides = (
                (4, Direction::Up),
                (1, Direction::Right),
                (2, Direction::Down),
                (1, Direction::Left),
            );
            sides[1].adjacent_sides = (
                (1, Direction::Up),
                (0, Direction::Right),
                (1, Direction::Down),
                (0, Direction::Left),
            );
            sides[2].adjacent_sides = (
                (0, Direction::Up),
                (2, Direction::Right),
                (4, Direction::Down),
                (2, Direction::Left),
            );
            sides[3].adjacent_sides = (
                (5, Direction::Up),
                (4, Direction::Right),
                (5, Direction::Down),
                (4, Direction::Left),
            );
            sides[4].adjacent_sides = (
                (2, Direction::Up),
                (3, Direction::Right),
                (0, Direction::Down),
                (3, Direction::Left),
            );
            sides[5].adjacent_sides = (
                (3, Direction::Up),
                (5, Direction::Right),
                (3, Direction::Down),
                (5, Direction::Left),
            );
        }
        Some((
            directions_part,
            Self {
                sides,
                current_cube_idx: 0,
                current_position: (0, 0),
            },
        ))
    }

    fn step_forward(&mut self, steps: u8, mut direction: Direction) -> Direction {
        #![allow(clippy::panic)]
        let mut delta = direction.delta();
        for _step in 0..steps {
            let mut new_position = (
                self.current_position.0 + delta.0,
                self.current_position.1 + delta.1,
            );
            let mut new_cube_idx = self.current_cube_idx;
            let mut new_direction = direction;

            if let Some((c, facing_direction)) = if new_position.0 < 0 {
                Some(self.sides[self.current_cube_idx].adjacent_sides.3)
            } else if new_position.0 == CubeSide::SIZE as i32 {
                Some(self.sides[self.current_cube_idx].adjacent_sides.1)
            } else if new_position.1 < 0 {
                Some(self.sides[self.current_cube_idx].adjacent_sides.0)
            } else if new_position.1 == CubeSide::SIZE as i32 {
                Some(self.sides[self.current_cube_idx].adjacent_sides.2)
            } else {
                None
            } {
                new_cube_idx = c;
                new_position = match (direction, facing_direction) {
                    (Direction::Up, Direction::Up) => (new_position.0, (CubeSide::SIZE - 1) as i32),
                    (Direction::Up, Direction::Right) => (0, new_position.0),
                    (Direction::Right, Direction::Up) => {
                        (new_position.1, (CubeSide::SIZE - 1) as i32)
                    }
                    (Direction::Right, Direction::Right) => (0, new_position.1),
                    (Direction::Right, Direction::Left) => (
                        (CubeSide::SIZE - 1) as i32,
                        (CubeSide::SIZE - 1) as i32 - new_position.1,
                    ),
                    (Direction::Left, Direction::Right) => {
                        (0, (CubeSide::SIZE - 1) as i32 - new_position.1)
                    }
                    (Direction::Left, Direction::Down) => (new_position.1, 0),
                    (Direction::Left, Direction::Left) => {
                        ((CubeSide::SIZE - 1) as i32, new_position.1)
                    }
                    (Direction::Down, Direction::Down) => (new_position.0, 0),
                    (Direction::Down, Direction::Left) => {
                        ((CubeSide::SIZE - 1) as i32, new_position.0)
                    }
                    _ => {
                        panic!("Unhandled combo");
                    }
                };
                new_direction = facing_direction;
            }

            if self.sides[new_cube_idx].is_free(new_position.0, new_position.1) {
                self.current_position = new_position;
                self.current_cube_idx = new_cube_idx;
                direction = new_direction;
                delta = direction.delta();
            } else {
                break;
            }
        }
        direction
    }

    const fn password(&self, direction: Direction) -> u64 {
        let (side_x, side_y) = self.sides[self.current_cube_idx].grid_position;
        let (offset_x, offset_y) = (side_x * CubeSide::SIZE, side_y * CubeSide::SIZE);
        1000 * (self.current_position.1 as u64 + 1 + offset_y as u64)
            + 4 * (self.current_position.0 as u64 + 1 + offset_x as u64)
            + (direction as u64)
    }
}

/// "Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^)."
#[derive(Copy, Clone)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    const fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
    const fn delta(self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};

    let real_input = include_str!("day22_input.txt");
    test_part_one!(real_input => 89_224);
    test_part_two!(real_input => 136_182);

    let real_input = include_str!("day22_input_other.txt");
    test_part_one!(real_input => 76_332);
    test_part_two!(real_input => 144_012);

    test_part_one_error!("\n\n" => "Invalid input");
}
