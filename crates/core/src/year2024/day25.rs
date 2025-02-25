use crate::common::array_stack::ArrayStack;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    let mut patterns = ArrayStack::<1024, u64>::new();
    let mut result = 0;
    for pattern in input.text.split("\n\n") {
        let pattern = pattern
            .trim_ascii()
            .bytes()
            .enumerate()
            .map(|(i, c)| u64::from(c == b'#') << i)
            .sum();
        for &y in patterns.slice() {
            result += u32::from(pattern & y == 0);
        }
        patterns.push(pattern)?;
    }
    Ok(result)
}

#[test]
pub fn tests() {
    use crate::input::test_part_one_no_allocations;

    let test_input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    test_part_one_no_allocations!(test_input => 3);

    let real_input = include_str!("day25_input.txt");
    test_part_one_no_allocations!(real_input => 3107);
}
