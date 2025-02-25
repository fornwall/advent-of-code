use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<i64, String> {
    let (mut x, mut y) = (0, 0);
    let (mut area_sum, mut trench_len) = (0_i64, 0_i64);

    for line in input.text.lines() {
        let mut parts = line.split(' ');
        let (direction, amount) = if input.is_part_one() {
            let first = parts.next().ok_or_else(on_error)?;
            let second = parts.next().ok_or_else(on_error)?;
            (
                first.as_bytes()[0],
                second.parse::<i64>().map_err(|_| on_error())?,
            )
        } else {
            let color = parts.nth(2).ok_or_else(on_error)?;
            (
                color.as_bytes()[7],
                i64::from_str_radix(&color[2..7], 16).map_err(|_| on_error())?,
            )
        };

        let (new_x, new_y) = match direction {
            b'U' | b'0' => (x, y + amount),
            b'R' | b'1' => (x + amount, y),
            b'D' | b'2' => (x, y - amount),
            _ => (x - amount, y),
        };

        area_sum += x * new_y - new_x * y;
        trench_len += (new_x - x).abs() + (new_y - y).abs();
        (x, y) = (new_x, new_y);
    }

    // https://en.wikipedia.org/wiki/Shoelace_formula#Triangle_formula:
    let area_inside = area_sum.abs() / 2;
    // In addition to the area inside of the trench path, we need to count
    // the 1/2 wide areas outside of the border:
    //    12
    // 8 ---
    // 7 | | 3
    //   --- 4
    //   65
    // With the above sketch starting from the top right, there are 1/4
    // square missed for each clockwise turn, and a 1/4 of a square double
    // counted for each counterclockwise turn.
    // Since this is a loop there are 4 more clockwise turns -> add 4*1/4=1.
    let area_from_border = trench_len / 2 + 1;
    Ok(area_inside + area_from_border)
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
    test_part_two_no_allocations!(test_input => 952_408_144_115);

    let real_input = include_str!("day18_input.txt");
    test_part_one_no_allocations!(real_input => 50_746);
    test_part_two_no_allocations!(real_input => 70_086_216_556_038);
}
