use crate::{
    common::array_stack::ArrayStack,
    input::{on_error, Input},
};

use std::ops::RangeInclusive;

const RANGE: RangeInclusive<i64> = 200_000_000_000_000..=400_000_000_000_000;

pub const fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

#[derive(Clone, Copy)]
struct Vector {
    x: i128,
    y: i128,
    z: i128,
}

/// 3D vector implementation.
impl Vector {
    const fn add(self, other: Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Self { x, y, z }
    }

    const fn sub(self, other: Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Self { x, y, z }
    }

    const fn cross(self, other: Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Self { x, y, z }
    }

    // Changes the magnitude (but not direction) of the vector.
    // Prevents numeric overflow.
    const fn gcd(self) -> Self {
        let gcd = gcd(gcd(self.x, self.y), self.z);
        let x = self.x / gcd;
        let y = self.y / gcd;
        let z = self.z / gcd;
        Self { x, y, z }
    }

    const fn sum(self) -> i128 {
        self.x + self.y + self.z
    }
}

/// Solution from https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day25.rs
pub fn solve(input: &Input) -> Result<u64, String> {
    let mut i = ArrayStack::<512, [i64; 6]>::new();
    for line in input.text.lines() {
        let (position_str, velocity_str) = line.split_once(" @ ").ok_or_else(on_error)?;
        let mut p_parts = position_str.split(", ");
        let mut v_parts = velocity_str.split(", ");
        i.push([
            p_parts
                .next()
                .ok_or_else(on_error)?
                .parse()
                .map_err(|_| on_error())?,
            p_parts
                .next()
                .ok_or_else(on_error)?
                .parse()
                .map_err(|_| on_error())?,
            p_parts
                .next()
                .ok_or_else(on_error)?
                .parse()
                .map_err(|_| on_error())?,
            v_parts
                .next()
                .ok_or_else(on_error)?
                .parse()
                .map_err(|_| on_error())?,
            v_parts
                .next()
                .ok_or_else(on_error)?
                .parse()
                .map_err(|_| on_error())?,
            v_parts
                .next()
                .ok_or_else(on_error)?
                .parse()
                .map_err(|_| on_error())?,
        ])?;
    }

    if input.is_part_one() {
        Ok(part1(i.slice()) as u64)
    } else {
        Ok(part2(i.slice()) as u64)
    }
}

pub fn part1(input: &[[i64; 6]]) -> u32 {
    let mut result = 0;

    for (index, &[a, b, _, c, d, _]) in input[1..].iter().enumerate() {
        for &[e, f, _, g, h, _] in &input[..index + 1] {
            // If the determinant is zero there is no solution possible
            // which implies the trajectories are parallel.
            let determinant = d * g - c * h;
            if determinant == 0 {
                continue;
            }

            // Invert the 2x2 matrix then multiply by the respective columns to find the times.
            let t = (g * (f - b) - h * (e - a)) / determinant;
            let u = (c * (f - b) - d * (e - a)) / determinant;

            // We can pick either the first or second hailstone to find the intersection position.
            let x = a + t * c;
            let y = b + t * d;

            // Both times must be in the future and the position within the specified area.
            if t >= 0 && u >= 0 && RANGE.contains(&x) && RANGE.contains(&y) {
                result += 1;
            }
        }
    }

    result
}

pub fn part2(input: &[[i64; 6]]) -> i128 {
    // Calculations need the range of `i128`.
    let widen = |i: usize| {
        let [px, py, pz, vx, vy, vz] = input[i].map(|n| n as i128);
        let p = Vector {
            x: px,
            y: py,
            z: pz,
        };
        let v = Vector {
            x: vx,
            y: vy,
            z: vz,
        };
        (p, v)
    };

    // Take 3 arbitrary hailstones.
    let (p0, v0) = widen(0);
    let (p1, v1) = widen(1);
    let (p2, v2) = widen(2);

    // Subtract the positions and velocities to make them relative.
    // The first hailstone is stationary at the origin.
    let p3 = p1.sub(p0);
    let p4 = p2.sub(p0);
    let v3 = v1.sub(v0);
    let v4 = v2.sub(v0);

    // Find the normal to the plane that the second and third hailstones velocity lies in.
    // This is the cross product of their respective position and velocity.
    // The cross product `s` of these two vectors is the same direction but not necessarily the
    // same magnitude of the desired velocity of the rock.
    // Only the direction is relevant (not the magnitude) so we can normalize the vector by the
    // GCD of its components in order to prevent numeric overflow.
    let q = v3.cross(p3).gcd();
    let r = v4.cross(p4).gcd();
    let s = q.cross(r).gcd();

    // Find the times when the second and third hailstone intercept this vector.
    // If the times are different then we can extrapolate the original position of the rock.
    let t = (p3.y * s.x - p3.x * s.y) / (v3.x * s.y - v3.y * s.x);
    let u = (p4.y * s.x - p4.x * s.y) / (v4.x * s.y - v4.y * s.x);
    assert!(t != u);

    // Calculate the original position of the rock, remembering to add the first hailstone's
    // position to convert back to absolute coordinates.
    let a = p0.add(p3).sum();
    let b = p0.add(p4).sum();
    let c = v3.sub(v4).sum();
    (u * a - t * b + u * t * c) / (u - t)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let real_input = include_str!("day24_input.txt");
    test_part_one_no_allocations!(real_input => 21_679);
    test_part_two_no_allocations!(real_input => 566_914_635_762_564);
}
