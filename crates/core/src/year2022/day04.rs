use crate::input::Input;

fn parse_intervals(line: &str) -> Option<((u8, u8), (u8, u8))> {
    fn parse_range(range: &str) -> Option<(u8, u8)> {
        let (s1, s2) = range.split_once('-')?;
        Some((s1.parse().ok()?, s2.parse().ok()?))
    }

    let (a, b) = line.split_once(',')?;
    Some((parse_range(a)?, parse_range(b)?))
}

const fn contains(a: (u8, u8), b: (u8, u8)) -> bool {
    b.0 >= a.0 && b.1 <= a.1 || a.0 >= b.0 && a.1 <= b.1
}

const fn overlaps(a: (u8, u8), b: (u8, u8)) -> bool {
    b.0 <= a.1 && b.1 >= a.0
}

pub fn solve(input: &mut Input) -> Result<usize, String> {
    let condition: fn((u8, u8), (u8, u8)) -> bool = input.part_values(contains, overlaps);
    let mut count = 0;

    for (line_idx, line) in input.text.lines().enumerate() {
        let intervals = parse_intervals(line)
            .ok_or_else(|| format!("Line {}: Invalid input - expected 'u8-u8,u8-u8'", line_idx))?;
        count += usize::from(condition(intervals.0, intervals.1));
    }
    Ok(count)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};

    let test_input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    test_part_one!(test_input => 2);
    test_part_two!(test_input => 4);

    let real_input = include_str!("day04_input.txt");
    test_part_one!(real_input => 569);
    test_part_two!(real_input => 936);

    for input in ["1-2,3-4\nfoo", "1-2,3-4\n300-400,1-2", "1-2,3-4\n-1-2,3-4"] {
        test_part_one_error!(input => "Line 1: Invalid input - expected 'u8-u8,u8-u8'");
    }
}
