use crate::{
    common::array_stack::ArrayStack,
    input::{on_error, Input},
};

pub fn solve(input: &Input) -> Result<u32, String> {
    const SPACE_WIDTH: i32 = 101;
    const SPACE_HEIGHT: i32 = 103;
    const MIDDLE_X: i32 = SPACE_WIDTH / 2;
    const MIDDLE_Y: i32 = SPACE_HEIGHT / 2;
    const MAX_TIME_PART2: usize = 10_000;

    let mut robots = ArrayStack::<512, (i8, i8, i8, i8)>::new();

    for line in input.text.lines() {
        let mut parts = line.split(&['=', 'v', ' ', ',']).skip(1);
        let p_x = parse_it(&mut parts)?;
        let p_y = parse_it(&mut parts)?;
        let mut parts = parts.skip(2);
        let v_x = parse_it(&mut parts)?;
        let v_y = parse_it(&mut parts)?;
        robots.push((p_x, p_y, v_x, v_y))?;
    }

    if input.is_part_one() {
        let mut quadrants = [0; 4];
        for &(p_x, p_y, v_x, v_y) in robots.slice() {
            let (p_x, p_y, v_x, v_y) = (p_x as i32, p_y as i32, v_x as i32, v_y as i32);
            let f_x = (p_x + v_x * 100).rem_euclid(SPACE_WIDTH);
            let f_y = (p_y + v_y * 100).rem_euclid(SPACE_HEIGHT);
            if f_x != MIDDLE_X && f_y != MIDDLE_Y {
                let part_x = f_x / ((SPACE_WIDTH + 1) / 2);
                let part_y = f_y / ((SPACE_HEIGHT + 1) / 2);
                quadrants[part_y as usize * 2 + part_x as usize] += 1;
            }
        }
        Ok(quadrants.iter().product())
    } else {
        // Thanks https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2024/day14.rs
        let mut x_over_time = [[0_i8; 500]; SPACE_WIDTH as usize];
        let mut y_over_time = [[0_i8; 500]; SPACE_HEIGHT as usize];
        for (time, row) in x_over_time.iter_mut().enumerate() {
            for (i, (x, _, dx, _)) in robots.slice().iter().enumerate() {
                row[i] = (*x as i32 + *dx as i32 * time as i32).rem_euclid(SPACE_WIDTH) as i8;
            }
        }
        for (time, row) in y_over_time.iter_mut().enumerate() {
            for (i, (_, y, _, dy)) in robots.slice().iter().enumerate() {
                row[i] = (*y as i32 + *dy as i32 * time as i32).rem_euclid(SPACE_HEIGHT) as i8;
            }
        }

        let mut visited = [0_usize; (SPACE_WIDTH * SPACE_HEIGHT) as usize];
        'time: for time in 1..MAX_TIME_PART2 {
            for (&x, &y) in x_over_time[time % (SPACE_WIDTH as usize)]
                .iter()
                .zip(y_over_time[time % (SPACE_HEIGHT as usize)].iter())
            {
                let idx = y as usize + (x as usize) * (SPACE_WIDTH as usize);
                if visited[idx] == time {
                    continue 'time;
                }
                visited[idx] = time;
            }
            return Ok(time as u32);
        }

        Err(format!(
            "No christmas tree found in {MAX_TIME_PART2} seconds"
        ))
    }
}

fn parse_it<'a, I: Iterator<Item = &'a str>>(it: &mut I) -> Result<i8, String> {
    it.next()
        .ok_or_else(on_error)?
        .parse::<i8>()
        .map_err(|_| on_error())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let real_input = include_str!("day14_input.txt");
    test_part_one_no_allocations!(real_input => 226_236_192);
    test_part_two_no_allocations!(real_input => 8_168);
}
