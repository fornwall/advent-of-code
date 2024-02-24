use crate::common::array_stack::ArrayStack;
use crate::input::{on_error, Input};
use std::ops::RangeInclusive;

pub fn solve(input: &Input) -> Result<u64, String> {
    solve_range(input.text, 200_000_000_000_000..=400_000_000_000_000)
}

pub fn solve_range(input: &str, interval: RangeInclusive<i128>) -> Result<u64, String> {
    let mut particles = ArrayStack::<512, Particle>::new();

    for line in input.lines() {
        let (position_str, speed_str) = line.split_once(" @ ").ok_or_else(on_error)?;
        particles.push(Particle {
            position: Coordinate::parse(position_str)?,
            speed: Coordinate::parse(speed_str)?,
        })?;
    }

    let mut count = 0;

    for (i, a) in particles.slice().iter().enumerate() {
        for b in &particles.elements[i + 1..particles.len()] {
            if a.intersects_xy_with(*b, &interval) {
                count += 1;
            }
        }
    }
    Ok(count)
}

#[derive(Copy, Clone, Default)]
struct Coordinate {
    x: i128,
    y: i128,
    #[allow(dead_code)]
    z: i128,
}

impl Coordinate {
    fn parse(s: &str) -> Result<Self, String> {
        let mut parts = s.split(", ");
        Ok(Self {
            x: parts
                .next()
                .ok_or_else(on_error)?
                .trim()
                .parse()
                .map_err(|_| on_error())?,
            y: parts
                .next()
                .ok_or_else(on_error)?
                .trim()
                .parse()
                .map_err(|_| on_error())?,
            z: parts
                .next()
                .ok_or_else(on_error)?
                .trim()
                .parse()
                .map_err(|_| on_error())?,
        })
    }
}

#[derive(Copy, Clone, Default)]
struct Particle {
    position: Coordinate,
    speed: Coordinate,
}

impl Particle {
    fn intersects_xy_with(&self, other: Self, interval: &RangeInclusive<i128>) -> bool {
        // position_1.x + t0 * speed_1.x = position_2.x + t1 * speed_2.x
        // position_1.y + t0 * speed_1.y = position_2.y + t1 * speed_2.y
        // =>
        // t0 * speed_1.x - t1 * speed_2.x = (position_2.x - position_1.x)
        // t0 * speed_1.y - t1 * speed_2.y = (position_2.y - position_2.y)
        // =>
        // [ speed_1.x -speed2.x ] x [ t0 ] = [ position_2.x-position_1.x ]
        // [ speed_1.x -speed2.y ]   [ t1 ]   [ position_2.y-position_2.y ]
        // =>
        // AX = B where X is unknown
        // =>
        // X = inv(A) * B
        //
        // Inverse of 2x2 matrix:
        // For A = [ a b ]
        //         [ c d ]
        // inv(A) = 1 / (ad - bc) * [  d -b ]
        //                          [ -c  a ]
        let a = self.speed.x;
        let b = -other.speed.x;
        let c = self.speed.y;
        let d = -other.speed.y;
        let b_1 = other.position.x - self.position.x;
        let b_2 = other.position.y - self.position.y;
        let div = a * d - b * c;
        if div == 0 {
            return false;
        }
        let t_1 = (d * b_1 - b * b_2) as f64 / div as f64;
        let t_2 = (-c * b_1 + a * b_2) as f64 / div as f64;
        if t_1 < 0. || t_2 < 0. {
            return false;
        }
        let x = t_1.mul_add(self.speed.x as f64, self.position.x as f64);
        let y = t_1.mul_add(self.speed.y as f64, self.position.y as f64);
        interval.contains(&(x as i128)) && interval.contains(&(y as i128))
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    assert_eq!(solve_range(test_input, 7..=27), Ok(2));

    let real_input = include_str!("day24_input.txt");
    test_part_one_no_allocations!(real_input => 21_679);
    test_part_two_no_allocations!(real_input => 21_679); // FIXME
}
