use crate::input::Input;
use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

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
            let error_message = || format!("Invalid input one line {line_number}");
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

pub fn solve(input: &Input) -> Result<i32, String> {
    let points = parse_input(input.text)?;

    let (left, top, right, bottom) = points.iter().fold(
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        |(left, top, right, bottom), point| {
            (
                cmp::min(left, point.x),
                cmp::min(top, point.y),
                cmp::max(right, point.x),
                cmp::max(bottom, point.y),
            )
        },
    );

    if input.is_part_one() {
        let mut id_to_count = HashMap::new();
        let mut point_ids_with_infinite_area = HashSet::new();

        for y in top..=bottom {
            for x in left..=right {
                let mut closest_distance = i32::MAX;
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
                    point_ids_with_infinite_area.insert(closest_point_id);
                    id_to_count.remove(&closest_point_id);
                } else if !point_ids_with_infinite_area.contains(&closest_point_id) {
                    *id_to_count.entry(closest_point_id).or_insert(0) += 1;
                }
            }
        }

        let max = id_to_count
            .iter()
            .max_by_key(|&(_, &value)| value)
            .ok_or("No solution found")?;
        Ok(*max.1)
    } else {
        let mut sum: i32 = 0;

        for y in top..=bottom {
            for x in left..=right {
                let total_distance = points.iter().fold(0, |acc, point| {
                    acc + (x - point.x).abs() + (y - point.y).abs()
                });
                if total_distance < 10000 {
                    sum += 1;
                }
            }
        }

        Ok(sum)
    }
}

#[test]
fn tests() {
    test_part_one!(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9" => 17
    );

    test_part_one!(
            "0, 0
0, 100
1, 50
80, 20
80, 50
80, 80
100, 0
100, 50
100, 100" => 1876
    );

    let input = include_str!("day06_input.txt");
    test_part_one!(input => 5333);
    test_part_two!(input => 35334);
}
