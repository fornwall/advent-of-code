use crate::common::u256::U256;
use crate::input::{Input, on_error};

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
    } else if grid.width >= 150 {
        return Err("Invalid input - too big rectangle".to_string());
    }

    let mut visited = [U256::default(); 192];

    let mut sum = 0;
    for y in 0..grid.width {
        for x in 0..grid.width {
            if !visited[y as usize].is_bit_set(x as usize) {
                let position = (x, y);
                let current_region_type = grid.at(position);
                let result = visit_region(
                    current_region_type,
                    position,
                    &grid,
                    &mut visited,
                    input.is_part_one(),
                );
                sum += result.0 * result.1;
            }
        }
    }
    Ok(sum as u32)
}

struct Grid<'a> {
    s: &'a [u8],
    width: i32,
}

impl Grid<'_> {
    const fn at(&self, position: (i32, i32)) -> u8 {
        if position.0 < 0 || position.0 >= self.width || position.1 < 0 || position.1 >= self.width
        {
            return b'@';
        }
        self.s[(position.0 + (self.width + 1) * position.1) as usize]
    }
}

fn visit_region(
    current_region_type: u8,
    position: (i32, i32),
    grid: &Grid,
    visited: &mut [U256; 192],
    part1: bool,
) -> (/*area*/ i32, /*perimeter*/ i32) {
    #![allow(clippy::unusual_byte_groupings)]
    visited[position.1 as usize].set_bit(position.0 as usize);

    let mut result = (1, 0);

    let corners = [
        // Top row
        (-1, -1),
        (0, -1),
        (1, -1),
        // left/right
        (-1, 0),
        (1, 0),
        // Bottom
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let bits = corners
        .iter()
        .enumerate()
        .fold(0, |acc, (offset, (dx, dy))| {
            let n = (position.0 + dx, position.1 + dy);
            acc | ((1 << offset) * u8::from(grid.at(n) == current_region_type))
        });

    for offset in [1, 4, 3, 6] {
        let d = corners[offset];
        if (1 << offset) & bits != 0 {
            let n = (position.0 + d.0, position.1 + d.1);
            if !visited[n.1 as usize].is_bit_set(n.0 as usize) {
                let v = visit_region(current_region_type, n, grid, visited, part1);
                result.0 += v.0;
                result.1 += v.1;
            }
        }
    }

    result.1 += if part1 {
        let left_is_perimeter = bits & 0b000_01_000 == 0;
        let right_is_perimeter = bits & 0b000_10_000 == 0;
        let top_is_perimeter = bits & 0b010_00_000 == 0;
        let bottom_is_perimeter = bits & 0b000_00_010 == 0;
        i32::from(left_is_perimeter)
            + i32::from(right_is_perimeter)
            + i32::from(top_is_perimeter)
            + i32::from(bottom_is_perimeter)
    } else {
        // Number of sides == number of corners.
        // Corners have two variants:
        //   .X
        //   XX
        // OR
        //   ?.
        //   .X
        let top_left_is_corner =
            (bits & 0b000_01_011) == 0b000_01_010 || (bits & 0b000_01_010) == 0;
        let top_right_is_corner =
            (bits & 0b000_10_110) == 0b000_10_010 || (bits & 0b000_10_010) == 0;
        let bottom_left_is_corner =
            (bits & 0b011_01_000) == 0b010_01_000 || (bits & 0b010_01_000) == 0;
        let bottom_right_is_corner =
            (bits & 0b110_10_000) == 0b010_10_000 || (bits & 0b010_10_000) == 0;
        i32::from(top_left_is_corner)
            + i32::from(top_right_is_corner)
            + i32::from(bottom_left_is_corner)
            + i32::from(bottom_right_is_corner)
    };
    result
}

#[test]
pub fn tests() {
    let test_input = "AAAA
BBCD
BBCC
EEEC";
    test_part_two_no_allocations!(test_input => 80);

    let test_input = "AAAA
BBCD
BBCC
EEEC";
    test_part_one_no_allocations!(test_input => 140);
    let test_input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    test_part_one_no_allocations!(test_input => 1930);
    let test_input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    test_part_two_no_allocations!(test_input => 236);
    let test_input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    test_part_two_no_allocations!(test_input => 368);

    let real_input = include_str!("day12_input.txt");
    test_part_one_no_allocations!(real_input => 1_477_924);
    test_part_two_no_allocations!(real_input => 841_934);
}
