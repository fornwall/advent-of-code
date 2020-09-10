use std::cmp::{max, min};
use std::collections::HashSet;

struct Point {
    x: i32,
    y: i32,
    x_speed: i32,
    y_speed: i32,
}

pub fn part1(input_string: &str) -> String {
    find_letters(input_string).0
}

pub fn part2(input_string: &str) -> String {
    find_letters(input_string).1.to_string()
}

pub fn find_letters(input_string: &str) -> (String, u32) {
    let mut points: Vec<Point> = input_string
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(|c| c == '<' || c == '>' || c == ',').collect();

            let x = parts[1].trim().parse::<i32>().unwrap();
            let y = parts[2].trim().parse::<i32>().unwrap();
            let x_speed = parts[4].trim().parse::<i32>().unwrap();
            let y_speed = parts[5].trim().parse::<i32>().unwrap();
            Point {
                x,
                y,
                x_speed,
                y_speed,
            }
        })
        .collect();

    let mut previous_height = std::i32::MAX;
    let mut seconds = 0;
    loop {
        let mut min_y = std::i32::MAX;
        let mut max_y = std::i32::MIN;
        for point in &mut points {
            point.x += point.x_speed;
            point.y += point.y_speed;
            min_y = min(min_y, point.y);
            max_y = max(max_y, point.y);
        }

        let this_height = max_y - min_y;
        if this_height > previous_height {
            break;
        };
        previous_height = this_height;
        seconds += 1;
    }

    let mut occupied = HashSet::new();
    let mut borders = (std::i32::MAX, std::i32::MIN, std::i32::MIN, std::i32::MAX);
    for point in &mut points {
        // Step back efter last expanding step.
        point.x -= point.x_speed;
        point.y -= point.y_speed;

        borders.0 = min(borders.0, point.y);
        borders.1 = max(borders.1, point.x);
        borders.2 = max(borders.2, point.y);
        borders.3 = min(borders.3, point.x);

        occupied.insert((point.x, point.y));
    }

    let mut result = String::new();
    for y in borders.0..=borders.2 {
        for x in borders.3..=borders.1 {
            result += if occupied.contains(&(x, y)) { "#" } else { "." }
        }
        result += "\n";
    }

    (result, seconds)
}

#[test]
fn tests_part1() {
    assert_eq!(
        "#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
",
        part1(
            "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"
        )
    );

    assert_eq!(
        "#....#..#....#.....###..######....##....#....#....##....######
#....#..#...#.......#...#........#..#...#...#....#..#...#.....
#....#..#..#........#...#.......#....#..#..#....#....#..#.....
#....#..#.#.........#...#.......#....#..#.#.....#....#..#.....
######..##..........#...#####...#....#..##......#....#..#####.
#....#..##..........#...#.......######..##......######..#.....
#....#..#.#.........#...#.......#....#..#.#.....#....#..#.....
#....#..#..#....#...#...#.......#....#..#..#....#....#..#.....
#....#..#...#...#...#...#.......#....#..#...#...#....#..#.....
#....#..#....#...###....#.......#....#..#....#..#....#..#.....
",
        part1(include_str!("day10_input.txt"))
    );
}

#[test]
fn tests_part2() {
    assert_eq!(
        "3",
        part2(
            "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"
        )
    );

    assert_eq!("10888", part2(include_str!("day10_input.txt")));
}
