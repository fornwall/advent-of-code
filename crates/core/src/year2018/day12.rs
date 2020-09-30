use std::collections::{HashMap, HashSet, VecDeque};

struct Tunnel {
    current_gen: std::vec::Vec<bool>,
    next_gen: std::vec::Vec<bool>,
    offset: usize,
    evolutions: HashMap<(bool, bool, bool, bool, bool), bool>,
    used_steps: HashSet<(bool, bool, bool, bool, bool)>,
}

impl Tunnel {
    fn parse(input_string: &str, space_for_generations: usize) -> Result<Self, String> {
        let mut evolutions = HashMap::new();

        let mut lines = input_string.lines();
        let next_line = lines.next().ok_or("Invalid tunnel format")?;
        let initial_line: &str = &next_line["initial state: ".len()..];

        let max_growth = space_for_generations * 2;
        let state_length = initial_line.len() + 2 * max_growth;
        let mut current_gen = vec![false; state_length];
        let next_gen = vec![false; state_length];
        for (i, byte) in initial_line.bytes().enumerate() {
            current_gen[max_growth + i] = byte == b'#';
        }

        lines.next(); // Skip empty line
        for line in lines {
            let parts: Vec<&str> = line.split(" => ").collect();
            let from_bytes: Vec<u8> = parts[0].bytes().collect();
            let result = parts[1] == "#";
            let from = (
                from_bytes[0] == b'#',
                from_bytes[1] == b'#',
                from_bytes[2] == b'#',
                from_bytes[3] == b'#',
                from_bytes[4] == b'#',
            );
            evolutions.insert(from, result);
        }

        let capacity = evolutions.len();
        Ok(Self {
            current_gen,
            next_gen,
            offset: max_growth,
            evolutions,
            used_steps: HashSet::with_capacity(capacity),
        })
    }

    fn evolve(&mut self) {
        self.used_steps.clear();

        for i in 2..self.current_gen.len() - 2 {
            let current = (
                self.current_gen[i - 2],
                self.current_gen[i - 1],
                self.current_gen[i],
                self.current_gen[i + 1],
                self.current_gen[i + 2],
            );

            self.next_gen[i] = match self.evolutions.get(&current) {
                Some(&value) => {
                    if value {
                        self.used_steps.insert(current);
                    }
                    value
                }
                None => false,
            };
        }

        std::mem::swap(&mut self.next_gen, &mut self.current_gen);
    }

    fn score(&self) -> i64 {
        let mut sum = 0;
        for (index, &value) in self.current_gen.iter().enumerate() {
            if value {
                let index = index as i32 - self.offset as i32;
                sum += i64::from(index);
            }
        }
        sum
    }
}

pub fn part1(input_string: &str) -> Result<i64, String> {
    let mut tunnel = Tunnel::parse(input_string, 20)?;
    for _ in 0..20 {
        tunnel.evolve();
    }
    Ok(tunnel.score())
}

pub fn part2(input_string: &str) -> Result<i64, String> {
    let max_steps = 200;
    let mut tunnel = Tunnel::parse(input_string, max_steps)?;

    let mut score_diffs = VecDeque::with_capacity(5);
    let mut previous_score = -1;
    for generation in 1..=max_steps {
        tunnel.evolve();

        let score_diff = tunnel.score() - previous_score;
        previous_score = tunnel.score();
        score_diffs.push_back(score_diff);

        if score_diffs.len() > 5 {
            score_diffs.pop_front();
            if score_diffs.iter().filter(|&&e| e == score_diffs[0]).count() == 5 {
                let remaining_generations = 50_000_000_000_i64 - generation as i64;
                let increase_per_generation = score_diff;
                let final_score = tunnel.score() + remaining_generations * increase_per_generation;
                return Ok(final_score);
            }
        }
    }

    Err("No cycle found".to_string())
}

#[test]
fn tests_part1() {
    assert_eq!(
        Ok(325),
        part1(
            "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"
        )
    );
    assert_eq!(Ok(2140), part1(include_str!("day12_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(
        Ok(1_900_000_000_384),
        part2(include_str!("day12_input.txt"))
    );
}
