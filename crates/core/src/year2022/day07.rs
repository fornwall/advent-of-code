use crate::input::Input;

struct Stack {
    data: [u64; Self::MAX_SIZE],
    size: usize,
}

impl Stack {
    const MAX_SIZE: usize = 500;

    const fn new() -> Self {
        Self {
            data: [0; Self::MAX_SIZE],
            size: 0,
        }
    }

    const fn push(&mut self, value: u64) {
        self.data[self.size] = value;
        self.size += 1;
    }

    fn data(&self) -> &[u64] {
        &self.data[0..self.size]
    }

    const fn is_full(&self) -> bool {
        self.size == Self::MAX_SIZE
    }
}

fn sizes<'a, I: Iterator<Item = &'a str>>(lines: &mut I, stack: &mut Stack) -> Result<u64, String> {
    if stack.is_full() {
        return Err(format!(
            "Stack overflow - max {} directories supported",
            Stack::MAX_SIZE
        ));
    }

    let mut dir_size = 0;

    while let Some(line) = lines.next() {
        if line.starts_with("$ cd ..") {
            break;
        } else if line.starts_with("$ cd ") {
            let subdir_size = sizes(lines, stack)?;
            dir_size += subdir_size;
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
            // Ignore
        } else {
            let file_size = line
                .split(' ')
                .next()
                .and_then(|word| word.parse::<u32>().ok())
                .ok_or_else(|| "Invalid file listing - not starting with a u32".to_string())?;
            dir_size += u64::from(file_size);
        }
    }

    stack.push(dir_size);
    Ok(dir_size)
}

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut dir_stack = Stack::new();
    sizes(&mut input.text.lines().skip(1), &mut dir_stack)?;
    if input.is_part_one() {
        Ok(dir_stack
            .data()
            .iter()
            .filter(|&&size| size <= 100_000)
            .sum())
    } else {
        let root_dir_size = dir_stack.data().last().copied().unwrap_or_default();
        // 70_000_000 - root_dir_size + delete_bigger_than >= 30_000_000 =>:
        let delete_bigger_than = root_dir_size.checked_sub(40_000_000).unwrap_or_default();
        Ok(dir_stack
            .data()
            .iter()
            .filter(|&&size| size >= delete_bigger_than)
            .min()
            .copied()
            .unwrap_or_default())
    }
}

#[test]
pub fn tests() {
    let test_input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    test_part_one_no_allocations!(test_input => 95_437);
    test_part_two_no_allocations!(test_input => 24_933_642);

    let real_input = include_str!("day07_input.txt");
    test_part_one_no_allocations!(real_input => 1_428_881);
    test_part_two_no_allocations!(real_input => 10_475_598);
}
