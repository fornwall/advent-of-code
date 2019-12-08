extern crate bytecount;

fn count(slice: &[u8], wanted_by: u8) -> usize {
    bytecount::count(slice, wanted_by)
}

pub fn part1(input_string: &str) -> String {
    let layer_size = 150;
    assert_eq!(input_string.len() % 150, 0);

    let (_, slice) = input_string
        .as_bytes()
        .chunks(layer_size)
        .map(|slice| {
            let num_zeros = count(slice, b'0');
            (num_zeros, slice)
        })
        .min_by_key(|(num_zeros, _)| *num_zeros)
        .expect("Nothing found by min_by_key()");

    let count_1 = count(slice, b'1');
    let count_2 = count(slice, b'2');
    let result = count_1 * count_2;

    result.to_string()
}

pub fn part2(_input_string: &str) -> String {
    String::from("")
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day08_input.txt")), "2413");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(""), "");

    // assert_eq!(part2(include_str!("day08_input.txt")), "");
}
