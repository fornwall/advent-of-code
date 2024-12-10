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
    } else if grid.width >= 64 {
        return Err("Invalid input - too big rectangle".to_string());
    }

    let mut sum = 0;
    for y in 0..grid.width {
        for x in 0..grid.width {
            if grid.at((x, y)) == b'0' {
                let mut visited = [0_u64; 64];
                sum += search_at(&grid, (x, y), &mut visited, input.is_part_two());
            }
        }
    }
    Ok(sum)
}

fn search_at(grid: &Grid, position: (i32, i32), visited: &mut [u64; 64], part2: bool) -> u32 {
    let height_at_position = grid.at(position);
    [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .map(|(dx, dy)| {
            let neighbour_position = (position.0 + dx, position.1 + dy);
            let height_at_neighbour = grid.at(neighbour_position);
            if height_at_neighbour == height_at_position + 1 {
                if height_at_neighbour == b'9' {
                    if part2
                        || visited[neighbour_position.1 as usize]
                            & (1 << neighbour_position.0 as usize)
                            == 0
                    {
                        visited[neighbour_position.1 as usize] |=
                            1 << neighbour_position.0 as usize;
                        1
                    } else {
                        0
                    }
                } else {
                    search_at(grid, neighbour_position, visited, part2)
                }
            } else {
                0
            }
        })
        .sum()
}

struct Grid<'a> {
    s: &'a [u8],
    width: i32,
}

impl Grid<'_> {
    const fn at(&self, position: (i32, i32)) -> u8 {
        if position.0 < 0 || position.0 >= self.width || position.1 < 0 || position.1 >= self.width
        {
            return b'Q';
        }
        self.s[(position.0 + (self.width + 1) * position.1) as usize]
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
    test_part_one_no_allocations!(test_input => 2);
    let test_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    test_part_one_no_allocations!(test_input => 36);
    let test_input = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";
    test_part_two_no_allocations!(test_input => 3);

    let real_input = include_str!("day10_input.txt");
    test_part_one_no_allocations!(real_input => 737);
    test_part_two_no_allocations!(real_input => 1619);
}
