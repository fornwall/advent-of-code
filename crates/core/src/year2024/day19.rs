use crate::input::{on_error, Input};

const NUM_NODES: usize = 1024;

pub fn solve(input: &Input) -> Result<u64, String> {
    let (patterns, designs) = input.text.split_once("\n\n").ok_or_else(on_error)?;

    let trie = patterns.split(", ").collect::<Trie>();

    let ceil = input.part_values(1, u64::MAX);
    Ok(designs
        .lines()
        .map(|design| trie.count(design).min(ceil))
        .sum())
}

#[derive(Clone, Copy)]
struct TrieNode {
    continuations: [u16; 5],
    terminal: bool,
}

struct Trie {
    nodes: [TrieNode; NUM_NODES],
    num_allocated: u16,
}

impl Default for Trie {
    fn default() -> Self {
        Self {
            nodes: [TrieNode {
                continuations: [u16::MAX; 5],
                terminal: false,
            }; NUM_NODES],
            num_allocated: 1,
        }
    }
}

impl<'a> FromIterator<&'a str> for Trie {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut trie = Self::default();
        for word in iter {
            trie.add(word);
        }
        trie
    }
}

impl Trie {
    fn add(&mut self, word: &str) {
        let mut current_node = &mut self.nodes[0];
        for b in word.bytes() {
            let child_node_idx = Self::map_to_range(b);
            let mut child_idx = current_node.continuations[child_node_idx as usize];
            if child_idx == u16::MAX {
                child_idx = self.num_allocated;
                current_node.continuations[child_node_idx as usize] = child_idx;
                self.num_allocated += 1;
            }
            current_node = &mut self.nodes[child_idx as usize];
        }
        current_node.terminal = true;
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
        let mut cache = [u64::MAX; NUM_NODES];
        self.count_worker(pattern, &mut cache, 0)
    }

    fn count_worker(&self, towel: &str, cache: &mut [u64; NUM_NODES], offset: usize) -> u64 {
        if cache[offset] != u64::MAX {
            return cache[offset];
        }
        let mut current_node = &self.nodes[0];
        let mut result = 0;
        for (internal_offset, b) in towel.bytes().skip(offset).enumerate() {
            let child_offset = Self::map_to_range(b);
            let child_idx = current_node.continuations[child_offset as usize];
            if child_idx == u16::MAX {
                break;
            }
            current_node = &self.nodes[child_idx as usize];
            if current_node.terminal {
                let new_offset = offset + internal_offset + 1;
                if new_offset == towel.len() {
                    result += 1;
                } else {
                    result += self.count_worker(towel, cache, new_offset);
                }
            }
        }
        cache[offset] = result;
        result
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let mut trie = Trie::default();
    trie.add("wub");
    trie.add("wuu");
    trie.add("bbb");
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
