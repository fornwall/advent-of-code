#[derive(Debug, Clone)]
struct Moons {
    positions: [[i32; 3]; 4],
    velocities: [[i32; 3]; 4],
}

pub const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

pub const fn lcd(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub const fn lcd3(a: u64, b: u64, c: u64) -> u64 {
    lcd(a, lcd(b, c))
}

impl Moons {
    fn parse(input: &str) -> Result<Self, String> {
        let mut positions = [[0; 3]; 4];
        for (i, line) in input.lines().enumerate() {
            let error_message = |_| format!("Invalid line: {}", i + 1);
            let parts: Vec<&str> = line
                .split(|c| c == '=' || c == ' ' || c == '>' || c == ',')
                .collect();

            if parts.len() != 9 {
                return Err(format!("Invalid line: {}", i + 1));
            }

            positions[i][0] = parts[1].trim().parse::<i32>().map_err(error_message)?;
            positions[i][1] = parts[4].trim().parse::<i32>().map_err(error_message)?;
            positions[i][2] = parts[7].trim().parse::<i32>().map_err(error_message)?;
        }

        Ok(Self {
            positions,
            velocities: [[0; 3]; 4],
        })
    }

    fn total_energy(&self) -> u64 {
        let mut total_energy = 0_u64;
        for i in 0..4 {
            let potential_energy = self.positions[i].iter().map(|&x| x.abs()).sum::<i32>() as u64;
            let kinetic_energy = self.velocities[i].iter().map(|&x| x.abs()).sum::<i32>() as u64;
            total_energy += potential_energy * kinetic_energy;
        }
        total_energy
    }

    fn step(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                if i != j {
                    for k in 0..3 {
                        self.velocities[i][k] +=
                            signum(self.positions[j][k] - self.positions[i][k]);
                    }
                }
            }
        }

        for (i, position) in self.positions.iter_mut().enumerate() {
            for (k, p) in position.iter_mut().enumerate() {
                *p += self.velocities[i][k];
            }
        }
    }
}

const fn signum(value: i32) -> i32 {
    match value {
        _ if value < 0 => -1,
        _ if value == 0 => 0,
        _ => 1,
    }
}

pub fn part1_nth(input_string: &str, n: usize) -> Result<u64, String> {
    let mut moons = Moons::parse(input_string)?;
    for _ in 0..n {
        moons.step();
    }
    Ok(moons.total_energy())
}

pub fn part1(input_string: &str) -> Result<u64, String> {
    part1_nth(input_string, 1000)
}

pub fn part2(input_string: &str) -> Result<u64, String> {
    let mut moons = Moons::parse(input_string)?;
    let initial_moons = moons.clone();
    let mut cycles: [Option<u64>; 3] = [None; 3];

    let mut step = 0;
    while cycles.iter().any(|x| x.is_none()) {
        moons.step();
        step += 1;

        for (i, cycle) in cycles.iter_mut().enumerate() {
            if cycle.is_none() {
                let mut same = true;
                for moon in 0..4 {
                    if initial_moons.positions[moon][i] != moons.positions[moon][i]
                        || initial_moons.velocities[moon][i] != moons.velocities[moon][i]
                    {
                        same = false;
                    }
                }
                if same {
                    cycle.replace(step);
                }
            }
        }
    }

    Ok(lcd3(
        cycles[0].ok_or("Cycles not found")?,
        cycles[1].ok_or("Cycles not found")?,
        cycles[2].ok_or("Cycles not found")?,
    ))
}

#[test]
pub fn tests_part1() {
    assert_eq!(
        part1_nth(
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
            10
        ),
        Ok(179)
    );

    assert_eq!(part1(include_str!("day12_input.txt")), Ok(6220));
}

#[test]
fn tests_part2() {
    assert_eq!(
        part2(include_str!("day12_input.txt")),
        Ok(548_525_804_273_976)
    );
}
