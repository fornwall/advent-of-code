use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;

struct Point {
    id: i32,
    x: i32,
    y: i32,
}

fn parse_input(input_string: &str) -> Result<Vec<Point>, String> {
    input_string
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            let line_number = (line_index + 1) as i32;
            let parts: Vec<&str> = line.split(", ").collect();
            let error_message = || format!("Invalid input one line {}: {}", line_number, line);
            if parts.len() != 2 {
                return Err(error_message());
            }
            let x = parts[0].parse::<i32>().map_err(|_| error_message())?;
            let y = parts[1].parse::<i32>().map_err(|_| error_message())?;
            Ok(Point {
                id: line_number,
                x,
                y,
            })
        })
        .collect::<Result<Vec<Point>, String>>()
}

pub fn part1(input_string: &str) -> Result<i32, String> {
    let points = parse_input(input_string)?;

    let (left, top, right, bottom) = points.iter().fold(
        (std::i32::MAX, std::i32::MAX, std::i32::MIN, std::i32::MIN),
        |(left, top, right, bottom), point| {
            (
                cmp::min(left, point.x),
                cmp::min(top, point.y),
                cmp::max(right, point.x),
                cmp::max(bottom, point.y),
            )
        },
    );

    let mut id_to_count = HashMap::new();

    for y in top..=bottom {
        for x in left..=right {
            let mut closest_distance = std::i32::MAX;
            let mut closest_point_id = -1;

            for point in points.iter() {
                let distance = (x - point.x).abs() + (y - point.y).abs();

                match distance.cmp(&closest_distance) {
                    Ordering::Greater => {}
                    Ordering::Less => {
                        closest_distance = distance;
                        closest_point_id = point.id;
                    }
                    Ordering::Equal => {
                        closest_point_id = -1;
                    }
                };
            }

            if x == left || x == right || y == top || y == bottom {
                // These points have infinite area, so do not count them:
                id_to_count.remove(&closest_point_id);
            } else {
                *id_to_count.entry(closest_point_id).or_insert(0) += 1;
            }
        }
    }

    let max = id_to_count
        .iter()
        .max_by_key(|(_, &value)| value)
        .ok_or("No solution found")?;
    Ok(*max.1)
}

pub fn part2_param(input_string: &str, max_distance_exclusive: i32) -> Result<i32, String> {
    let points = parse_input(input_string)?;

    let (left, top, right, bottom) = points.iter().fold(
        (std::i32::MAX, std::i32::MAX, std::i32::MIN, std::i32::MIN),
        |(left, top, right, bottom), point| {
            (
                cmp::min(left, point.x),
                cmp::min(top, point.y),
                cmp::max(right, point.x),
                cmp::max(bottom, point.y),
            )
        },
    );

    let mut sum: i32 = 0;

    for y in top..=bottom {
        for x in left..=right {
            let total_distance = points.iter().fold(0, |acc, point| {
                acc + (x - point.x).abs() + (y - point.y).abs()
            });
            if total_distance < max_distance_exclusive {
                sum += 1;
            }
        }
    }

    Ok(sum)
}

pub fn part2(input_string: &str) -> Result<i32, String> {
    part2_param(input_string, 10000)
}

#[test]
fn tests_part1() {
    assert_eq!(
        Ok(17),
        part1(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"
        )
    );

    assert_eq!(
        Ok(1876),
        part1(
            "0, 0
0, 100
1, 50
80, 20
80, 50
80, 80
100, 0
100, 50
100, 100"
        )
    );

    assert_eq!(Ok(5333), part1(include_str!("day06_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(
        Ok(16),
        part2_param(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
            32
        )
    );

    assert_eq!(Ok(35334), part2(include_str!("day06_input.txt")));
}
