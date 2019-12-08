use std::str;
extern crate bytecount;

const PIXELS_WIDE: u32 = 25;
const PIXELS_TALL: u32 = 6;
const LAYER_SIZE: usize = (PIXELS_WIDE * PIXELS_TALL) as usize;

pub fn part1(input_string: &str) -> String {
    assert_eq!(input_string.len() % LAYER_SIZE, 0);

    let (_, slice) = input_string
        .as_bytes()
        .chunks(LAYER_SIZE)
        .map(|slice| {
            let num_zeros = bytecount::count(slice, b'0');
            (num_zeros, slice)
        })
        .min_by_key(|(num_zeros, _)| *num_zeros)
        .expect("Nothing found by min_by_key()");

    let count_1 = bytecount::count(slice, b'1');
    let count_2 = bytecount::count(slice, b'2');
    let result = count_1 * count_2;

    result.to_string()
}

pub fn part2(input_string: &str) -> String {
    part2_sized(input_string, PIXELS_WIDE, PIXELS_TALL)
}

pub fn part2_sized(input_string: &str, width: u32, height: u32) -> String {
    let layer_size = (width * height) as usize;
    let mut image = vec![b'2'; layer_size];

    input_string
        .as_bytes()
        .chunks(layer_size)
        .for_each(|layer| {
            image
                .iter_mut()
                .zip(layer.iter())
                .for_each(|(image_pixel, &layer_pixel)| {
                    if *image_pixel == b'2' {
                        *image_pixel = layer_pixel;
                    }
                });
        });

    str::from_utf8(&image)
        .unwrap()
        .replace('1', "█")
        .replace('0', " ")
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % (width as usize) == 0 {
                Some('\n')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day08_input.txt")), "2413");
}

#[test]
fn tests_part2() {
    assert_eq!(part2_sized("0222112222120000", 2, 2), " █\n█ ");

    assert_eq!(part2(include_str!("day08_input.txt")),
"███   ██  ███  ████ ███  \n█  █ █  █ █  █    █ █  █ \n███  █    █  █   █  ███  \n█  █ █    ███   █   █  █ \n█  █ █  █ █    █    █  █ \n███   ██  █    ████ ███  ");
}
