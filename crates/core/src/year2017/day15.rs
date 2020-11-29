use crate::input::Input;

struct Generator {
    value: u64,
    factor: u64,
    only_multiples_of: u64,
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.value = (self.value * self.factor) % 2147483647;
            if self.value % self.only_multiples_of == 0 {
                return Some(self.value);
            }
        }
    }
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let starting_values = input
        .text
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            let last_word = line
                .split(' ')
                .last()
                .ok_or_else(|| "Empty line".to_string())?;
            last_word.parse::<u64>().map_err(|parse_error| {
                format!(
                    "Line {}: Unable to parse starting value: {}",
                    line_index + 1,
                    parse_error.to_string()
                )
            })
        })
        .collect::<Result<Vec<u64>, _>>()?;

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
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day15_input.txt");
    test_part_one!(real_input => 597);
    test_part_two!(real_input => 303);
}
