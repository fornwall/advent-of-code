use crate::input::Input;
use std::ops::RangeInclusive;

pub fn solve(input: &Input) -> Result<i32, String> {
    let trench = Trench::parse(input.text).ok_or_else(|| "Unable to parse trench".to_string())?;

    let mut max_y = 0;
    let mut count = 0;
    let max_x_to_try = *trench.x_range.end();
    let min_y_to_try = *trench.y_range.start();
    for initial_y_velocity in min_y_to_try..=1000 {
        for initial_x_velocity in 1..=max_x_to_try {
            if let Some(value) =
                probes_ends_in_trench(initial_x_velocity, initial_y_velocity, &trench)
            {
                max_y = value;
                count += 1;
            }
        }
    }
    Ok(input.part_values(max_y, count))
}

struct Trench {
    x_range: RangeInclusive<i16>,
    y_range: RangeInclusive<i16>,
}

impl Trench {
    fn parse(text: &str) -> Option<Self> {
        let target_area = if text.len() < 18 {
            return None;
        } else {
            &text[15..]
        };
        let (x_range, y_range) = if let Some((x_range, y_range)) = target_area.split_once(", y=") {
            (Self::parse_range(x_range)?, Self::parse_range(y_range)?)
        } else {
            return None;
        };
        Some(Self { x_range, y_range })
    }

    fn parse_range(range: &str) -> Option<RangeInclusive<i16>> {
        if let Some((start, end)) = range.split_once("..") {
            let a = start.parse::<i16>().ok()?;
            let b = end.parse::<i16>().ok()?;
            Some(std::cmp::min(a, b)..=std::cmp::max(a, b))
        } else {
            None
        }
    }
}

fn probes_ends_in_trench(
    horizontal_starting_velocity: i16,
    vertical_starting_velocity: i16,
    trench: &Trench,
) -> Option<i32> {
    let mut x = 0;
    let mut y = 0_i32;
    let mut dx = horizontal_starting_velocity;
    let mut dy = i32::from(vertical_starting_velocity);
    let mut max_y = 0;
    loop {
        x += dx;
        y += dy;
        dx -= dx.signum();
        dy -= 1;
        max_y = std::cmp::max(y, max_y);

        if y > i32::from(i16::MAX) {
            continue;
        }

        if !trench.x_range.contains(&x) {
            let stopped_by_drag = dx == 0;
            let passed_trench = x > *trench.x_range.end();
            if stopped_by_drag || passed_trench {
                return None;
            } else {
                continue;
            }
        }

        if !trench.y_range.contains(&(y as i16)) {
            let below_trench = (y as i16) < *trench.y_range.end();
            let falling_down = dy < 0;
            if below_trench && falling_down {
                return None;
            } else {
                continue;
            }
        }

        return Some(max_y);
    }
}

#[test]
pub fn tests() {
    let example = "target area: x=20..30, y=-10..-5";
    test_part_one!(example => 45);
    test_part_two!(example => 112);

    let real_input = include_str!("day17_input.txt");
    test_part_one!(real_input => 7381);
    test_part_two!(real_input => 3019);
}
