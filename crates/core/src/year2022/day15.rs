// Adaption of https://github.com/SLiV9/AdventOfCode2022/blob/main/src/bin/day15/main.rs
use crate::input::Input;

const MAX_COORDINATE: i32 = 4_000_000;

pub fn solve(input: &Input) -> Result<u64, String> {
    let sensors = Sensor::parse(input.text).ok_or_else(|| "Invalid input".to_string())?;

    if input.is_part_one() {
        Ok(solve_part_1(&sensors))
    } else {
        solve_part_2(&sensors)
    }
}

type Point = (i32, i32);

#[derive(Copy, Clone)]
struct Sensor {
    position: Point,
    closest_beacon: Point,
    range: u32,
}

impl Sensor {
    fn parse(input: &str) -> Option<Vec<Self>> {
        fn parse_x_y(input: &str) -> Option<Point> {
            let mut parts = input.split(", y=");
            Some((parts.next()?.parse().ok()?, parts.next()?.parse().ok()?))
        }

        input
            .lines()
            .map(|line| {
                if line.len() < 20 {
                    return None;
                }
                let mut parts = line[12..].split(": closest beacon is at x=");
                let position = parse_x_y(parts.next()?)?;
                let closest_beacon = parse_x_y(parts.next()?)?;
                let range =
                    position.0.abs_diff(closest_beacon.0) + position.1.abs_diff(closest_beacon.1);
                Some(Self {
                    position,
                    closest_beacon,
                    range,
                })
            })
            .collect()
    }

    const fn contains(&self, position: Point) -> bool {
        self.position.0.abs_diff(position.0) + self.position.1.abs_diff(position.1) <= self.range
    }
}

fn solve_part_1(sensors: &[Sensor]) -> u64 {
    let mut not_possible_intervals = sensors
        .iter()
        .filter_map(|sensor| {
            const ROW: i32 = 2_000_000;

            // Consider the sensor at S:
            //
            //     .........
            //     ....#....
            // R-> ...###...
            //     ..##S##..
            //     ...###...
            //     ....#....
            //     .........
            //
            // The intersection at row R is at:
            //   x_start = S_x - radius + abs(R_y - S_y)
            //   x_end   = S_x + radius - abs(R_y - S_y)
            if ROW.abs_diff(sensor.position.1) <= sensor.range {
                let intersection_distance = ROW.abs_diff(sensor.position.1) as i32;
                let x_start = sensor.position.0 - sensor.range as i32 + intersection_distance;
                let x_end = sensor.position.0 + sensor.range as i32 - intersection_distance;
                // We then need to adjust the intersection if the beacon is there:
                //   R-> ...B#B...
                // That is done by adding one to x_start, or subtracting one from x_end, if necessary.
                Some((
                    x_start + i32::from((x_start, ROW) == sensor.closest_beacon),
                    x_end - i32::from((x_end, ROW) == sensor.closest_beacon),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut not_possible_positions_count = 0;
    let mut last_interval = (i32::MIN + 1, i32::MIN);

    not_possible_intervals.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    for interval in not_possible_intervals {
        if interval.0 <= last_interval.1 && interval.1 >= last_interval.0 {
            last_interval = (
                interval.0.min(last_interval.0),
                interval.1.max(last_interval.1),
            );
        } else {
            not_possible_positions_count += last_interval.1 - last_interval.0 + 1;
            last_interval = interval;
        }
    }
    not_possible_positions_count += last_interval.1 - last_interval.0 + 1;

    not_possible_positions_count as u64
}

/// Since there is only one possible space, it (unless it's at the
/// edge of the boundary) needs to be where diagonal lines along the
/// outer edges of the diamond shaped sensor covered areas:
///
/// ..\......./...
/// ...\..#../....
/// ....\#S#/#....
/// ....#\#/####..
/// ...#S#.##S###.
/// ....#/#\###...
/// ..../###\#....
/// .../##S##\....
/// ../.......\...
///
/// These two diagonal lines must come from two different sensors.
///
/// The possible space could also be along the edge of the boundary.
fn solve_part_2(sensors: &[Sensor]) -> Result<u64, String> {
    let ascending_lines = diagonal_line_candidates(sensors, true);
    let descending_lines = diagonal_line_candidates(sensors, false);

    for &ascending_line in ascending_lines.iter() {
        for &descending_line in descending_lines.iter() {
            let intersection = intersection_of(ascending_line, descending_line);
            if is_within_bounds(intersection)
                && !sensors.iter().any(|sensor| sensor.contains(intersection))
            {
                return Ok((intersection.0 as u64) * 4_000_000 + (intersection.1 as u64));
            }
        }
    }

    // Also check the edge of the boundary:
    for offset in 0..=MAX_COORDINATE {
        for position in [
            (0, offset),
            (MAX_COORDINATE, offset),
            (offset, 0),
            (offset, MAX_COORDINATE),
        ] {
            if !sensors.iter().any(|sensor| sensor.contains(position)) {
                return Ok((position.0 as u64) * 4_000_000 + (position.1 as u64));
            }
        }
    }

    Err("No solution found".to_string())
}

/// Returns diagonal lines along the outer edge of sensors, represented as an i32
/// for the x origin. Only duplicated lines are returned, as they are the only
/// candidates for intersections at the only possible space.
fn diagonal_line_candidates(sensors: &[Sensor], ascending: bool) -> Vec<i32> {
    let y_multiplier = if ascending { -1 } else { 1 };

    let mut diagonal_lines = sensors
        .iter()
        .flat_map(|sensor| {
            let outer_edge_leftmost = sensor.position.0 - sensor.range as i32 - 1;
            let outer_edge_rightmost = sensor.position.0 + sensor.range as i32 + 1;
            // Contains the x origins of diagonal (either ascending or descending) lines:
            // .................
            // .........#.......
            // .......x#S#x.....
            // ....../.\#/.\....
            // ...../...X...\...
            // ..../.../.\...\..
            // ---a---a---d---d-
            [
                outer_edge_leftmost + y_multiplier * sensor.position.1,
                outer_edge_rightmost + y_multiplier * sensor.position.1,
            ]
        })
        .collect::<Vec<_>>();

    diagonal_lines.sort_unstable();
    let mut diagonal_lines = diagonal_lines
        .windows(2)
        .filter_map(|window| (window[0] == window[1]).then_some(window[0]))
        .collect::<Vec<_>>();
    diagonal_lines.dedup();
    diagonal_lines
}

const fn is_within_bounds(position: Point) -> bool {
    position.0 >= 0
        && position.0 <= MAX_COORDINATE
        && position.1 >= 0
        && position.1 <= MAX_COORDINATE
}

/// Given the x origin of an ascending (represented with 'a' below) and
/// a descending (represented with 'd' below) line, return the point of
/// intersection (represented with 'x' below):
///
/// ...............
/// .......x.......
/// ....../.\......
/// ...../...\.....
/// ..../.....\....
/// ---a---h---d---
const fn intersection_of(ascending_origin_x: i32, descending_origin_x: i32) -> Point {
    let halfway = (descending_origin_x - ascending_origin_x) / 2;
    (ascending_origin_x + halfway, halfway)
}

#[test]
pub fn tests() {
    let real_input = include_str!("day15_input.txt");
    test_part_one!(real_input => 5_240_818);
    test_part_two!(real_input => 13_213_086_906_101);
}
