use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    let (algorithm, image) = parse(input.text)?;
    let enhancement_steps = input.part_values(2, 50);
    let lit_pixels = algorithm.enhance(&image, enhancement_steps);
    Ok(lit_pixels)
}

struct Image {
    pixels: Vec<bool>,
}

impl Image {
    const fn new(pixels: Vec<bool>) -> Self {
        Self { pixels }
    }

    fn size(&self) -> usize {
        (self.pixels.len() as f64).sqrt() as usize
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

    fn enhance(&self, image: &Image, steps: usize) -> u32 {
        #![allow(clippy::unusual_byte_groupings)]
        const fn is_pixel_lit(
            pixels: &[bool],
            prev_dim: usize,
            image_size: usize,
            x: usize,
            y: isize,
            eternity_val: bool,
        ) -> bool {
            if x >= prev_dim || y < 0 || y >= (prev_dim as isize) {
                eternity_val
            } else {
                pixels[x + (y as usize) * image_size]
            }
        }

        let start_image_size = image.size();
        let final_image_size = start_image_size + 2 * steps;
        let mut current = vec![false; final_image_size * final_image_size];
        let mut new = vec![false; final_image_size * final_image_size];

        for (idx, &val) in image.pixels.iter().enumerate() {
            let x = idx % start_image_size;
            let y = idx / start_image_size;
            current[x + y * final_image_size] = val;
        }

        let flashes_at_infinity = self.flashes_at_infinity();

        for step in 1..=steps {
            let infinity_is_lit = flashes_at_infinity && step % 2 == 0;
            let round_size = start_image_size + 2 * (step - 1);

            for y in 0..start_image_size + 2 * step {
                let mut running_idx = if infinity_is_lit { 0b011_011_011 } else { 0 };

                for x in 0..start_image_size + 2 * step {
                    let above = y as isize - 2;
                    let at = y as isize - 1;
                    let below = y as isize;

                    let new_column = (usize::from(is_pixel_lit(
                        &current,
                        round_size,
                        final_image_size,
                        x,
                        above,
                        infinity_is_lit,
                    )) << 6)
                        | (usize::from(is_pixel_lit(
                            &current,
                            round_size,
                            final_image_size,
                            x,
                            at,
                            infinity_is_lit,
                        )) << 3)
                        | usize::from(is_pixel_lit(
                            &current,
                            round_size,
                            final_image_size,
                            x,
                            below,
                            infinity_is_lit,
                        ));
                    running_idx = ((running_idx << 1) & 0b110_110_110) | new_column;

                    new[x + y * final_image_size] = self.mappings[running_idx];
                }
            }

            std::mem::swap(&mut current, &mut new);
        }

        current.iter().filter(|&&b| b).count() as u32
    }

    const fn flashes_at_infinity(&self) -> bool {
        self.mappings[0]
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

    let image_height = image_string.lines().count();
    let image_width = image_string
        .lines()
        .next()
        .map(str::len)
        .unwrap_or_default();
    if image_height < 4 || image_width < 4 {
        return Err("Too small image (4x4 is minimal size)".to_string());
    }
    if image_height != image_width {
        return Err("Not a square image".to_string());
    }

    let mut image_pixels = vec![false; image_width * image_height];

    for (y, line_str) in image_string.lines().enumerate() {
        let line_bytes = line_str.bytes();
        if line_bytes.len() != image_width {
            return Err("Not all image rows have equal length".to_string());
        }
        for (x, b) in line_bytes.enumerate() {
            if b == b'#' {
                image_pixels[x + y * image_width] = true;
            }
        }
    }
    let image = Image::new(image_pixels);
    Ok((algo, image))
}

#[test]
pub fn tests() {
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
