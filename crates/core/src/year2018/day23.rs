use std::cmp::{max, min};

#[derive(Debug)]
struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    strength: i64,
}

impl Nanobot {
    fn parse(input_string: &str) -> Vec<Nanobot> {
        input_string
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line
                    .split(|c| c == '<' || c == '>' || c == ',' || c == '=')
                    .collect();
                //println!("{:?}", parts);
                Nanobot {
                    x: parts[2].parse::<i64>().unwrap(),
                    y: parts[3].parse::<i64>().unwrap(),
                    z: parts[4].parse::<i64>().unwrap(),
                    strength: parts[7].parse::<i64>().unwrap(),
                }
            })
            .collect()
    }

    fn distance_to_bot(&self, other: &Nanobot) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn distance_to_point(&self, x: i64, y: i64, z: i64) -> i64 {
        (self.x - x).abs() + (self.y - y).abs() + (self.z - z).abs()
    }

    fn is_bot_within_range(&self, other: &Nanobot) -> bool {
        self.distance_to_bot(other) <= self.strength
    }

    fn is_point_within_range(&self, x: i64, y: i64, z: i64) -> bool {
        self.distance_to_point(x, y, z) <= self.strength
    }
}
pub fn part1(input_string: &str) -> String {
    let bots = Nanobot::parse(input_string);
    let strongest_bot = bots
        .iter()
        .max_by(|x, y| x.strength.cmp(&y.strength))
        .unwrap();
    bots.iter()
        .filter(|&bot| strongest_bot.is_bot_within_range(bot))
        .count()
        .to_string()
}

pub fn part2(input_string: &str) -> String {
    let bots = Nanobot::parse(input_string);

    let (mut min_x, mut max_x, mut min_y, mut max_y, mut min_z, mut max_z) =
        bots.iter().fold((0, 0, 0, 0, 0, 0), |acc, bot| {
            (
                min(acc.0, bot.x),
                max(acc.1, bot.x),
                min(acc.2, bot.y),
                max(acc.3, bot.y),
                min(acc.4, bot.z),
                max(acc.5, bot.z),
            )
        });

    let mut range = 1;
    while range < max_x - min_x && range < max_y - min_y && range < max_z - min_z {
        range *= 2;
    }

    loop {
        let mut best_bots_within_range = 0;
        let mut best_point = (0, 0, 0);
        let mut best_distance = 0;

        for x in (min_x..=max_x).step_by(range as usize) {
            for y in (min_y..=max_y).step_by(range as usize) {
                for z in (min_z..=max_z).step_by(range as usize) {
                    let distance_from_origin = x.abs() + y.abs() + z.abs();

                    let bots_within_range = bots
                        .iter()
                        .filter(|&b| b.is_point_within_range(x, y, z))
                        .count();

                    // "Find the coordinates that are in range of the largest number of nanobots.
                    // What is the shortest manhattan distance between any of those points and 0,0,0?"
                    if bots_within_range > best_bots_within_range
                        || (bots_within_range == best_bots_within_range
                            && distance_from_origin < best_distance)
                    {
                        best_bots_within_range = bots_within_range;
                        best_distance = distance_from_origin;
                        best_point = (x, y, z);
                    }
                }
            }
        }

        if range == 1 {
            return best_distance.to_string();
        }

        min_x = best_point.0 - range;
        max_x = best_point.0 + range;
        min_y = best_point.1 - range;
        max_y = best_point.1 + range;
        min_z = best_point.2 - range;
        max_z = best_point.2 + range;

        range /= 2;
    }
}

#[test]
fn tests_part1() {
    assert_eq!(
        "7",
        part1(
            "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"
        )
    );

    assert_eq!("270", part1(include_str!("day23_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(
        "36",
        part2(
            "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"
        )
    );

    assert_eq!("106323091", part2(include_str!("day23_input.txt")));
}
