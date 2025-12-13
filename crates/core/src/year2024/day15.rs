use crate::common::array_stack::ArrayStack;
use crate::input::{Input, on_error};
use std::mem::swap;

pub fn solve(input: &Input) -> Result<u32, String> {
    let (grid_str, moves) = input.text.split_once("\n\n").ok_or_else(on_error)?;
    let mut grid = Grid::parse(grid_str, input.is_part_one())?;

    for m in moves.bytes() {
        grid.move_robot(
            match m {
                b'<' => (-1, 0),
                b'>' => (1, 0),
                b'^' => (0, -1),
                b'v' => (0, 1),
                _ => continue,
            },
            input.is_part_two(),
        );
    }

    Ok(grid.boxes_gps_sum(input.part_values(b'O', b'[')))
}

struct Grid {
    width: usize,
    cells: [u8; 75 * 75],
    robot_position: (i32, i32),
}

impl Grid {
    fn parse(input_string: &str, part1: bool) -> Result<Self, String> {
        let mut height = 0;
        let mut width = 0;
        let mut robot_position = (0, 0);
        let mut cells = [0_u8; 75 * 75];
        let mut cells_size = 0;
        let width_multiplier = if part1 { 1 } else { 2 };

        for (line_idx, line) in input_string.lines().enumerate() {
            let new_width = line.len() * width_multiplier;
            if line_idx == 0 {
                width = new_width;
            } else if new_width != width {
                return Err("Not all lines have equal length".into());
            }
            line.bytes().enumerate().for_each(|(x, b)| {
                if b == b'@' {
                    robot_position = (x as i32 * width_multiplier as i32, height as i32);
                }
                if part1 {
                    cells[cells_size] = b;
                } else {
                    let (b1, b2) = match b {
                        b'@' => (b'@', b'.'),
                        b'O' => (b'[', b']'),
                        _ => (b, b),
                    };
                    cells[cells_size] = b1;
                    cells_size += 1;
                    cells[cells_size] = b2;
                }
                cells_size += 1;
            });
            height += 1;
        }

        if width == 0 {
            return Err("Empty input".into());
        } else if width / width_multiplier != height {
            return Err("Non-square input".into());
        } else if height >= 75 {
            return Err("Too big input".into());
        }

        Ok(Self {
            width,
            cells,
            robot_position,
        })
    }

    const fn at(&self, position: (i32, i32)) -> u8 {
        self.cells[position.1 as usize * self.width + position.0 as usize]
    }

    fn move_robot(&mut self, direction: (i32, i32), part2: bool) {
        #![allow(clippy::unwrap_used)]
        if part2 && direction.1 != 0 {
            let mut will_push_up = ArrayStack::<150, (i32, i32)>::new();
            will_push_up.push(self.robot_position).unwrap();
            let mut already_pushed = [false; 75 * 75];

            let mut i = 0;
            while i < will_push_up.len() {
                let push_up_at = add(will_push_up.elements[i], direction);
                i += 1;
                let diff_x = match self.at(push_up_at) {
                    b'#' => return,
                    b'[' => 1,
                    b']' => -1,
                    _ => continue,
                };
                for to_push in [push_up_at, (push_up_at.0 + diff_x, push_up_at.1)] {
                    let to_push_idx = to_push.1 as usize * self.width + to_push.0 as usize;
                    if !already_pushed[to_push_idx] {
                        already_pushed[to_push_idx] = true;
                        will_push_up.push(to_push).unwrap();
                    }
                }
            }

            for &previous in will_push_up.slice().iter().rev() {
                let next = add(previous, direction);
                let previous_idx = previous.1 as usize * self.width + previous.0 as usize;
                self.cells[next.1 as usize * self.width + next.0 as usize] =
                    self.cells[previous_idx];
                self.cells[previous_idx] = b'.';
            }
        } else {
            let mut position = self.robot_position;
            let mut num_steps = 1;
            loop {
                match self.at(position) {
                    b'#' => return,
                    b'.' => break,
                    _ => {
                        position = add(position, direction);
                        num_steps += 1;
                    }
                }
            }

            let mut replace_with = b'.';
            position = self.robot_position;
            for _ in 0..num_steps {
                swap(
                    &mut self.cells[position.1 as usize * self.width + position.0 as usize],
                    &mut replace_with,
                );
                position = add(position, direction);
            }
        }

        self.robot_position = add(self.robot_position, direction);
    }

    fn boxes_gps_sum(&self, box_type: u8) -> u32 {
        self.cells
            .iter()
            .enumerate()
            .map(|(idx, &c)| {
                usize::from(c == box_type) * ((idx / self.width) * 100 + idx % self.width)
            })
            .sum::<usize>() as u32
    }
}

const fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

#[test]
pub fn tests() {
    let test_input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    test_part_one_no_allocations!(test_input => 2028);
    let test_input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    test_part_two_no_allocations!(test_input => 9021);

    let real_input = include_str!("day15_input.txt");
    test_part_one_no_allocations!(real_input => 1_412_971);
    test_part_two_no_allocations!(real_input => 1_429_299);
}
