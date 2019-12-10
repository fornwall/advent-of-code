use std::collections::HashSet;

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

pub fn part1(input_string: &str) -> String {
    let points: Vec<(usize, usize)> = input_string
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, character)| {
                    if character == '#' {
                        Some((row, col))
                    } else {
                        None
                    }
                })
        })
        .collect();

    let mut max_seen = 0 as usize;
    for (i, this_point) in points.iter().enumerate() {
        let mut seen = HashSet::new();
        for (j, other_point) in points.iter().enumerate() {
            if i != j {
                let mut distance_x = this_point.0 as i64 - other_point.0 as i64;
                let mut distance_y = this_point.1 as i64 - other_point.1 as i64;
                let divisor = gcd(distance_x.abs(), distance_y.abs());
                distance_x /= divisor;
                distance_y /= divisor;
                seen.insert((distance_x, distance_y));
            }
        }
        max_seen = std::cmp::max(max_seen, seen.len());
    }

    max_seen.to_string()
}

pub fn part2(_input_string: &str) -> String {
    String::from("")
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
        "8"
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
        "33"
    );

    assert_eq!(part1(include_str!("day10_input.txt")), "319");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(""), "");

    // assert_eq!(part2(include_str!("day10_input.txt")), "");
}
