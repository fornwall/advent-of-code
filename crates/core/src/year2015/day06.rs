use crate::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    fn parse_tuple(tuple: &str) -> Option<(u16, u16)> {
        let mut parts = tuple.split(',');
        let first = parts.next()?.parse::<u16>().ok()?;
        let second = parts.next()?.parse::<u16>().ok()?;
        Some((first, second))
    }

    let mut grid = [0_u8; 1_000_000];
    for line in input.text.lines() {
        let words = line.split(' ').collect::<Vec<&str>>();
        let (from, to) = if words[0] == "toggle" {
            (words[1], words[3])
        } else {
            (words[2], words[4])
        };

        let (from_x, from_y) = parse_tuple(from).ok_or("Invalid input")?;
        let (to_x, to_y) = parse_tuple(to).ok_or("Invalid input")?;

        for x in from_x..=to_x {
            for y in from_y..=to_y {
                let index = x as usize + y as usize * 1000;
                grid[index] = match words[1] {
                    "on" => {
                        if input.is_part_one() {
                            1
                        } else {
                            grid[index] + 1
                        }
                    }
                    "off" => {
                        if input.is_part_one() {
                            0
                        } else {
                            grid[index] - if grid[index] == 0 { 0 } else { 1 }
                        }
                    }
                    _ => {
                        if input.is_part_one() {
                            if grid[index] == 0 {
                                1
                            } else {
                                0
                            }
                        } else {
                            grid[index] + 2
                        }
                    }
                };
            }
        }
    }

    Ok(grid.iter().map(|&i| i as usize).sum())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day06_input.txt");
    test_part_one!(real_input => 569_999);
    test_part_two!(real_input => 17_836_115);
}
