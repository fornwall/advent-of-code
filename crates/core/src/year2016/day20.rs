use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    let mut intervals = Vec::new();
    for line in input.text.lines() {
        let (from, to) = line
            .split_once('-')
            .and_then(|(from, to)| Some((from.parse::<u32>().ok()?, to.parse::<u32>().ok()?)))
            .ok_or("Invalid input")?;
        if from > to {
            return Err("Invalid interval with from > to".into());
        }
        intervals.push((from, to));
    }

    intervals.sort_unstable();

    if input.is_part_one() {
        Ok(intervals.iter().fold(0, |lowest_allowed, &(from, to)| {
            if from > lowest_allowed {
                lowest_allowed
            } else {
                std::cmp::max(lowest_allowed, to + 1)
            }
        }))
    } else {
        let mut in_gaps = intervals[0].0;
        let mut highest_blocked = intervals[0].1;
        for &(from, to) in intervals.iter() {
            if highest_blocked != u32::MAX && from > highest_blocked + 1 {
                in_gaps += from - highest_blocked - 1;
            }
            highest_blocked = std::cmp::max(highest_blocked, to);
        }
        Ok(u32::MAX - highest_blocked + in_gaps)
    }
}

#[test]
pub fn tests() {
    let real_input = include_str!("day20_input.txt");
    test_part_one!(real_input => 17_348_574);
    test_part_two!(real_input => 104);
}
