use crate::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let target_presents = input
        .text
        .parse::<u32>()
        .map_err(|e| format!("Could not parse house number: {}", e))?;

    let present_multiplier = input.part_values(10, 11);
    let max_visits = input.part_values(usize::MAX, 50);

    let highest_house_number = target_presents / present_multiplier;
    let mut presents = vec![0; highest_house_number as usize];

    for elf_number in 1..highest_house_number {
        for house_number in (elf_number..highest_house_number)
            .step_by(elf_number as usize)
            .take(max_visits)
        {
            presents[house_number as usize] += elf_number * present_multiplier;
        }
    }

    presents
        .iter()
        .enumerate()
        .find(|&(_index, &current)| current >= target_presents)
        .map(|(index, _current)| index as u32)
        .ok_or_else(|| "No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day20_input.txt");
    test_part_one!(real_input => 786_240);
    test_part_two!(real_input => 831_600);
}
