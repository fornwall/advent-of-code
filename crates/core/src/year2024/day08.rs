use crate::common::array_stack::ArrayStack;
use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<u32, String> {
    let width = input
        .text
        .lines()
        .next()
        .map(|line| line.len())
        .ok_or_else(on_error)? as i32;
    let grid = Grid {
        s: input.text.as_bytes(),
        width,
    };
    if grid.s.len() != ((grid.width + 1) * grid.width - 1) as usize {
        return Err("Invalid input - not a rectangle".to_string());
    }

    let mut visited = [0_u64; 64];
    let mut antennas: [ArrayStack<10, (u8, u8)>; u8::MAX as usize] =
        std::array::from_fn(|_idx| ArrayStack::<10, (u8, u8)>::new());

    for y in 0..grid.width {
        for x in 0..grid.width {
            let cell = grid.at((x, y));
            if cell != b'.' {
                if input.is_part_two() {
                    visited[y as usize] |= 1 << x;
                }
                let same_antennas = &mut antennas[cell as usize];
                for &same_freq in same_antennas.slice().iter() {
                    let mut multiplier = 0;
                    loop {
                        let dist = (same_freq.0 as i32 - x, same_freq.1 as i32 - y);
                        let mut within_bounds = false;

                        for m in [2 + multiplier, -(1 + multiplier)] {
                            let antinode = (x + dist.0 * m, y + dist.1 * m);
                            if antinode.0 >= 0
                                && antinode.0 < grid.width
                                && antinode.1 >= 0
                                && antinode.1 < grid.width
                            {
                                visited[antinode.1 as usize] |= 1 << antinode.0;
                                within_bounds = true;
                            }
                        }

                        if input.is_part_one() || !within_bounds {
                            break;
                        } else {
                            multiplier += 1;
                        }
                    }
                }
                same_antennas.push((x as u8, y as u8)).unwrap();
            }
        }
    }

    Ok(visited.iter().map(|i| i.count_ones()).sum())
}

struct Grid<'a> {
    s: &'a [u8],
    width: i32,
}

impl Grid<'_> {
    const fn at(&self, position: (i32, i32)) -> u8 {
        if position.0 < 0 || position.0 >= self.width || position.1 < 0 || position.1 >= self.width
        {
            return b'L';
        }
        self.s[(position.0 + (self.width + 1) * position.1) as usize]
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    test_part_one_no_allocations!(test_input => 14);
    test_part_two_no_allocations!(test_input => 34);

    let real_input = include_str!("day08_input.txt");
    test_part_one_no_allocations!(real_input => 256);
    test_part_two_no_allocations!(real_input => 1005);
}
