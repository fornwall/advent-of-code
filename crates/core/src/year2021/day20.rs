use crate::input::Input;
use std::collections::HashSet;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let (algorithm, mut image) = parse(input.text)?;
    for _ in 0..input.part_values(2, 50) {
        image = algorithm.enhance(&image);
    }
    Ok(image.lit_pixels.len() as u32)
}

type PixelCoordinate = i32;

struct Image {
    lit_pixels: HashSet<(PixelCoordinate, PixelCoordinate)>,
    even_run: bool,
    infinity_flashing: bool,
    min_x: PixelCoordinate,
    max_x: PixelCoordinate,
    min_y: PixelCoordinate,
    max_y: PixelCoordinate,
}

impl Image {
    fn new(
        lit_pixels: HashSet<(PixelCoordinate, PixelCoordinate)>,
        even_run: bool,
        infinity_flashing: bool,
    ) -> Self {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for &lit_pixel in lit_pixels.iter() {
            min_x = std::cmp::min(min_x, lit_pixel.0);
            max_x = std::cmp::max(max_x, lit_pixel.0);
            min_y = std::cmp::min(min_y, lit_pixel.1);
            max_y = std::cmp::max(max_y, lit_pixel.1);
        }

        Self {
            lit_pixels,
            even_run,
            infinity_flashing,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn is_lit(&self, pixel: (PixelCoordinate, PixelCoordinate)) -> usize {
        if (self.min_x..=self.max_x).contains(&pixel.0)
            && (self.min_y..=self.max_y).contains(&pixel.1)
        {
            self.lit_pixels.contains(&pixel) as usize
        } else {
            (self.infinity_flashing && !self.even_run) as usize
        }
    }
}

struct ImageEnhancementAlgorithm {
    mappings: [bool; 512],
}

impl ImageEnhancementAlgorithm {
    fn parse(text: &str) -> Self {
        let mut mappings = [false; 512];
        for (idx, value) in text.bytes().enumerate() {
            mappings[idx] = value == b'#';
        }
        Self { mappings }
    }

    fn enhance(&self, image: &Image) -> Image {
        #![allow(clippy::unusual_byte_groupings, clippy::identity_op)]
        let mut lit_pixels = HashSet::new();

        for x in (image.min_x - 1)..=(image.max_x + 1) {
            for y in (image.min_y - 1)..=(image.max_y + 1) {
                let algorithm_idx = 0b100_000_000 * image.is_lit((x - 1, y - 1))
                    + 0b010_000_000 * image.is_lit((x, y - 1))
                    + 0b001_000_000 * image.is_lit((x + 1, y - 1))
                    + 0b000_100_000 * image.is_lit((x - 1, y))
                    + 0b000_010_000 * image.is_lit((x, y))
                    + 0b000_001_000 * image.is_lit((x + 1, y))
                    + 0b000_000_100 * image.is_lit((x - 1, y + 1))
                    + 0b000_000_010 * image.is_lit((x, y + 1))
                    + 0b000_000_001 * image.is_lit((x + 1, y + 1));
                if self.mappings[algorithm_idx] {
                    lit_pixels.insert((x, y));
                }
            }
        }

        Image::new(lit_pixels, !image.even_run, image.infinity_flashing)
    }
}

fn parse(text: &str) -> Result<(ImageEnhancementAlgorithm, Image), String> {
    let (algo_string, image_string) = text.split_once("\n\n").ok_or("No two blocks of text")?;
    if algo_string.len() != 512 {
        return Err("Image enhancement algorithm is not 512 chars long".to_string());
    }

    let algo = ImageEnhancementAlgorithm::parse(algo_string);
    if algo.mappings[0] && algo.mappings[511] {
        return Err(
            "Cannot have both first and last output pixels set in image enhancement algorithm"
                .to_string(),
        );
    }

    let mut lit_pixels = HashSet::new();
    for (y, line) in image_string.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                lit_pixels.insert((x as PixelCoordinate, y as PixelCoordinate));
            }
        }
    }
    let image = Image::new(lit_pixels, true, algo.mappings[0]);
    Ok((algo, image))
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
    test_part_one!(example => 35);
    test_part_two!(example => 3351);

    let real_input = include_str!("day20_input.txt");
    test_part_one!(real_input => 4917);
    test_part_two!(real_input => 16389);
}
