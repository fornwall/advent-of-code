use crate::common::highest_values::HighestValues;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    if input.is_part_one() {
        solve_part::<1>(input)
    } else {
        solve_part::<3>(input)
    }
}

pub fn solve_part<const NUM: usize>(input: &Input) -> Result<u64, String> {
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
    let test_input = "1000\n\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    test_part_one_no_allocations!(test_input => 24_000);
    test_part_two_no_allocations!(test_input => 45_000);

    let test_input = "4294967296";
    test_part_one_no_allocations!(test_input => 0);
    test_part_two_no_allocations!(test_input => 0);

    let test_input = "4294967295\n\n1\n\n1";
    test_part_one_no_allocations!(test_input => 4_294_967_295);
    test_part_two_no_allocations!(test_input => 4_294_967_297);

    let real_input = include_str!("day01_input.txt");
    test_part_one_no_allocations!(real_input => 71_300);
    test_part_two_no_allocations!(real_input => 209_691);
}
