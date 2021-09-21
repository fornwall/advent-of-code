use crate::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut wrapping_paper = 0;
    let mut ribbon = 0;

    for (line_idx, line) in input.text.lines().enumerate() {
        let on_error = || format!("Line {}: Invalid input", line_idx + 1);
        let mut parts = line.split('x');
        let length = parts
            .next()
            .ok_or_else(on_error)?
            .parse::<u32>()
            .map_err(|_| on_error())?;
        let width = parts
            .next()
            .ok_or_else(on_error)?
            .parse::<u32>()
            .map_err(|_| on_error())?;
        let height = parts
            .next()
            .ok_or_else(on_error)?
            .parse::<u32>()
            .map_err(|_| on_error())?;

        wrapping_paper += 2 * (length * width + width * height + height * length)
            + std::cmp::min(
                length * width,
                std::cmp::min(width * height, height * length),
            );

        let mut dimensions = [length, width, height];
        dimensions.sort_unstable();
        ribbon += 2 * dimensions[0] + 2 * dimensions[1] + length * width * height;
    }

    Ok(input.part_values(wrapping_paper, ribbon))
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day02_input.txt");
    test_part_one!(real_input => 1_606_483);
    test_part_two!(real_input => 3_842_356);
}
