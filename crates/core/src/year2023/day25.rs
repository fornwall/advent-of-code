use crate::common::array_deque::ArrayDeque;
use crate::common::id_assigner_copy::IdAssigner;
use crate::input::{on_error, Input};

const MAX_ENTRIES: usize = 2000;
const MAX_CONNECTIONS: usize = 10;
const INITIAL_ELEMENT_ID: u16 = 0;
const EMPTY: u16 = u16::MAX;

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut id_assigner = IdAssigner::<MAX_ENTRIES, &str>::new("");
    let mut entries = [Entry::default(); MAX_ENTRIES];

    for line in input.text.lines() {
        let (from, to) = line.split_once(": ").ok_or_else(on_error)?;
        let from = id_assigner.id_of(from)?;
        for to in to.split(' ') {
            let to = id_assigner.id_of(to)?;
            for (from, to) in [(from, to), (to, from)] {
                let entry = &mut entries[from as usize];
                entry.to[entry.num_entries as usize] = to;
                entry.num_entries += 1;
            }
        }
    }

    let mut furthest_away = None;

    for _ in 0..3 {
        let (mut path_end, previous) = bfs(&entries, furthest_away)?;
        furthest_away = Some(path_end);

        while path_end != 0 {
            let from = previous[path_end as usize];
            let entry = &mut entries[from as usize];
            for i in 0..entry.num_entries {
                if entry.to[i as usize] == path_end {
                    // Delete connection.
                    entry.to.swap(i as usize, entry.num_entries as usize);
                    entry.num_entries -= 1;
                    break;
                }
            }
            path_end = from;
        }
    }

    let (_, previous) = bfs(&entries, furthest_away)?;
    let one_group_size = previous.iter().filter(|&&p| p != EMPTY).count() as u64;
    let other_group_size = id_assigner.len() as u64 - one_group_size;
    Ok(one_group_size * other_group_size)
}

fn bfs(
    entries: &[Entry; MAX_ENTRIES],
    target: Option<u16>,
) -> Result<(u16, [u16; MAX_ENTRIES]), String> {
    let mut previous = [EMPTY; MAX_ENTRIES];
    let mut work_queue = ArrayDeque::<2000, u16>::new();
    work_queue.push_back(INITIAL_ELEMENT_ID)?;

    let mut last = 0;
    while let Some(current) = work_queue.pop_front() {
        last = current;
        if target.is_some_and(|t| t == current) {
            break;
        }
        let num_entries = entries[current as usize].num_entries;
        for &to in &entries[current as usize].to[0..(num_entries as usize)] {
            if previous[to as usize] == EMPTY {
                previous[to as usize] = current;
                work_queue.push_back(to)?;
            }
        }
    }

    Ok((last, previous))
}

#[derive(Copy, Clone, Default)]
struct Entry {
    to: [u16; MAX_CONNECTIONS],
    num_entries: u8,
}

#[test]
pub fn tests() {
    use crate::input::test_part_one_no_allocations;

    let real_input = include_str!("day25_input.txt");
    test_part_one_no_allocations!(real_input => 543_564);
}
