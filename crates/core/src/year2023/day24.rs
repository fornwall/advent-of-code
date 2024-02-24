use crate::common::array_stack::ArrayStack;
use crate::input::{on_error, Input};
use std::ops::RangeInclusive;

const D: usize = 6;

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut particles = ArrayStack::<512, Particle>::new();
    for line in input.text.lines() {
        let (position_str, velocity_str) = line.split_once(" @ ").ok_or_else(on_error)?;
        particles.push(Particle {
            position: Coordinate::parse(position_str)?,
            velocity: Coordinate::parse(velocity_str)?,
        })?;
    }

    if input.is_part_one() {
        let mut count = 0;
        let interval = 200_000_000_000_000..=400_000_000_000_000;
        for (i, a) in particles.slice().iter().enumerate() {
            for b in &particles.elements[i + 1..particles.len()] {
                if a.intersects_xy_with(*b, &interval) {
                    count += 1;
                }
            }
        }
        Ok(count)
    } else {
        // From https://raw.githubusercontent.com/sebastianotronto/aoc/master/2023/24/24b.c

        // "To figure the correct starting point + velocity, we solve some systems of linear equations.
        // Write your starting point and velocity as unknowns x, y, z, Vx, Vy, Vz.
        // Equating the position of the rock at time t1 (another unknown parameter) with the position
        // of one of the hailstones at the same time t1, we get a system of 3 equations and 7 unknowns.
        // Unfortunately, these equations have degree 2 - this is not a linear system.
        // However, manipulating these equations a bit we can get a linear equation of the type:
        //     (Vy1-Vy2)x - (Vx1-Vx2)y + - (y1-y2)Vx + (x1-x2)Vy = y2Vx2 + x2Vy2 - y1Vx1 - x1Vy1
        // Where x1, y1, z2, Vx1, Vy1, Vz1 and x2, y2, z2, Vx2, Vy2, Vz2
        // are the starting points and velocities of two of the hailstones.
        // So with 2 lines we can get a linear equation. Similarly, we can
        // get equations involving the unknowns z and Vz.
        // We can use the myriad of hailstones we have to generate as many
        // equations as we like. The system is going to be overdetermined,
        // but the problem statement seems to ensure that there is going to
        // be a solution. On the other hand it can happen that we make a
        // bad choice of lines and the equation we use are underdetermined.
        // This last problem is not accounted for in the code - if it happens,
        // one can shuffle the input file until it works."
        let l = [
            particles.elements[0].to_float(),
            particles.elements[1].to_float(),
            particles.elements[2].to_float(),
        ];

        let a: [[f64; D]; D] = [
            // First equation: Particles 0 and 1, x and y only.
            [
                l[0].v.y - l[1].v.y,
                l[1].v.x - l[0].v.x,
                0.0,
                l[1].p.y - l[0].p.y,
                l[0].p.x - l[1].p.x,
                0.0,
            ],
            // Second equation: Particles 0 and 2, x and y only.
            [
                l[0].v.y - l[2].v.y,
                l[2].v.x - l[0].v.x,
                0.0,
                l[2].p.y - l[0].p.y,
                l[0].p.x - l[2].p.x,
                0.0,
            ],
            // Third equation: Particles 0 and 1, x and z only.
            [
                l[0].v.z - l[1].v.z,
                0.0,
                l[1].v.x - l[0].v.x,
                l[1].p.z - l[0].p.z,
                0.0,
                l[0].p.x - l[1].p.x,
            ],
            // Fourth equation: Particles 0 and 2, x and z only.
            [
                l[0].v.z - l[2].v.z,
                0.0,
                l[2].v.x - l[0].v.x,
                l[2].p.z - l[0].p.z,
                0.0,
                l[0].p.x - l[2].p.x,
            ],
            // Fifth equation: Particles 0 and 1, y and z only.
            [
                0.0,
                l[0].v.z - l[1].v.z,
                l[1].v.y - l[0].v.y,
                0.0,
                l[1].p.z - l[0].p.z,
                l[0].p.y - l[1].p.y,
            ],
            // Sixth equation: Particles 0 and 2, y and z only.
            [
                0.0,
                l[0].v.z - l[2].v.z,
                l[2].v.y - l[0].v.y,
                0.0,
                l[2].p.z - l[0].p.z,
                l[0].p.y - l[2].p.y,
            ],
        ];
        #[allow(clippy::suboptimal_flops)]
        let c: [f64; D] = [
            // First equation: Particles 0 and 1, x and y only.
            l[1].p.y * l[1].v.x - l[1].p.x * l[1].v.y - l[0].p.y * l[0].v.x + l[0].p.x * l[0].v.y,
            // Second equation: Particles 0 and 2, x and y only.
            l[2].p.y * l[2].v.x - l[2].p.x * l[2].v.y - l[0].p.y * l[0].v.x + l[0].p.x * l[0].v.y,
            // Third equation: Particles 0 and 1, x and z only.
            l[1].p.z * l[1].v.x - l[1].p.x * l[1].v.z - l[0].p.z * l[0].v.x + l[0].p.x * l[0].v.z,
            // Fourth equation: Particles 0 and 2, x and z only.
            l[2].p.z * l[2].v.x - l[2].p.x * l[2].v.z - l[0].p.z * l[0].v.x + l[0].p.x * l[0].v.z,
            // Fifth equation: Particles 0 and 1, y and z only.
            l[1].p.z * l[1].v.y - l[1].p.y * l[1].v.z - l[0].p.z * l[0].v.y + l[0].p.y * l[0].v.z,
            // Sixth equation: Particles 0 and 2, y and z only.
            l[2].p.z * l[2].v.y - l[2].p.y * l[2].v.z - l[0].p.z * l[0].v.y + l[0].p.y * l[0].v.z,
        ];
        if let Some([x, y, z]) = solve_system(a, c) {
            Ok(x.round() as u64 + y.round() as u64 + z.round() as u64)
        } else {
            Err("No solution found".to_string())
        }
    }
}

