use crate::input::Input;

#[cfg(feature = "visualization")]
use super::day24_renderer::Renderer;

pub fn solve(input: &Input) -> Result<i32, String> {
    const MAX_STEPS: usize = 10_000;
    let mut remaining_trips = input.part_values(1, 3);

    let mut valley = parse(input.text)?;
    let mut reachable = vec![0; valley.width];
    #[cfg(feature = "visualization")]
    let mut renderer = Renderer::new(&reachable, &valley);

    let top_row_bitmask = 1;
    let bottom_row_bitmask = 1 << (valley.height - 1);

    for minute in 0..MAX_STEPS {
        valley
            .blizzards_up
            .iter_mut()
            .for_each(|m| *m = (*m >> 1) | ((*m & top_row_bitmask) << (valley.height - 1)));
        valley
            .blizzards_down
            .iter_mut()
            .for_each(|m| *m = (*m << 1) | ((*m & bottom_row_bitmask) >> (valley.height - 1)));
        valley.blizzards_right.rotate_right(1);
        valley.blizzards_left.rotate_left(1);

        let heading_down = remaining_trips % 2 == 1;

        let one_trip_completed = if heading_down {
            reachable[valley.width - 1] & bottom_row_bitmask != 0
        } else {
            reachable[0] & 1 != 0
        };

        if one_trip_completed {
            reachable.fill(0);
            remaining_trips -= 1;

            #[cfg(feature = "visualization")]
            renderer
                .reachable_per_step
                .push((reachable.clone(), !heading_down));

            if remaining_trips == 0 {
                #[cfg(feature = "visualization")]
                input
                    .rendered_svg
                    .replace(renderer.final_svg(&valley, minute));
                return Ok(minute as i32 + 1);
            }
            continue;
        }

        let mut prev = if heading_down { top_row_bitmask } else { 0 };
        let last = if heading_down { 0 } else { bottom_row_bitmask };
        for x in 0..valley.width {
            let prev = std::mem::replace(&mut prev, reachable[x]);
            let next = reachable.get(x + 1).copied().unwrap_or(last);

            // Expand reachable up, down, left and right:
            reachable[x] |= (reachable[x] >> 1) | (reachable[x] << 1) | prev | next;
            // Positions where there are blizzards are not reachable:
            reachable[x] &= valley.blizzards_up[x]
                & valley.blizzards_down[x]
                & valley.blizzards_right[x]
                & valley.blizzards_left[x];
        }

        #[cfg(feature = "visualization")]
        renderer
            .reachable_per_step
            .push((reachable.clone(), heading_down));
    }

    Err(format!("No solution found in {MAX_STEPS} minutes"))
}

pub struct Valley {
    pub width: usize,
    pub height: usize,
    pub blizzards_up: Vec<u64>,
    pub blizzards_down: Vec<u64>,
    pub blizzards_right: Vec<u64>,
    pub blizzards_left: Vec<u64>,
}

fn parse(input: &str) -> Result<Valley, String> {
    let width = input
        .find('\n')
        .ok_or("Invalid input - not multiple lines")?
        - 2;
    let height = input.lines().count() - 2;
    if width < 3 || height < 3 {
        return Err("Too small valley".to_string());
    } else if height > 64 || width > 200 {
        return Err("Too big valley".to_string());
    }

    let mut blizzards_up = vec![(1 << height) - 1; width];
    let mut blizzards_down = vec![(1 << height) - 1; width];
    let mut blizzards_right = vec![(1 << height) - 1; width];
    let mut blizzards_left = vec![(1 << height) - 1; width];

    for (y, line) in input.lines().skip(1).take(height).enumerate() {
        if line.len() != width + 2 {
            return Err("Not all lines have equal length".to_string());
        }
        for (x, c) in line.bytes().skip(1).take(width).enumerate() {
            match c {
                b'^' => blizzards_up[x] &= !(1 << y),
                b'v' => blizzards_down[x] &= !(1 << y),
                b'>' => blizzards_right[x] &= !(1 << y),
                b'<' => blizzards_left[x] &= !(1 << y),
                _ => {}
            }
        }
    }

    Ok(Valley {
        width,
        height,
        blizzards_up,
        blizzards_down,
        blizzards_right,
        blizzards_left,
    })
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
    test_part_one!(test_input => 18);
    test_part_two!(test_input => 54);

    let real_input = include_str!("day24_input.txt");
    test_part_one!(real_input => 242);
    test_part_two!(real_input => 720);
}
