use crate::input::Input;
use std::collections::HashMap;

enum EntryType {
    BeginShift { guard_id: u32 },
    FallsAsleep,
    WakesUp,
}

struct LogEntry {
    minute: u32,
    entry: EntryType,
}

fn parse_input(input_string: &str) -> Result<Vec<LogEntry>, String> {
    let mut lines: Vec<&str> = input_string.lines().collect();
    lines.sort_unstable();

    lines
        .iter()
        .enumerate()
        .map(|(line_index, line)| {
            let error_message = || format!("Incorrect input at line {}", line_index + 1);
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 || parts[1].len() != 6 {
                return Err(error_message());
            }

            let minute = parts[1][3..5].parse().map_err(|_| error_message())?;

            let entry = match *parts.last().ok_or("Internal error: No last value")? {
                "shift" => EntryType::BeginShift {
                    guard_id: parts[3][1..].parse().map_err(|_| error_message())?,
                },
                "asleep" => EntryType::FallsAsleep,
                "up" => EntryType::WakesUp,
                _ => {
                    return Err(error_message());
                }
            };

            Ok(LogEntry { minute, entry })
        })
        .collect()
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let entries = parse_input(input.text)?;

    let mut sleepers = HashMap::new();
    let mut current_guard = 0;
    let mut start_minute = 0;

    for log_entry in entries.iter() {
        match log_entry.entry {
            EntryType::BeginShift { guard_id } => current_guard = guard_id,
            EntryType::FallsAsleep => start_minute = log_entry.minute,
            EntryType::WakesUp => {
                let sleep_record = sleepers.entry(current_guard).or_insert_with(|| vec![0; 61]);
                for minute in start_minute..log_entry.minute {
                    sleep_record[minute as usize] += 1;
                }
            }
        }
    }

    if input.is_part_one() {
        let (&most_sleepy_guard, sleep_record) = sleepers
            .iter()
            .max_by_key(|(_key, value)| value.iter().sum::<i32>())
            .ok_or("Internal error: No most sleep guard")?;
        let most_sleepy_minute = sleep_record
            .iter()
            .enumerate()
            .max_by_key(|(_minute, count)| *count)
            .ok_or("Internal error: No most sleepy minute")?
            .0 as u32;

        Ok(most_sleepy_guard * most_sleepy_minute)
    } else {
        let mut highest_sleep_count = -1;
        let mut sleepiest_guard_id = 0;
        let mut most_sleepy_minute = 0;
        for (&guard_id, sleep_record) in sleepers.iter() {
            let (sleepy_minute, &sleep_count) = sleep_record
                .iter()
                .enumerate()
                .max_by_key(|(_minute, count)| *count)
                .ok_or("No sleep record for guard")?;
            if sleep_count > highest_sleep_count {
                highest_sleep_count = sleep_count;
                sleepiest_guard_id = guard_id;
                most_sleepy_minute = sleepy_minute;
            }
        }
        Ok(sleepiest_guard_id * most_sleepy_minute as u32)
    }
}

#[test]
fn test() {
    use crate::{test_part_one, test_part_two};

    test_part_one!(
            "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"
    => 240
    );

    let input = include_str!("day04_input.txt");
    test_part_one!(input => 84834);

    test_part_two!(
            "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"
        =>4455
    );

    test_part_two!(input => 53427);
}
