use crate::input::Input;

type JoltageAmount = u64;

pub fn solve(input: &Input) -> Result<JoltageAmount, String> {
    // "Any given adapter can take an input 1, 2, or 3 jolts lower than
    // its rating and still produce its rated output joltage":
    const MAX_DIFF: JoltageAmount = 3;

    let mut joltages = std::iter::once(Ok(0))
        .chain(input.text.lines().enumerate().map(|(line_idx, line)| {
            line.parse::<JoltageAmount>().map_err(|parse_error| {
                format!("Line {}: Invalid joltage - {}", line_idx + 1, parse_error)
            })
        }))
        .collect::<Result<Vec<_>, _>>()?;

    joltages.sort_unstable();

    // "In addition, your device has a built-in joltage adapter rated
    // for 3 jolts higher than the highest-rated adapter in your bag":
    joltages.push(joltages[joltages.len() - 1] + MAX_DIFF);

    if input.is_part_one() {
        joltages
            .windows(2)
            .try_fold([0; MAX_DIFF as usize + 1], |mut diff_count, window| {
                let diff = window[1] - window[0];
                if diff > MAX_DIFF {
                    Err(format!(
                        "Too big difference between adapters - cannot go from {} to {}",
                        window[0], window[1]
                    ))
                } else {
                    diff_count[diff as usize] += 1;
                    Ok(diff_count)
                }
            })
            .map(|diff_counts| diff_counts[1] * diff_counts[3])
    } else {
        let mut distinct_ways_counts = vec![0; joltages.len()];
        distinct_ways_counts[0] = 1;

        for (idx, joltage) in joltages.iter().enumerate() {
            let this_distinct_count = distinct_ways_counts[idx];

            joltages[(idx + 1)..]
                .iter()
                .take_while(|&higher_joltage| higher_joltage - joltage <= MAX_DIFF)
                .enumerate()
                .for_each(|(offset, _)| {
                    distinct_ways_counts[idx + offset + 1] += this_distinct_count;
                });
        }

        Ok(distinct_ways_counts[distinct_ways_counts.len() - 1])
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};

    let example = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
    test_part_one!(example => 35);
    test_part_two!(example => 8);

    test_part_one_error!(" " => "Line 1: Invalid joltage - invalid digit found in string");
    test_part_one_error!("100" => "Too big difference between adapters - cannot go from 0 to 100");
    test_part_one!("1" => 1);
    test_part_two!("1" => 1);

    let real_input = include_str!("day10_input.txt");
    test_part_one!(real_input => 2376);
    test_part_two!(real_input => 129_586_085_429_248);
}
