use crate::input::Input;
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

struct ParsedInput {
    step_map: HashMap<char, Step>,
    remaining_dependencies: HashMap<char, HashSet<char>>,
}

fn parse_input(input_string: &str) -> Result<ParsedInput, String> {
    let mut step_map = HashMap::new();
    let mut remaining_dependencies: HashMap<char, HashSet<char>> = HashMap::new();

    for (line_index, line) in input_string.lines().enumerate() {
        let line_number = line_index + 1;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 10 {
            return Err(format!("Invalid line: {line_number}"));
        }
        let step_name = parts[7]
            .chars()
            .next()
            .ok_or(format!("Invalid line: {line_number}"))?;
        let depends_on = parts[1]
            .chars()
            .next()
            .ok_or(format!("Invalid line: {line_number}"))?;

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

    Ok(ParsedInput {
        step_map,
        remaining_dependencies,
    })
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

pub fn solve(input: &Input) -> Result<String, String> {
    const WORKERS: usize = 5;
    const STEP_DURATION_BASE: i32 = 60;

    let ParsedInput {
        step_map,
        mut remaining_dependencies,
    } = parse_input(input.text)?;

    if input.is_part_one() {
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
                    let v = remaining_dependencies
                        .get_mut(needed_by_name)
                        .ok_or("Dependency not found")?;
                    v.remove(&step.name);
                    if v.is_empty() {
                        queue.push(&step_map[needed_by_name]);
                    };
                }
            }
        }

        Ok(result)
    } else {
        let mut work_queue: BinaryHeap<Work> = BinaryHeap::new();
        let mut step_queue: BinaryHeap<&Step> = BinaryHeap::new();
        step_map
            .values()
            .filter(|step| step.dependencies.is_empty())
            .for_each(|step| {
                step_queue.push(step);
            });

        while work_queue.len() < WORKERS && !step_queue.is_empty() {
            let step = step_queue.pop().ok_or("No step to pop")?;
            let done_at_time = STEP_DURATION_BASE + (1 + step.name as i32 - 'A' as i32);
            work_queue.push(Work::new(step.name, done_at_time));
        }

        let mut result = String::new();

        let mut latest_work_done_at = 0;
        while let Some(work_done) = work_queue.pop() {
            latest_work_done_at = work_done.done_at_second;

            result.push(work_done.name);

            let step = &step_map[&work_done.name];

            for needed_by_name in step.needed_by.iter().rev() {
                let v = remaining_dependencies
                    .get_mut(needed_by_name)
                    .ok_or("Dependency not found")?;
                v.remove(&step.name);
                if v.is_empty() {
                    step_queue.push(&step_map[needed_by_name]);
                };
            }

            while work_queue.len() < WORKERS && !step_queue.is_empty() {
                let next_step = step_queue.pop().ok_or("No step to pop")?;
                let next_step_done_at = work_done.done_at_second
                    + STEP_DURATION_BASE
                    + (1 + next_step.name as i32 - 'A' as i32);
                work_queue.push(Work::new(next_step.name, next_step_done_at));
            }
        }

        Ok(latest_work_done_at.to_string())
    }
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!(
            "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
    => "CABDFE".to_string()
    );

    test_part_one!(
            "Step B must be finished before step A can begin.
    Step C must be finished before step A can begin."
    => "BCA".to_string()
    );

    test_part_one!(
            "Step C must be finished before step A can begin.
    Step B must be finished before step A can begin."
        => "BCA".to_string()
    );

    let input = include_str!("day07_input.txt");
    test_part_one!(
        input => "OUGLTKDJVBRMIXSACWYPEQNHZF".to_string()
    );
    test_part_two!(input => "929".to_string());
}
