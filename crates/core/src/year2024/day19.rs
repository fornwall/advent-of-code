use crate::input::{on_error, Input};

const MAX_TRIE_NODES: usize = 1024;
const MAX_DESIGN_LEN: usize = 64;

pub fn solve(input: &Input) -> Result<u64, String> {
    let (patterns, designs) = input.text.split_once("\n\n").ok_or_else(on_error)?;

    let trie = Trie::try_from_iter(patterns.split(", "))?;

    let ceil = input.part_values(1, u64::MAX);
    designs
        .lines()
        .map(|design| {
            if design.len() > MAX_DESIGN_LEN {
                Err(format!("Too long design - max {MAX_DESIGN_LEN} supported"))
            } else {
                Ok(trie.count(design).min(ceil))
            }
        })
        .sum()
}

#[derive(Clone, Copy)]
struct TrieNode {
    continuations: [u16; 5],
    terminal: bool,
}

struct Trie {
    nodes: [TrieNode; MAX_TRIE_NODES],
    num_allocated: u16,
}

impl Default for Trie {
    fn default() -> Self {
        Self {
            nodes: [TrieNode {
                continuations: [u16::MAX; 5],
                terminal: false,
            }; MAX_TRIE_NODES],
            num_allocated: 1,
        }
    }
}

impl Trie {
    fn try_from_iter<'a, I: Iterator<Item = &'a str>>(iter: I) -> Result<Self, String> {
        let mut trie = Self::default();
        for word in iter {
            trie.add(word)?;
        }
        Ok(trie)
    }

    fn add(&mut self, word: &str) -> Result<(), String> {
        let mut current_node = &mut self.nodes[0];
        for b in word.bytes() {
            let child_node_idx = Self::map_to_range(b);
            let mut child_idx = current_node.continuations[child_node_idx as usize];
            if child_idx == u16::MAX {
                child_idx = self.num_allocated;
                if child_idx as usize >= MAX_TRIE_NODES {
                    return Err("Too many patterns".to_string());
                }
                current_node.continuations[child_node_idx as usize] = child_idx;
                self.num_allocated += 1;
            }
            current_node = &mut self.nodes[child_idx as usize];
        }
        current_node.terminal = true;
        Ok(())
    }

    const fn map_to_range(color: u8) -> u8 {
        // white (w), blue (u), black (b), red (r), or green (g).
        match color {
            b'w' => 0,
            b'u' => 1,
            b'b' => 2,
            b'r' => 3,
            _ => 4,
        }
    }

    fn count(&self, pattern: &str) -> u64 {
        let pattern = pattern.as_bytes();
        let mut endings_at = [0; MAX_DESIGN_LEN];

        for start in 0..pattern.len() {
            let num_starts = if start == 0 { 1 } else { endings_at[start - 1] };
            if num_starts > 0 {
                let mut current_node = &self.nodes[0];

                for end in start..pattern.len() {
                    let child_offset = Self::map_to_range(pattern[end]);
                    let child_idx = current_node.continuations[child_offset as usize];
                    if child_idx == u16::MAX {
                        break;
                    }
                    current_node = &self.nodes[child_idx as usize];
                    endings_at[end] += u64::from(current_node.terminal) * num_starts;
                }
            }
        }
        endings_at[pattern.len() - 1]
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let mut trie = Trie::default();
    trie.add("wub").unwrap();
    trie.add("wuu").unwrap();
    trie.add("bbb").unwrap();
    assert_eq!(trie.count("wub"), 1);
    assert_eq!(trie.count("wuu"), 1);
    assert_eq!(trie.count("bbb"), 1);
    assert_eq!(trie.count("wuw"), 0);

    let test_input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    test_part_one_no_allocations!(test_input => 6);
    test_part_two_no_allocations!(test_input => 16);

    let real_input = include_str!("day19_input.txt");
    test_part_one_no_allocations!(real_input => 290);
    test_part_two_no_allocations!(real_input => 712_058_625_427_487);
}
