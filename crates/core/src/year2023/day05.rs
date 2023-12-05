use crate::input::Input;

pub fn solve(input: &Input) -> Result<i64, String> {
    let mut lines = input.text.lines();
    let initial_line = &lines.next().unwrap()["seeds: ".len()..];
    let mut first_value = None;
    let mut mapped_intervals = initial_line.split(" ").filter_map(|n| {
        let n = n.parse::<i64>().unwrap();
        if input.is_part_two() {
            if let Some(start) = first_value {
                first_value = None;
                Some((start, start + n))
            } else {
                first_value = Some(n);
                None
            }
        } else {
            Some((n, n + 1))
        }
    }).collect::<Vec<_>>();

    let mut source_intervals = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        } else if line.ends_with("map:") {
            // Any source numbers that aren't mapped correspond to the same destination number:
            mapped_intervals.extend(&source_intervals);

            source_intervals.clear();
            std::mem::swap(&mut source_intervals, &mut mapped_intervals);
        } else {
            let mut parts = line.split(' ');
            let destination_range_start = parts.next().unwrap().parse::<i64>().unwrap();
            let source_range_start = parts.next().unwrap().parse::<i64>().unwrap();
            let range_len = parts.next().unwrap().parse::<i64>().unwrap();

            let source = (source_range_start, source_range_start + range_len);
            let dest_diff = destination_range_start - source_range_start;

            let mut new_source = Vec::new();
            for source_interval in source_intervals.into_iter() {
                let (before, inside, after) = intersect_intervals(source_interval, source);
                if let Some(before) = before {
                    new_source.push(before);
                }
                if let Some(after) = after {
                    new_source.push(after);
                }
                if let Some(inside) = inside {
                    mapped_intervals.push((inside.0 + dest_diff, inside.1 + dest_diff));
                } else {
                    new_source.push(source_interval);
                }
            }

            source_intervals = new_source;
        }
    }

    // Any source numbers that aren't mapped correspond to the same destination number:
    mapped_intervals.extend(&source_intervals);
    Ok(mapped_intervals.iter().map(|i| i.0).min().unwrap())
}


// -> (before, inside, after), for parts of source interval before, isnide and after destination itnerval
fn intersect_intervals(interval_a: (i64, i64), interval_b: (i64, i64)) -> (Option<(i64, i64)>, Option<(i64, i64)>, Option<(i64, i64)>) {
    if interval_b.0 > interval_a.1 || interval_a.0 > interval_b.1 {
        (None, None, None)
    } else {
        let intersection_start = std::cmp::max(interval_a.0, interval_b.0);
        let intersection_end = std::cmp::min(interval_a.1, interval_b.1);
        let inside = (intersection_start, intersection_end);

        let mut before = None;
        if interval_a.0 < intersection_start {
            before = Some((interval_a.0, intersection_start));
        }

        let mut after = None;
        if interval_a.1 > intersection_end {
            after = Some((intersection_end, interval_a.1));
        }

        (before, Some(inside), after)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    assert_eq!(intersect_intervals((1, 10), (20, 30)), (None, None, None));
    assert_eq!(intersect_intervals((1, 10), (1, 10)), (None, Some((1, 10)), None));
    assert_eq!(intersect_intervals((1, 10), (5, 10)), (Some((1, 5)), Some((5, 10)), None));
    assert_eq!(intersect_intervals((1, 10), (1, 5)), (None, Some((1, 5)), Some((5, 10))));
    assert_eq!(intersect_intervals((1, 10), (3, 6)), (Some((1, 3)), Some((3, 6)), Some((6, 10))));

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
    test_part_one_no_allocations!(real_input => 331445006);
    test_part_two_no_allocations!(real_input => 6472060);
}
