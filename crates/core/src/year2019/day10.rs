use std::collections::{HashMap, HashSet};

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

pub fn parse_points(input_string: &str) -> Vec<(usize, usize)> {
    input_string
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, character)| match character {
                    '#' => Some((col, row)),
                    _ => None,
                })
        })
        .collect()
}

/// Return (max_seen, (x, y)) of station.
pub fn determine_station(points: &[(usize, usize)]) -> (usize, (usize, usize)) {
    points
        .iter()
        .map(|&this_point| {
            let seen_count = points
                .iter()
                .filter(|&&point| point != this_point)
                .fold(HashSet::new(), |mut seen, other_point| {
                    let mut distance_x = other_point.0 as i64 - this_point.0 as i64;
                    let mut distance_y = other_point.1 as i64 - this_point.1 as i64;
                    let divisor = gcd(distance_x.abs(), distance_y.abs());
                    distance_x /= divisor;
                    distance_y /= divisor;
                    seen.insert((distance_x, distance_y));
                    seen
                })
                .len();
            (seen_count, this_point)
        })
        .max_by_key(|&(seen_count, _)| seen_count)
        .unwrap()
}

pub fn part1(input_string: &str) -> Result<usize, String> {
    let points = parse_points(input_string);
    Ok(determine_station(&points).0)
}

pub fn part2(input_string: &str) -> Result<i64, String> {
    let (x, y) = part2_nth(input_string, 200);
    Ok(x * 100 + y)
}

pub fn part2_nth(input_string: &str, nth: u32) -> (i64, i64) {
    let points = parse_points(input_string);
    let (_, base_location) = determine_station(&points);

    let mut seen = HashMap::new();
    for &(x, y) in points.iter().filter(|&&p| p != base_location) {
        let distance_x = x as i64 - base_location.0 as i64;
        let distance_y = y as i64 - base_location.1 as i64;
        let divisor = gcd(distance_x.abs(), distance_y.abs());
        let direction_x = distance_x / divisor;
        let direction_y = distance_y / divisor;

        seen.entry((direction_x, direction_y))
            .or_insert_with(Vec::new)
            .push((distance_x, distance_y));
    }

    let mut points_grouped_by_direction: Vec<Vec<(i64, i64)>> = seen.values().cloned().collect();

    // Sort each group so that closest points are at end of vector:
    for points_on_line in points_grouped_by_direction.iter_mut() {
        points_on_line.sort_by_key(|point| -((point.0 * point.0 + point.1 * point.1) as i64));
    }

    // Sort between groups in clockwise order:
    points_grouped_by_direction.sort_by(|p1, p2| {
        // Use atan2(x, y) instead of atan2(y, x) to start from y axis,
        // and negate value to get clockwise direction:
        // https://en.wikipedia.org/wiki/Atan2#/media/File:Atan2definition.svg
        let (x, y) = p1[0];
        let a1 = -(x as f64).atan2(y as f64);

        let (x, y) = p2[0];
        let a2 = -(x as f64).atan2(y as f64);

        a1.partial_cmp(&a2).unwrap()
    });

    let mut destroyed_count = 0;
    loop {
        let mut i = 0;
        while i != points_grouped_by_direction.len() {
            let points = &mut points_grouped_by_direction[i];

            let destroyed = points.pop().unwrap();
            destroyed_count += 1;
            if destroyed_count == nth {
                let result_x = destroyed.0 + base_location.0 as i64;
                let result_y = destroyed.1 + base_location.1 as i64;
                return (result_x, result_y);
            }

            if points.is_empty() {
                points_grouped_by_direction.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

#[test]
pub fn tests_part1() {
    assert_eq!(
        part1(
            ".#..#
.....
#####
....#
...##"
        ),
        Ok(8)
    );

    assert_eq!(
        part1(
            "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
        ),
        Ok(33)
    );

    assert_eq!(part1(include_str!("day10_input.txt")), Ok(319));
}

#[test]
fn tests_part2() {
    let input_string = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";
    assert_eq!(part2_nth(input_string, 1), (8, 1));
    assert_eq!(part2_nth(input_string, 2), (9, 0));
    assert_eq!(part2_nth(input_string, 3), (9, 1));
    assert_eq!(part2_nth(input_string, 4), (10, 0));
    assert_eq!(part2_nth(input_string, 5), (9, 2));
    assert_eq!(part2_nth(input_string, 6), (11, 1));
    assert_eq!(part2_nth(input_string, 7), (12, 1));
    assert_eq!(part2_nth(input_string, 8), (11, 2));
    assert_eq!(part2_nth(input_string, 9), (15, 1));
    assert_eq!(part2_nth(input_string, 10), (12, 2));
    assert_eq!(part2_nth(input_string, 11), (13, 2));
    assert_eq!(part2_nth(input_string, 12), (14, 2));
    assert_eq!(part2_nth(input_string, 13), (15, 2));
    assert_eq!(part2_nth(input_string, 14), (12, 3));
    assert_eq!(part2_nth(input_string, 15), (16, 4));

    assert_eq!(part2(include_str!("day10_input.txt")), Ok(517));
}
