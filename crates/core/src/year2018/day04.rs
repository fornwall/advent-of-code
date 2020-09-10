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

fn parse_input(input_string: &str) -> Vec<LogEntry> {
    let mut lines: Vec<&str> = input_string.lines().collect();
    lines.sort();

    lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let minute = parts[1][3..5].parse().unwrap();

            let entry = match *parts.last().unwrap() {
                "shift" => EntryType::BeginShift {
                    guard_id: parts[3][1..].parse().expect("Could not parse guard"),
                },
                "asleep" => EntryType::FallsAsleep,
                "up" => EntryType::WakesUp,
                _ => panic!("Invalid line"),
            };

            LogEntry { minute, entry }
        })
        .collect()
}

pub fn part1(input_string: &str) -> String {
    let entries = parse_input(input_string);

    let mut sleepers = HashMap::new();
    let mut current_guard = 0;
    let mut start_minute = 0;

    for log_entry in entries.iter() {
        match log_entry.entry {
            EntryType::BeginShift { guard_id } => current_guard = guard_id,
            EntryType::FallsAsleep => start_minute = log_entry.minute,
            EntryType::WakesUp => {
                let duration = log_entry.minute - start_minute;
                *sleepers.entry(current_guard).or_insert(0) += duration;
            }
        }
    }

    let most_sleepy_guard = *sleepers
        .iter()
        .max_by_key(|(_key, value)| *value)
        .unwrap()
        .0;

    let mut sleep_record = vec![0; 61];
    for log_entry in entries.iter() {
        match log_entry.entry {
            EntryType::BeginShift { guard_id } => current_guard = guard_id,
            EntryType::FallsAsleep => start_minute = log_entry.minute,
            EntryType::WakesUp => {
                if current_guard == most_sleepy_guard {
                    for minute in start_minute..log_entry.minute {
                        sleep_record[minute as usize] += 1;
                    }
                }
            }
        }
    }

    let most_sleepy_minute = sleep_record
        .iter()
        .enumerate()
        .max_by_key(|(_minute, count)| *count)
        .unwrap()
        .0 as u64;

    (u64::from(most_sleepy_guard) * most_sleepy_minute).to_string()
}

pub fn part2(input_string: &str) -> String {
    let entries = parse_input(input_string);

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

    let (&guard_id, most_sleepy_minute, _) = sleepers
        .iter()
        .map(|(guard_id, sleep_record)| {
            let (most_sleepy_minute, sleep_count) = sleep_record
                .iter()
                .enumerate()
                .max_by_key(|(_minute, count)| *count)
                .unwrap();
            (guard_id, most_sleepy_minute, sleep_count)
        })
        .max_by_key(|(_, _, sleep_count)| *sleep_count)
        .unwrap();

    (u64::from(guard_id) * most_sleepy_minute as u64).to_string()
}

#[test]
fn tests_part1() {
    assert_eq!(
        "240",
        part1(
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
        )
    );

    assert_eq!("84834", part1(include_str!("day04_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(
        "4455",
        part2(
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
        )
    );

    assert_eq!("53427", part2(include_str!("day04_input.txt")));
}
