use crate::input::Input;

/// The highest `NUM` values out of the values provided to `on_value(new_value)`.
struct HighestValues<const NUM: usize> {
    values: [u64; NUM],
}

impl<const NUM: usize> HighestValues<NUM> {
    const fn new() -> Self {
        Self { values: [0; NUM] }
    }

    fn on_value(&mut self, new_value: u64) {
        self.values.sort_unstable();
        for existing_value in self.values.iter_mut() {
            if *existing_value < new_value {
                *existing_value = new_value;
                return;
            }
        }
    }

    fn sum(&self) -> u64 {
        self.values.iter().sum()
    }
}

impl<const NUM: usize> FromIterator<u64> for HighestValues<NUM> {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let mut highest = Self::new();
        for i in iter {
            highest.on_value(i);
        }
        highest
    }
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    if input.is_part_one() {
        solve_part::<1>(input)
    } else {
        solve_part::<3>(input)
    }
}

pub fn solve_part<const NUM: usize>(input: &mut Input) -> Result<u64, String> {
    Ok(input
        .text
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| u64::from(line.parse::<u32>().unwrap_or_default()))
                .sum()
        })
        .collect::<HighestValues<NUM>>()
        .sum())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "1000\n\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    test_part_one!(test_input => 24_000);
    test_part_two!(test_input => 45_000);

    let test_input = "4294967296";
    test_part_one!(test_input => 0);
    test_part_two!(test_input => 0);

    let test_input = "4294967295\n\n1\n\n1";
    test_part_one!(test_input => 4_294_967_295);
    test_part_two!(test_input => 4_294_967_297);

    let real_input = include_str!("day01_input.txt");
    test_part_one!(real_input => 71_300);
    test_part_two!(real_input => 209_691);
}
