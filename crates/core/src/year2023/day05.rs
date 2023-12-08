use crate::common::array_stack::ArrayStack;
use crate::input::{on_error, Input};

type Interval = (i64, i64);

pub fn solve(input: &Input) -> Result<i64, String> {
    const MAX_INTERVALS: usize = 256;

    let mut mapped_intervals = ArrayStack::<MAX_INTERVALS, Interval>::new();
    let mut source_intervals = ArrayStack::<MAX_INTERVALS, Interval>::new();
    let mut scratch_intervals = ArrayStack::<MAX_INTERVALS, Interval>::new();

    let mut lines = input.text.lines();
    let initial_line = &lines.next().ok_or_else(on_error)?["seeds: ".len()..];
    let mut first_value = None;
    for n in initial_line.split(' ') {
        let n = n.parse::<i64>().map_err(|_| on_error())?;
        if input.is_part_two() {
            if let Some(start) = first_value {
                mapped_intervals.push((start, start + n))?;
                first_value = None;
            } else {
                first_value = Some(n);
            }
        } else {
            mapped_intervals.push((n, n + 1))?;
        }
    }

    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.ends_with("map:") {
            // Any source numbers that aren't mapped correspond to the same destination number:
            for source_interval in source_intervals.slice() {
                mapped_intervals.push(*source_interval)?;
            }

            std::mem::swap(&mut source_intervals, &mut mapped_intervals);
            mapped_intervals.clear();
        } else {
            let mut parts = line.split(' ');
            let destination_range_start = parse_num(parts.next())?;
            let source_range_start = parse_num(parts.next())?;
            let range_len = parse_num(parts.next())?;

            let source = (source_range_start, source_range_start + range_len);
            let dest_diff = destination_range_start - source_range_start;

            for source_interval in source_intervals.slice() {
                let [before, inside, after] = intersect_intervals(*source_interval, source);
                if let Some(inside) = inside {
                    mapped_intervals.push((inside.0 + dest_diff, inside.1 + dest_diff))?;

                    if let Some(before) = before {
                        scratch_intervals.push(before)?;
                    }
                    if let Some(after) = after {
                        scratch_intervals.push(after)?;
                    }
                } else {
                    scratch_intervals.push(*source_interval)?;
                }
            }

            std::mem::swap(&mut source_intervals, &mut scratch_intervals);
            scratch_intervals.clear();
        }
    }

    Ok(mapped_intervals
        .slice()
        .iter()
        .chain(source_intervals.slice().iter())
        .map(|i| i.0)
        .min()
        .unwrap_or_default())
}

fn parse_num(part: Option<&str>) -> Result<i64, String> {
    part.ok_or_else(on_error)?
        .parse::<i64>()
        .map_err(|_| on_error())
}

fn intersect_intervals(interval_a: Interval, interval_b: Interval) -> [Option<Interval>; 3] {
    if interval_b.0 > interval_a.1 || interval_a.0 > interval_b.1 {
        [None, None, None]
    } else {
        let intersection_start = std::cmp::max(interval_a.0, interval_b.0);
        let intersection_end = std::cmp::min(interval_a.1, interval_b.1);
        let inside = (intersection_start, intersection_end);

        let before = if interval_a.0 < intersection_start {
            Some((interval_a.0, intersection_start))
        } else {
            None
        };

        let after = if interval_a.1 > intersection_end {
            Some((intersection_end, interval_a.1))
        } else {
            None
        };

        [before, Some(inside), after]
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    assert_eq!(intersect_intervals((1, 10), (20, 30)), [None, None, None]);
    assert_eq!(
        intersect_intervals((1, 10), (1, 10)),
        [None, Some((1, 10)), None]
    );
    assert_eq!(
        intersect_intervals((1, 10), (5, 10)),
        [Some((1, 5)), Some((5, 10)), None]
    );
    assert_eq!(
        intersect_intervals((1, 10), (1, 5)),
        [None, Some((1, 5)), Some((5, 10))]
    );
    assert_eq!(
        intersect_intervals((1, 10), (3, 6)),
        [Some((1, 3)), Some((3, 6)), Some((6, 10))]
    );

    let test_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    test_part_one_no_allocations!(test_input => 35);
    test_part_two_no_allocations!(test_input => 46);

    let real_input = include_str!("day05_input.txt");
    test_part_one_no_allocations!(real_input => 331_445_006);
    test_part_two_no_allocations!(real_input => 6_472_060);
}
