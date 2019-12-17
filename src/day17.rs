use crate::int_code::Program;

pub fn part1(input_string: &str) -> String {
    let mut program = Program::parse(input_string);
    program.run();
    let map: String = program
        .output_values
        .iter()
        .map(|&b| (b as u8) as char)
        .collect();
    part1_map(&map)
}

pub fn part1_map(map: &str) -> String {
    let map: Vec<&[u8]> = map.lines().map(|line| line.as_bytes()).collect();

    let mut alignment_parameters_sum = 0;
    for y in 1..(map.len() - 1) {
        for x in 1..(map[0].len() - 1) {
            if map[y][x] == b'#'
                && map[y][x - 1] == b'#'
                && map[y][x + 1] == b'#'
                && map[y - 1][x] == b'#'
                && map[y + 1][x] == b'#'
            {
                alignment_parameters_sum += x * y;
            }
        }
    }

    alignment_parameters_sum.to_string()
}

pub fn part2(_input_string: &str) -> String {
    String::from("")
}

#[test]
pub fn tests_part1() {
    assert_eq!(
        part1_map(
            "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..",
        ),
        "76"
    );

    assert_eq!(part1(include_str!("day17_input.txt")), "11140");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(""), "");

    // assert_eq!(part2(include_str!("day17_input.txt")), "");
}
