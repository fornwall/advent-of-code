use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u64, String> {
    const MAX_DIFF: u64 = 3;

    let mut joltages = std::iter::once(Ok(0))
        .chain(input.text.lines().enumerate().map(|(line_idx, line)| {
            line.parse::<u64>().map_err(|parse_error| {
                format!("Line {}: Invalid joltage - {}", line_idx + 1, parse_error)
            })
        }))
        .collect::<Result<Vec<u64>, _>>()?;

    joltages.sort_unstable();
    joltages.push(joltages[joltages.len() - 1] + MAX_DIFF);

    if input.is_part_one() {
        let diff_counts =
            joltages
                .windows(2)
                .try_fold([0; MAX_DIFF as usize + 1], |mut acc, window| {
                    let diff = window[1] - window[0];
                    if diff > MAX_DIFF {
                        Err(format!(
                            "Too big difference between adapters - cannot go from {} to {}",
                            window[0], window[1]
                        ))
                    } else {
                        acc[diff as usize] += 1;
                        Ok(acc)
                    }
                })?;
        Ok(diff_counts[1] * diff_counts[3])
    } else {
        let mut distinct_ways_counts = vec![0; joltages.len()];
        distinct_ways_counts[0] = 1;

        for (idx, joltage) in joltages.iter().enumerate().skip(1) {
            let mut back_step = 1;
            while idx >= back_step {
                let back_idx = idx - back_step;
                let joltage_diff = joltage - joltages[back_idx];
                if joltage_diff <= MAX_DIFF {
                    distinct_ways_counts[idx] += distinct_ways_counts[back_idx];
                } else {
                    break;
                }
                back_step += 1;
            }
        }

        Ok(distinct_ways_counts[distinct_ways_counts.len() - 1])
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
    test_part_one!(example => 35);
    test_part_two!(example => 8);

    let real_input = include_str!("day10_input.txt");
    test_part_one!(real_input => 2376);
    test_part_two!(real_input => 129586085429248);
}
