use crate::input::Input;

fn sizes<'a, I: Iterator<Item = &'a str>>(lines: &mut I, delete_from: Option<u64>) -> (u64, u64) {
    let mut dir_size = 0;
    let mut dir_result = if delete_from.is_none() { 0 } else { u64::MAX };

    while let Some(line) = lines.next() {
        if line.starts_with("$ cd ..") {
            break;
        } else if line.starts_with("$ cd ") {
            let (subdir_size, subdir_matches) = sizes(lines, delete_from);
            dir_size += subdir_size;
            if delete_from.is_none() {
                dir_result += subdir_matches;
            } else {
                dir_result = dir_result.min(subdir_matches);
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
            // Ignore
        } else {
            let file_size = line
                .split(' ')
                .next()
                .and_then(|word| word.parse::<u32>().ok())
                .unwrap_or_default();
            dir_size += u64::from(file_size);
        }
    }

    if let Some(delete_bigger_than) = delete_from {
        if dir_size >= delete_bigger_than {
            dir_result = dir_result.min(dir_size);
        }
    } else if dir_size <= 100_000 {
        dir_result += dir_size;
    }
    (dir_size, dir_result)
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    if input.is_part_one() {
        Ok(sizes(&mut input.text.lines().skip(1), None).1)
    } else {
        let root_dir_size = sizes(&mut input.text.lines().skip(1), None).0;
        // 70_000_000 - root_dir_size + delete_bigger_than >= 30_000_000 gives:
        let delete_bigger_than = root_dir_size - 40_000_000;
        Ok(sizes(&mut input.text.lines().skip(1), Some(delete_bigger_than)).1)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

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
    test_part_one!(test_input => 95_437);
    test_part_two!(test_input => 24_933_642);

    let real_input = include_str!("day07_input.txt");
    test_part_one!(real_input => 1_428_881);
    test_part_two!(real_input => 10_475_598);
}

#[cfg(feature = "count-allocations")]
#[test]
pub fn no_memory_allocations() {
    use crate::input::{test_part_one, test_part_two};
    let real_input = include_str!("day07_input.txt");
    let allocations = allocation_counter::count(|| {
        test_part_one!(real_input => 1_428_881);
        test_part_two!(real_input => 10_475_598);
    });
    assert_eq!(allocations, 0);
}
