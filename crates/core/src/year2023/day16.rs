use crate::common::array_stack::ArrayStack;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    let mut up = ArrayStack::<128, u128>::new();
    let mut right = ArrayStack::<128, u128>::new();
    let mut down = ArrayStack::<128, u128>::new();
    let mut left = ArrayStack::<128, u128>::new();

    let mut mirror_down = ArrayStack::<128, u128>::new();
    let mut mirror_up = ArrayStack::<128, u128>::new();
    let mut splitter_horizontal = ArrayStack::<128, u128>::new();
    let mut splitter_vertical = ArrayStack::<128, u128>::new();

    let mut num_cols = 0;

    for row in input.text.lines() {
        let (mut md, mut mu, mut sh, mut sv) = (0, 0, 0, 0);
        for (col_idx, b) in row.bytes().enumerate() {
            num_cols = col_idx as i8 + 1;
            let bit = 1 << col_idx;
            match b {
                b'|' => sv |= bit,
                b'-' => sh |= bit,
                b'\\' => md |= bit,
                b'/' => mu |= bit,
                _ => {}
            }
        }
        up.push(0)?;
        right.push(0)?;
        down.push(0)?;
        left.push(0)?;
        mirror_up.push(mu)?;
        mirror_down.push(md)?;
        splitter_horizontal.push(sh)?;
        splitter_vertical.push(sv)?;
    }

    if input.is_part_one() {
        do_solve(
            0,
            0,
            Direction::Right,
            num_cols,
            up.clone(),
            right.clone(),
            down.clone(),
            left.clone(),
            &mirror_up,
            &mirror_down,
            &splitter_horizontal,
            &splitter_vertical,
        )
    } else {
        let max_x = num_cols - 1;
        let max_y = up.len() as i8 - 1;
        let mut max = 0;
        for entry_dir in [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ] {
            let (mut entry_x, mut entry_y, x_dir, y_dir) = match entry_dir {
                Direction::Up => (0, max_y, 1, 0),
                Direction::Right => (0, 0, 0, 1),
                Direction::Left => (max_x, 0, 0, 1),
                Direction::Down => (0, 0, 1, 0),
            };
            while entry_x >= 0 && entry_x < max_x && entry_y >= 0 && entry_y < max_y {
                max = max.max(do_solve(
                    entry_x,
                    entry_y,
                    entry_dir,
                    num_cols,
                    up.clone(),
                    right.clone(),
                    down.clone(),
                    left.clone(),
                    &mirror_up,
                    &mirror_down,
                    &splitter_horizontal,
                    &splitter_vertical,
                )?);
                entry_x += x_dir;
                entry_y += y_dir;
            }
        }
        Ok(max)
    }
}

type M = ArrayStack<128, u128>;

#[allow(clippy::too_many_arguments)]
fn do_solve(
    entry_x: i8,
    entry_y: i8,
    entry_direction: Direction,
    num_cols: i8,
    mut up: M,
    mut right: M,
    mut down: M,
    mut left: M,
    mirror_up: &M,
    mirror_down: &M,
    splitter_horizontal: &M,
    splitter_vertical: &M,
) -> Result<u32, String> {
    let mut energized = ArrayStack::<128, u128>::with_len(up.len());

    let mut current_ray = Ray {
        x: entry_x,
        y: entry_y,
        direction: entry_direction,
    };
    let mut rays = ArrayStack::<128, Ray>::new();

    loop {
        let y = current_ray.y as usize;
        let x_bit = 1 << current_ray.x.max(0) as usize;

        let arr = match current_ray.direction {
            Direction::Up => &mut up,
            Direction::Right => &mut right,
            Direction::Down => &mut down,
            Direction::Left => &mut left,
        };

        if current_ray.x < 0
            || current_ray.x >= num_cols
            || current_ray.y < 0
            || current_ray.y >= (energized.len() as i8)
            || ((arr.elements[y] & x_bit) != 0)
        {
            if let Some(ray) = rays.pop() {
                current_ray = ray;
                continue;
            } else {
                return Ok(energized.slice().iter().map(|bits| bits.count_ones()).sum());
            }
        }

        arr.elements[y] |= x_bit;
        energized.elements[y] |= x_bit;

        if mirror_up.elements[y] & x_bit != 0 {
            // "/"
            current_ray.direction = match current_ray.direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
            };
        } else if mirror_down.elements[y] & x_bit != 0 {
            current_ray.direction = match current_ray.direction {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
            };
        } else if splitter_horizontal.elements[y] & x_bit != 0
            && current_ray.direction.is_vertical()
        {
            current_ray.direction = Direction::Right;
            rays.push(Ray {
                x: current_ray.x,
                y: current_ray.y,
                direction: Direction::Left,
            })?;
        } else if splitter_vertical.elements[y] & x_bit != 0
            && current_ray.direction.is_horizontal()
        {
            current_ray.direction = Direction::Up;
            rays.push(Ray {
                x: current_ray.x,
                y: current_ray.y,
                direction: Direction::Down,
            })?;
        }

        match current_ray.direction {
            Direction::Up => current_ray.y -= 1,
            Direction::Right => current_ray.x += 1,
            Direction::Down => current_ray.y += 1,
            Direction::Left => current_ray.x -= 1,
        };
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}

impl Direction {
    const fn is_vertical(self) -> bool {
        matches!(self, Self::Up | Self::Down)
    }
    const fn is_horizontal(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
}

#[derive(Copy, Clone, Default)]
struct Ray {
    x: i8,
    y: i8,
    direction: Direction,
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    test_part_one_no_allocations!(test_input => 46);
    test_part_two_no_allocations!(test_input => 51);

    let real_input = include_str!("day16_input.txt");
    test_part_one_no_allocations!(real_input => 6361);
    test_part_two_no_allocations!(real_input => 6701);
}
