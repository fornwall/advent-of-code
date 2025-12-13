use crate::common::tuple_window_iterator::TupleWindowIteratorExt;
use std::collections::HashMap;

use crate::common::id_assigner;
use crate::input::Input;

const MAX_SIZE: usize = 1024;

type Graph = [BitSet; MAX_SIZE];
type Cache = HashMap<(u16, u16), u64>;

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut id_assigner = id_assigner::IdAssigner::<MAX_SIZE, str>::new("");
    let mut graph: Graph = std::array::from_fn(|_| BitSet::new());
    let mut start_position = u16::MAX;
    let start_name = input.part_values("you", "svr");
    let mut out_position = u16::MAX;

    // For part 2:
    let mut dac_position = u16::MAX;
    let mut fft_position = u16::MAX;

    for line in input.text.lines() {
        let mut parts = line.split(':');
        let node_name = parts
            .next()
            .ok_or_else(|| format!("Invalid line, missing node name: {}", line))?
            .trim();
        let node_id = id_assigner.id_of(node_name)?;
        if node_name == start_name {
            start_position = node_id;
        } else if node_name == "dac" {
            dac_position = node_id;
        } else if node_name == "fft" {
            fft_position = node_id;
        }
        let neighbors_part = parts
            .next()
            .ok_or_else(|| format!("Invalid line, missing neighbors: {}", line))?
            .trim();
        for neighbor_name in neighbors_part.split_whitespace() {
            let neighbor_id = id_assigner.id_of(neighbor_name)?;
            if neighbor_name == "out" {
                out_position = neighbor_id;
            }
            graph[node_id as usize].insert(neighbor_id as usize);
        }
    }
    if start_position == u16::MAX {
        return Err(format!("Missing '{start_name}'"));
    } else if out_position == u16::MAX {
        return Err("Missing 'out'".to_string());
    } else if input.is_part_two() && dac_position == u16::MAX {
        return Err("Missing 'dac'".to_string());
    } else if input.is_part_two() && fft_position == u16::MAX {
        return Err("Missing 'fft'".to_string());
    }
    Ok(if input.is_part_one() {
        num_ways_to_visit(&graph, start_position, out_position, &mut Cache::new())
    } else {
        [start_position, fft_position, dac_position, out_position]
            .iter()
            .tuple_windows()
            .map(|(&a, &b)| num_ways_to_visit(&graph, a, b, &mut Cache::new()))
            .product::<u64>()
            + [start_position, dac_position, fft_position, out_position]
                .iter()
                .tuple_windows()
                .map(|(&a, &b)| num_ways_to_visit(&graph, a, b, &mut Cache::new()))
                .product::<u64>()
    })
}

fn num_ways_to_visit(graph: &Graph, start: u16, end: u16, cache: &mut Cache) -> u64 {
    if let Some(&value) = cache.get(&(start, end)) {
        return value;
    } else if start == end {
        return 1;
    }
    let total_ways = (0..MAX_SIZE)
        .filter(|&n| graph[usize::from(start)].contains(n))
        .map(|n| num_ways_to_visit(graph, n as u16, end, cache))
        .sum();
    cache.insert((start, end), total_ways);
    total_ways
}

struct BitSet {
    bits: [u64; MAX_SIZE / 64],
}

impl BitSet {
    const fn new() -> Self {
        Self {
            bits: [0; MAX_SIZE / 64],
        }
    }

    const fn insert(&mut self, value: usize) -> bool {
        let index = value / 64;
        let bit = value % 64;
        let inserted_now = (self.bits[index] & (1 << bit)) == 0;
        self.bits[index] |= 1 << bit;
        inserted_now
    }

    const fn contains(&self, value: usize) -> bool {
        let index = value / 64;
        let bit = value % 64;
        (self.bits[index] & (1 << bit)) != 0
    }
}

#[test]
pub fn tests() {
    let mut b = BitSet::new();
    assert!(!b.contains(5));
    assert!(b.insert(0));
    assert!(b.insert(5));
    assert!(b.insert(64));
    assert!(b.insert(65));
    assert!(b.insert(512));
    assert!(!b.insert(512));
    assert!(b.contains(0));
    assert!(b.contains(5));
    assert!(b.contains(64));
    assert!(b.contains(65));
    assert!(b.contains(512));

    let test_input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
    test_part_one!(test_input => 5);
    let test_input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
    test_part_two!(test_input => 2);

    let real_input = include_str!("day11_input.txt");
    test_part_one!(real_input => 658);
    test_part_two!(real_input => 371_113_003_846_800);
}
