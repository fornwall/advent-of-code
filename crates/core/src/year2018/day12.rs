use crate::input::Input;

struct Tunnel {
    current_gen: std::vec::Vec<bool>,
    next_gen: std::vec::Vec<bool>,
    offset: usize,
    evolutions: [bool; 32],
}

impl Tunnel {
    fn parse(input_string: &str, space_for_generations: usize) -> Result<Self, String> {
        let mut evolutions = [false; 32];

        let mut lines = input_string.lines();
        let next_line = lines.next().ok_or("Invalid tunnel format")?;
        let prefix_length = "initial state: ".len();

        let initial_line: &str = next_line.get(prefix_length..).ok_or("Invalid input")?;

        let max_growth = space_for_generations * 2;
        let state_length = initial_line.len() + 2 * max_growth;
        let mut current_gen = vec![false; state_length];
        let next_gen = vec![false; state_length];
        for (i, byte) in initial_line.bytes().enumerate() {
            current_gen[max_growth + i] = byte == b'#';
        }

        lines.next(); // Skip empty line
        for line in lines {
            let (part1, part2) = line
                .split_once(" => ")
                .ok_or_else(|| "Invalid input".to_string())?;
            if part2 == "#" {
                let from_bytes: Vec<u8> = part1.bytes().collect();
                if from_bytes.len() != 5 {
                    return Err("Invalid input".to_string());
                }
                let from = ((from_bytes[0] == b'#') as usize)
                    + (((from_bytes[1] == b'#') as usize) << 1)
                    + (((from_bytes[2] == b'#') as usize) << 2)
                    + (((from_bytes[3] == b'#') as usize) << 3)
                    + (((from_bytes[4] == b'#') as usize) << 4);
                evolutions[from] = true;
            }
        }

        Ok(Self {
            current_gen,
            next_gen,
            offset: max_growth,
            evolutions,
        })
    }

    fn evolve(&mut self) {
        for i in 2..self.current_gen.len() - 2 {
            let current = (self.current_gen[i - 2] as usize)
                + ((self.current_gen[i - 1] as usize) << 1)
                + ((self.current_gen[i] as usize) << 2)
                + ((self.current_gen[i + 1] as usize) << 3)
                + ((self.current_gen[i + 2] as usize) << 4);
            self.next_gen[i] = self.evolutions[current];
        }

        std::mem::swap(&mut self.next_gen, &mut self.current_gen);
    }

    fn is_repeating(&self) -> bool {
        self.current_gen
            .iter()
            .skip_while(|&&populated| !populated)
            .zip(self.next_gen.iter().skip_while(|&&populated| !populated))
            .all(|(a, b)| a == b)
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

pub fn solve(input: &mut Input) -> Result<i64, String> {
    let max_steps = input.part_values(20, 1000);

    let mut tunnel = Tunnel::parse(input.text, max_steps)?;

    if input.is_part_one() {
        for _ in 0..20 {
            tunnel.evolve();
        }
        return Ok(tunnel.score());
    }

    let mut previous_score = -1;
    for generation in 1..=max_steps {
        tunnel.evolve();

        let score_diff = tunnel.score() - previous_score;
        previous_score = tunnel.score();

        if tunnel.is_repeating() {
            let remaining_generations = 50_000_000_000_i64 - generation as i64;
            let final_score = tunnel.score() + remaining_generations * score_diff;
            return Ok(final_score);
        }
    }

    Err("No cycle found".to_string())
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!(
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
        => 325
    );

    let input = include_str!("day12_input.txt");
    test_part_one!(input => 2140);
    test_part_two!(
        input => 1_900_000_000_384
    );
}
