use crate::input::Input;

#[derive(Copy, Clone)]
struct Generator {
    value: u64,
    factor: u64,
    only_multiples_of: u64,
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // https://www.reddit.com/r/adventofcode/comments/7jyz5x/2017_day_15_opportunities_for_optimization/drasfzr?utm_source=share&utm_medium=web2x&context=3
            // self.value = (self.value * self.factor) % 2147483647;
            let prod = self.value * self.factor;
            let g = (prod & 0x7fff_ffff) + (prod >> 31);
            self.value = if g >> 31 == 0 { g } else { g - 0x7fff_ffff };

            // Speedier version of modulo here:
            // if self.value % self.only_multiples_of == 0 {
            if self.value & (self.only_multiples_of - 1) == 0 {
                return Some(self.value);
            }
        }
    }
}

pub fn solve(input: &Input) -> Result<u32, String> {
    let starting_values = input
        .text
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            let last_word = line
                .split(' ')
                .next_back()
                .ok_or_else(|| "Empty line".to_string())?;
            last_word.parse::<u64>().map_err(|parse_error| {
                format!(
                    "Line {}: Unable to parse starting value: {}",
                    line_index + 1,
                    parse_error
                )
            })
        })
        .collect::<Result<Vec<u64>, _>>()?;

    if starting_values.len() != 2 {
        return Err("Invalid input - should be 2 lines".to_string());
    }

    let first_generator = Generator {
        value: starting_values[0],
        factor: 16807,
        only_multiples_of: input.part_values(1, 4),
    };

    let second_generator = Generator {
        value: starting_values[1],
        factor: 48271,
        only_multiples_of: input.part_values(1, 8),
    };

    let matches = first_generator
        .zip(second_generator)
        .take(input.part_values(40_000_000, 5_000_000))
        .filter(|(a, b)| *a as u16 == *b as u16)
        .count();
    Ok(matches as u32)
}

#[test]
fn tests() {
    let real_input = include_str!("day15_input.txt");
    test_part_one!(real_input => 597);
    test_part_two!(real_input => 303);
}
