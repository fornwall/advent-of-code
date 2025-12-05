use crate::{
    common::array_stack::ArrayStack,
    input::{Input, on_error},
};

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAX_RANGES: usize = 256;
    let mut ranges = ArrayStack::<MAX_RANGES, RangeInclusiveCopy>::new();

    let (ranges_text, ids_text) = input.text.split_once("\n\n").ok_or_else(on_error)?;

    for line in ranges_text.lines() {
        let (start_str, end_str) = line.split_once('-').ok_or_else(on_error)?;
        let start = start_str.parse::<u64>().map_err(|e| e.to_string())?;
        let end = end_str.parse::<u64>().map_err(|e| e.to_string())?;
        ranges.push(RangeInclusiveCopy { start, end })?;
    }

    if ranges.len() == 0 {
        return Err("No ranges provided".to_string());
    }

    ranges.slice_mut().sort();

    let mut joined_ranges = ArrayStack::<MAX_RANGES, RangeInclusiveCopy>::new();
    let mut last_range = ranges.slice()[0];
    for new_range in ranges.slice().iter().skip(1) {
        last_range = if let Some(joined) = last_range.join_if_overlaps(new_range) {
            joined
        } else {
            joined_ranges.push(last_range)?;
            *new_range
        }
    }
    joined_ranges.push(last_range)?;

    if input.is_part_one() {
        ids_text.lines().try_fold(0, |acc, line| {
            let id = line.parse::<u64>().map_err(|e| e.to_string())?;
            Ok(acc + u64::from(joined_ranges.slice().iter().any(|range| range.contains(id))))
        })
    } else {
        Ok(joined_ranges
            .slice()
            .iter()
            .map(|r| r.end - r.start + 1)
            .sum::<u64>())
    }
}

// The std::ops::RangeInclusive does not implement Copy, so we make our own version.
// https://stackoverflow.com/questions/43416914/why-doesnt-opsranget-implement-copy-even-if-t-is-copy
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
struct RangeInclusiveCopy {
    start: u64,
    end: u64,
}

impl RangeInclusiveCopy {
    const fn contains(&self, value: u64) -> bool {
        self.start <= value && value <= self.end
    }
    const fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
    fn join_if_overlaps(&self, other: &Self) -> Option<Self> {
        self.overlaps(other).then(|| Self {
            start: std::cmp::min(self.start, other.start),
            end: std::cmp::max(self.end, other.end),
        })
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    test_part_one_no_allocations!(test_input => 3);
    test_part_two_no_allocations!(test_input => 14);

    let real_input = include_str!("day05_input.txt");
    test_part_one_no_allocations!(real_input => 707);
    test_part_two_no_allocations!(real_input => 361_615_643_045_059);
}