fn float_eq(x: f64, y: f64) -> bool {
    const EPSILON: f64 = 1e-10;
    (x - y).abs() < EPSILON * (x.abs() + y.abs())
}

// Solve the linear system a*x = c with row reduction.
// Return Some(c) if it has a unique solution, otherwise None.
fn solve_system(mut a: [[f64; D]; D], mut c: [f64; D]) -> Option<[f64; 3]> {
    // Row reduction.
    for i in 0..D {
        // Make first nonzero.
        let mut imax = 0;
        let mut maxi = 0.0;

        for (j, aj) in a.iter().enumerate().skip(i) {
            if aj[i].abs() > maxi {
                maxi = aj[i].abs();
                imax = j;
            }
        }

        if float_eq(maxi, 0.0) {
            return None;
        }

        c.swap(i, imax);

        for j in 0..D {
            let tmp = a[i][j];
            a[i][j] = a[imax][j];
            a[imax][j] = tmp;
        }

        // Reduce rows.
        for ii in (i + 1)..D {
            let r = a[ii][i] / a[i][i];
            for k in i..D {
                a[ii][k] -= r * a[i][k];
            }
            c[ii] -= r * c[i];
        }
    }

    // Back substitution.
    let mut x = c;
    for i in (0..=(D - 1)).rev() {
        for j in (i + 1)..D {
            x[i] -= a[i][j] * x[j];
        }
        x[i] /= a[i][i];
    }

    Some([x[0], x[1], x[2]])
}

#[derive(Copy, Clone, Default)]
struct Coordinate {
    x: i128,
    y: i128,
    #[allow(dead_code)]
    z: i128,
}

#[derive(Copy, Clone, Default)]
struct FloatCoordinate {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone, Default)]
struct FloatParticle {
    p: FloatCoordinate,
    v: FloatCoordinate,
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

    const fn to_float(self) -> FloatCoordinate {
        FloatCoordinate {
            x: self.x as f64,
            y: self.y as f64,
            z: self.z as f64,
        }
    }
}

#[derive(Copy, Clone, Default)]
struct Particle {
    position: Coordinate,
    velocity: Coordinate,
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
        let a = self.velocity.x;
        let b = -other.velocity.x;
        let c = self.velocity.y;
        let d = -other.velocity.y;
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
        let x = t_1.mul_add(self.velocity.x as f64, self.position.x as f64);
        let y = t_1.mul_add(self.velocity.y as f64, self.position.y as f64);
        interval.contains(&(x as i128)) && interval.contains(&(y as i128))
    }

    const fn to_float(self) -> FloatParticle {
        FloatParticle {
            p: self.position.to_float(),
            v: self.velocity.to_float(),
        }
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let real_input = include_str!("day24_input.txt");
    test_part_one_no_allocations!(real_input => 21_679);
    test_part_two_no_allocations!(real_input => 566_914_635_762_564);
}
