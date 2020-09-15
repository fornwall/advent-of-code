use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};

#[derive(Eq)]
struct Step {
    name: char,
    dependencies: HashSet<char>,
    needed_by: BTreeSet<char>,
}

impl Step {
    fn new(name: char) -> Self {
        Self {
            name,
            dependencies: HashSet::new(),
            needed_by: BTreeSet::new(),
        }
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.name.cmp(&self.name)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn part1(input_string: &str) -> Result<String, String> {
    let mut step_map = HashMap::new();
    let mut remaining_dependencies: HashMap<char, HashSet<char>> = HashMap::new();

    for line in input_string.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let step_name = parts[7].chars().next().unwrap();
        let depends_on = parts[1].chars().next().unwrap();

        let step = step_map
            .entry(step_name)
            .or_insert_with(|| Step::new(step_name));
        step.dependencies.insert(depends_on);
        remaining_dependencies
            .entry(step_name)
            .or_insert_with(HashSet::new)
            .insert(depends_on);

        step_map
            .entry(depends_on)
            .or_insert_with(|| Step::new(depends_on))
            .needed_by
            .insert(step_name);
    }

    // Topological sorting:
    let mut queue: BinaryHeap<&Step> = BinaryHeap::new();
    step_map
        .values()
        .filter(|step| step.dependencies.is_empty())
        .for_each(|step| {
            queue.push(step);
        });

    let mut visited: HashSet<char> = HashSet::new();
    let mut result = String::new();

    while let Some(step) = queue.pop() {
        if visited.insert(step.name) {
            result.push(step.name);

            for needed_by_name in step.needed_by.iter().rev() {
                let v = remaining_dependencies.get_mut(needed_by_name).unwrap();
                v.remove(&step.name);
                if v.is_empty() {
                    queue.push(&step_map[needed_by_name]);
                };
            }
        }
    }

    Ok(result)
}

#[derive(Eq)]
struct Work {
    name: char,
    done_at_second: i32,
}

impl Work {
    const fn new(name: char, done_at_second: i32) -> Self {
        Self {
            name,
            done_at_second,
        }
    }
}

impl Ord for Work {
    fn cmp(&self, other: &Self) -> Ordering {
        other.done_at_second.cmp(&self.done_at_second)
    }
}

impl PartialOrd for Work {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Work {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn part2_param(input_string: &str, workers: usize, step_duration_base: i32) -> i32 {
    let mut step_map = HashMap::new();
    let mut remaining_dependencies: HashMap<char, HashSet<char>> = HashMap::new();

    for line in input_string.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let step_name = parts[7].chars().next().unwrap();
        let depends_on = parts[1].chars().next().unwrap();

        let step = step_map
            .entry(step_name)
            .or_insert_with(|| Step::new(step_name));
        step.dependencies.insert(depends_on);
        remaining_dependencies
            .entry(step_name)
            .or_insert_with(HashSet::new)
            .insert(depends_on);

        step_map
            .entry(depends_on)
            .or_insert_with(|| Step::new(depends_on))
            .needed_by
            .insert(step_name);
    }

    let mut work_queue: BinaryHeap<Work> = BinaryHeap::new();
    let mut step_queue: BinaryHeap<&Step> = BinaryHeap::new();
    step_map
        .values()
        .filter(|step| step.dependencies.is_empty())
        .for_each(|step| {
            step_queue.push(step);
        });

    while work_queue.len() < workers && !step_queue.is_empty() {
        let step = step_queue.pop().unwrap();
        let done_at_time = step_duration_base + (1 + step.name as i32 - 'A' as i32);
        work_queue.push(Work::new(step.name, done_at_time));
    }

    let mut visited: HashSet<char> = HashSet::new();
    let mut result = String::new();

    let mut latest_work_done_at = 0;
    while let Some(work_done) = work_queue.pop() {
        // println!("Step {} done at time {}", work_done.name, work_done.done_at_second);
        latest_work_done_at = work_done.done_at_second;

        result.push(work_done.name);
        visited.insert(work_done.name);

        let step = &step_map[&work_done.name];

        for needed_by_name in step.needed_by.iter().rev() {
            let v = remaining_dependencies.get_mut(needed_by_name).unwrap();
            v.remove(&step.name);
            if v.is_empty() {
                step_queue.push(&step_map[needed_by_name]);
            };
        }

        while work_queue.len() < workers && !step_queue.is_empty() {
            let next_step = step_queue.pop().unwrap();
            let next_step_done_at = work_done.done_at_second
                + step_duration_base
                + (1 + next_step.name as i32 - 'A' as i32);
            // println!("Starting {} to be done at {}", next_step.name, next_step_done_at);
            work_queue.push(Work::new(next_step.name, next_step_done_at));
        }
    }

    latest_work_done_at
}

pub fn part2(input_string: &str) -> Result<i32, String> {
    Ok(part2_param(input_string, 5, 60))
}

#[test]
fn tests_part1() {
    assert_eq!(
        "CABDFE",
        part1(
            "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
        )
        .unwrap()
    );

    assert_eq!(
        "BCA",
        part1(
            "Step B must be finished before step A can begin.
Step C must be finished before step A can begin."
        )
        .unwrap()
    );

    assert_eq!(
        "BCA",
        part1(
            "Step C must be finished before step A can begin.
Step B must be finished before step A can begin."
        )
        .unwrap()
    );

    assert_eq!(
        "OUGLTKDJVBRMIXSACWYPEQNHZF",
        part1(include_str!("day07_input.txt")).unwrap()
    );
}

#[test]
fn tests_part2() {
    assert_eq!(
        15,
        part2_param(
            "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.",
            2,
            0
        )
    );

    assert_eq!(Ok(929), part2(include_str!("day07_input.txt")));
}
