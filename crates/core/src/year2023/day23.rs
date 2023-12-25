use crate::common::array_stack::ArrayStack;
use crate::common::id_assigner_copy::IdAssigner;
use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<u16, String> {
    const MAX_COMPACTED_GRAPH_LEN: usize = 60;

    let map = Map::parse(input.text.as_bytes())?;

    let mut compacted_graph = [([(0_u16, 0_u16); 5], 0_u8); MAX_COMPACTED_GRAPH_LEN];
    let mut id_assigner = IdAssigner::<MAX_COMPACTED_GRAPH_LEN, (u16, u16)>::new((0, 0));
    let mut work_queue = ArrayStack::<1000, (u16, u16, u16, u16, u16, u16)>::new();

    let src_idx = id_assigner.id_of((1, 0))?;
    work_queue.push((1, 0, 0, 0, 0, src_idx))?;
    let mut destination_idx = 0;

    while let Some((x, y, from_x, from_y, path_len, compact_src_idx)) = work_queue.pop() {
        let current_square = map.get(x as usize, y as usize);
        let mut possible_paths = ArrayStack::<3, (u16, u16)>::new();

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if input.is_part_one()
                && ((current_square == b'>' && dx != 1)
                    || (current_square == b'<' && dx != -1)
                    || (current_square == b'^' && dy != -1)
                    || (current_square == b'v' && dy != 1))
            {
                continue;
            }
            let new_x = i32::from(x) + dx;
            let new_y = i32::from(y) + dy;
            if new_x < 0
                || new_x >= map.num_cols as i32
                || new_y < 0
                || new_y >= map.num_rows as i32
                || (new_x as u16 == from_x && new_y as u16 == from_y)
                || map.get(new_x as usize, new_y as usize) == b'#'
            {
                continue;
            }
            possible_paths.push((new_x as u16, new_y as u16))?;
        }

        let at_destination = x as usize == (map.num_cols - 2) && y as usize == (map.num_rows - 1);

        let (new_compact_node_idx, new_path_len) = if possible_paths.len() > 1 || at_destination {
            // A new node in the compacted graph.
            let orig_len = id_assigner.len();
            let new_idx = id_assigner.id_of((x, y))?;
            let visited_before = orig_len == id_assigner.len();

            let mut existing_path_found = false;
            for &(from, to) in [(compact_src_idx, new_idx), (new_idx, compact_src_idx)]
                .iter()
                .take(input.part_values(1, 2))
            {
                let src_node = &mut compacted_graph[from as usize];
                for i in 0..src_node.1 {
                    if src_node.0[i as usize].0 == to {
                        existing_path_found = true;
                        src_node.0[i as usize].1 = src_node.0[i as usize].1.max(path_len);
                    }
                }
                if !existing_path_found {
                    src_node.0[src_node.1 as usize] = (to, path_len);
                    src_node.1 += 1;
                }
            }

            if at_destination {
                destination_idx = new_idx;
                continue;
            }

            if visited_before {
                continue;
            }
            (new_idx, 1)
        } else {
            // Not a new node in the compacted graph.
            (compact_src_idx, path_len + 1)
        };

        while let Some((new_x, new_y)) = possible_paths.pop() {
            work_queue.push((new_x, new_y, x, y, new_path_len, new_compact_node_idx))?;
        }
    }

    let mut longest = 0;
    let mut work_queue = ArrayStack::<1000, (u16, u64, u16)>::new();
    work_queue.push((src_idx, 1 << src_idx, 0))?;
    while let Some((idx, visited_bitmask, path_len)) = work_queue.pop() {
        let node = compacted_graph[idx as usize];
        for i in 0..node.1 {
            let (edge_destination_idx, edge_len) = node.0[i as usize];
            let edge_destination_bit = 1 << edge_destination_idx;
            if edge_destination_idx == destination_idx {
                longest = longest.max(path_len + edge_len);
                continue;
            }
            if visited_bitmask & edge_destination_bit == 0 {
                let new_visited_bitmask = visited_bitmask | edge_destination_bit;
                work_queue.push((
                    edge_destination_idx,
                    new_visited_bitmask,
                    path_len + edge_len,
                ))?;
            }
        }
    }

    Ok(longest)
}

struct Map<'a> {
    bytes: &'a [u8],
    num_rows: usize,
    num_cols: usize,
}

impl<'a> Map<'a> {
    fn parse(bytes: &'a [u8]) -> Result<Self, String> {
        let num_cols = bytes
            .iter()
            .position(|&b| b == b'\n')
            .ok_or_else(on_error)?;
        if (bytes.len() + 1) % (num_cols + 1) != 0 {
            return Err(on_error());
        }
        let num_rows = (bytes.len() + 1) / (num_cols + 1);
        Ok(Self {
            bytes,
            num_rows,
            num_cols,
        })
    }

    const fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        (self.num_cols + 1) * y + x
    }

    const fn get(&self, x: usize, y: usize) -> u8 {
        self.bytes[self.xy_to_idx(x, y)]
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    test_part_one_no_allocations!(test_input => 94);
    test_part_two_no_allocations!(test_input => 154);

    let real_input = include_str!("day23_input.txt");
    test_part_one_no_allocations!(real_input => 2042);
    test_part_two_no_allocations!(real_input => 6466);
}
