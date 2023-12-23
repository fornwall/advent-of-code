use crate::common::array_deque::ArrayDeque;
use crate::common::array_stack::ArrayStack;
use crate::common::u256::U256;
use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<u16, String> {
    let mut map = Map::parse(input.text.as_bytes())?;
    let mut visited = [U256::default(); 200];
    let mut longest = 0;

    let mut work_queue = ArrayStack::<1000, (u16, u16, u16)>::new();
    work_queue.push((1, 0, 0))?;

    let mut undo_stack = ArrayStack::<10000, (u16, u16)>::new();

    while let Some((x, y, path_len)) = work_queue.pop() {
        if x as usize == map.num_cols - 2 && y as usize == map.num_rows - 1 {
            longest = longest.max(path_len);
            continue;
        }

        while undo_stack.len() > path_len as usize {
            let (undo_x, undo_y) = undo_stack.pop_unwrap();
            visited[undo_y as usize].clear_bit(undo_x as usize);
        }

        let current_square = map.get(x as usize, y as usize);
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if input.is_part_one() {
                if (current_square == b'>' && dx != 1)
                    || (current_square == b'<' && dx != -1)
                    || (current_square == b'^' && dy != -1)
                    || (current_square == b'v' && dy != 1) {
                    continue;
                }
            }
            let new_x = (x as i32) + dx;
            let new_y = (y as i32) + dy;
            if new_x < 0 || new_x >= map.num_cols as i32
                || new_y < 0 || new_y >= map.num_rows as i32
                || map.get(new_x as usize, new_y as usize) == b'#' {
                continue;
            }
            if visited[new_y as usize].is_bit_set(new_x as usize) {
                continue;
            }
            undo_stack.push((new_x as u16, new_y as u16))?;
            visited[new_y as usize].set_bit(new_x as usize);
            work_queue.push((new_x as u16, new_y as u16, path_len + 1))?;
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
    //test_part_two_no_allocations!(real_input => 0);
}
