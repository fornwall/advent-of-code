#[derive(Debug, Clone)]
struct Moons {
    positions: [[i32; 3]; 4],
    velocities: [[i32; 3]; 4],
}

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

pub fn lcd(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn lcd3(a: usize, b: usize, c: usize) -> usize {
    lcd(a, lcd(b, c))
}

impl Moons {
    fn parse(input: &str) -> Moons {
        let mut positions = [[0; 3]; 4];
        input.lines().enumerate().for_each(|(i, line)| {
            let parts: Vec<&str> = line
                .split(|c| c == '=' || c == ' ' || c == '>' || c == ',')
                .collect();

            positions[i][0] = parts[1].trim().parse::<i32>().unwrap();
            positions[i][1] = parts[4].trim().parse::<i32>().unwrap();
            positions[i][2] = parts[7].trim().parse::<i32>().unwrap();
        });

        Moons {
            positions,
            velocities: [[0; 3]; 4],
        }
    }

    fn total_energy(&self) -> u64 {
        let mut total_energy = 0 as u64;
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

fn signum(value: i32) -> i32 {
    match value {
        _ if value < 0 => -1,
        _ if value == 0 => 0,
        _ => 1,
    }
}

pub fn part1_nth(input_string: &str, n: usize) -> String {
    let mut moons = Moons::parse(input_string);
    for _ in 0..n {
        moons.step();
    }
    moons.total_energy().to_string()
}

pub fn part1(input_string: &str) -> String {
    part1_nth(input_string, 1000)
}

pub fn part2(input_string: &str) -> String {
    let mut moons = Moons::parse(input_string);
    let initial_moons = moons.clone();
    let mut cycles: [Option<usize>; 3] = [None; 3];

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

    lcd3(cycles[0].unwrap(), cycles[1].unwrap(), cycles[2].unwrap()).to_string()
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
        "179"
    );

    assert_eq!(part1(include_str!("day12_input.txt")), "6220");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day12_input.txt")), "548525804273976");
}
