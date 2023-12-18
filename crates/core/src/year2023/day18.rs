use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<u64, String> {
    const X_OFFSET: usize = 250;
    const Y_OFFSET: usize = 20;
    const GRID_WIDTH: usize = 520;
    const GRID_HEIGHT: usize = 260;

    let (mut x, mut y) = (X_OFFSET, Y_OFFSET);
    let (mut max_x, mut max_y) = (usize::MIN, usize::MIN);
    let (mut min_x, mut min_y) = (usize::MAX, usize::MAX);
    let mut visited = [false; GRID_WIDTH * GRID_HEIGHT];

    for line in input.text.lines() {
        let mut parts = line.split(' ');
        let direction = parts.next().ok_or_else(on_error)?;
        let amount = parts
            .next()
            .ok_or_else(on_error)?
            .parse::<i32>()
            .map_err(|_| on_error())?;
        let color = parts.next().ok_or_else(on_error)?;

        let (direction, amount) = if input.is_part_one() {
            (direction, amount)
        } else {
            let direction = match color.as_bytes()[7] {
                b'0' => "R",
                b'1' => "D",
                b'2' => "L",
                _ => "U",
            };
            (
                direction,
                i32::from_str_radix(&color[2..7], 16).map_err(|_| on_error())?,
            )
        };

        match direction {
            "U" => {
                for _ in 0..amount {
                    y += 1;
                    visited[(y * GRID_WIDTH) + x] = true;
                }
            }
            "R" => {
                for _ in 0..amount {
                    x += 1;
                    visited[(y * GRID_WIDTH) + x] = true;
                }
            }
            "D" => {
                for _ in 0..amount {
                    y -= 1;
                    visited[(y * GRID_WIDTH) + x] = true;
                }
            }
            _ => {
                for _ in 0..amount {
                    x -= 1;
                    visited[(y * GRID_WIDTH) + x] = true;
                }
            }
        }
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    let mut result = 0;
    for y in (min_y..=max_y).rev() {
        let mut inside_loop = false;
        for x in min_x..=max_x {
            //println!("{},{} is inside? {}", x, y, visited[(y * GRID_WIDTH) + x]);
            //print!("{}", if visited[(y * GRID_WIDTH) + x] { '#' } else { '.' });
            let this_is_edge = visited[(y * GRID_WIDTH) + x];

            if this_is_edge && visited[(y + 1) * GRID_WIDTH + x] {
                inside_loop = !inside_loop;
            }
            result += u64::from(this_is_edge | inside_loop);
        }
    }
    Ok(result)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    test_part_one_no_allocations!(test_input => 62);
    //test_part_two_no_allocations!(test_input => 0);

    let real_input = include_str!("day18_input.txt");
    test_part_one_no_allocations!(real_input => 50_746);
    //test_part_two_no_allocations!(real_input => 0);
}
