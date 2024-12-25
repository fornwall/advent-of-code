use std::collections::{HashMap, HashSet};

use crate::{
    common::{array_stack::ArrayStack, id_assigner::IdAssigner},
    input::{on_error, Input},
    year2017::disjoint_set::DisjointSet,
};

pub fn solve(input: &Input) -> Result<u32, String> {
    let mut with_t = ArrayStack::<1000, u16>::new();
    let mut to_adjacency = HashMap::new();

    for line in input.text.lines() {
        let (s1, s2) = line.split_once('-').ok_or_else(on_error)?;
        let (n1, n2) = (to_num(s1), to_num(s2));

        to_adjacency.entry(n1).or_insert_with(Vec::new).push(n2);
        to_adjacency.entry(n2).or_insert_with(Vec::new).push(n1);

        if s1.starts_with('t') {
            with_t.push(n1)?;
        }
        if s2.starts_with('t') {
            with_t.push(n2)?;
        }
    }

    let mut already_considered = HashSet::new();
    let mut result = 0;
    let slice = with_t.slice_mut();
    slice.sort_unstable();
    let mut last = u16::MAX;
    for n in slice {
        if *n != last {
            let adjacency_list = to_adjacency.get(n).unwrap();
            for (i, x) in adjacency_list.iter().enumerate() {
                for (j, y) in adjacency_list.iter().enumerate().skip(i + 1) {
                    if to_adjacency.get(x).unwrap().iter().any(|e| e == y) {
                        let mut aa = [*n, *x, *y];
                        aa.sort_unstable();
                        if already_considered.insert(aa) {
                            result += 1;
                        }
                    }
                }
            }
            last = *n;
        }
    }

    Ok(result)
}

fn to_num(p: &str) -> u16 {
    let p = p.as_bytes();
    u16::from(p[0] - b'a') + 29 * u16::from(p[1] - b'a')
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    test_part_one_no_allocations!(test_input => 7);
    //test_part_two_no_allocations!(test_input => 0);

    let real_input = include_str!("day23_input.txt");
    test_part_one_no_allocations!(real_input => 1175);
    //test_part_two_no_allocations!(real_input => 0);
}
