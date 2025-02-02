use std::collections::HashMap;

use crate::common::array_stack::ArrayStack;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<String, String> {
    let mut nodes = HashMap::<u16, Vec<u16>>::with_capacity(1_000);
    let mut edges = vec![[false; 676]; 676];

    for edge in input.text.as_bytes().chunks(6) {
        let from = to_num(&edge[..2]);
        let to = to_num(&edge[3..]);

        nodes.entry(from).or_default().push(to);
        nodes.entry(to).or_default().push(from);

        edges[from as usize][to as usize] = true;
        edges[to as usize][from as usize] = true;
    }

    let mut seen = [false; 1024];

    Ok(if input.is_part_one() {
        let mut triangles = 0;

        for &n1 in nodes.keys() {
            seen[n1 as usize] = true;

            if let Some(neighbours) = nodes.get(&n1) {
                for (i, &n2) in neighbours.iter().enumerate() {
                    for &n3 in neighbours.iter().skip(i) {
                        if !seen[n2 as usize]
                            && !seen[n3 as usize]
                            && edges[n2 as usize][n3 as usize]
                            && (starts_with_t(n1) || starts_with_t(n2) || starts_with_t(n3))
                        {
                            triangles += 1;
                        }
                    }
                }
            }
        }

        triangles.to_string()
    } else {
        let mut clique = ArrayStack::<1024, u16>::new();
        let mut largest = [0_u16; 1024];
        let mut largest_len = 0;

        for (n1, neighbours) in nodes {
            if !seen[n1 as usize] {
                clique.clear();
                clique.push(n1)?;

                for n2 in neighbours {
                    if clique
                        .slice()
                        .iter()
                        .all(|&c| edges[n2 as usize][c as usize])
                    {
                        seen[n2 as usize] = true;
                        clique.push(n2)?;
                    }
                }

                if clique.len() > largest_len {
                    largest_len = clique.len();
                    largest[0..largest_len].copy_from_slice(clique.slice());
                }
            }
        }

        let mut result = String::new();
        largest[0..largest_len].sort_unstable();
        let mut first = true;
        for &n in &largest[0..largest_len] {
            if first {
                first = false;
            } else {
                result.push(',');
            }
            result.push_str(&from_num(n));
        }
        result
    })
}

fn to_num(p: &[u8]) -> u16 {
    26 * u16::from(p[0] - b'a') + u16::from(p[1] - b'a')
}

fn from_num(num: u16) -> String {
    let first_char = ((num / 26) as u8 + b'a') as char;
    let second_char = ((num % 26) as u8 + b'a') as char;
    format!("{first_char}{second_char}")
}

const fn starts_with_t(num: u16) -> bool {
    let first_char = (num / 26) as u8 + b'a';
    first_char == b't'
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    assert_eq!("ab", from_num(to_num(b"ab")));
    assert_eq!("yz", from_num(to_num(b"yz")));

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
    test_part_one!(test_input => "7".to_string());
    test_part_two!(test_input => "co,de,ka,ta".to_string());

    let real_input = include_str!("day23_input.txt");
    test_part_one!(real_input => "1175".to_string());
    test_part_two!(real_input => "bw,dr,du,ha,mm,ov,pj,qh,tz,uv,vq,wq,xw".to_string());
}
