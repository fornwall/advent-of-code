use crate::input::Input;

struct HeightMap {
    width: u8,
    height: u8,
    height_data: Vec<u8>,
}

impl HeightMap {
    fn parse(input: &str) -> Result<Self, String> {
        let width = input.lines().next().map(str::len).unwrap_or_default();
        let height = input.lines().count();

        let mut height_data = vec![0; width * height];
        for (y, line) in input.lines().enumerate() {
            if line.len() != width {
                return Err("All rows in the map does not have equal length".to_string());
            } else if !line.bytes().all(|b| b.is_ascii_digit()) {
                return Err("Every character in the map is not a digit".to_string());
            }
            for (x, height_digit) in line.bytes().enumerate() {
                height_data[x + y * width] = height_digit - b'0';
            }
        }

        Ok(Self {
            width: width as u8,
            height: height as u8,
            height_data,
        })
    }

    fn height_at(&self, x: i32, y: i32) -> u8 {
        if x < 0 || x >= i32::from(self.width) || y < 0 || y >= (i32::from(self.height)) {
            9
        } else {
            self.height_data[x as usize + (y as usize * self.width as usize)]
        }
    }

    fn fill_map(&self, filled_map: &mut [bool], x: i32, y: i32) -> usize {
        if self.height_at(x, y) == 9 {
            // "Locations of height 9 do not count as being in any basin".
            return 0;
        }
        let map_idx = x as usize + (y as usize * self.width as usize);
        if filled_map[map_idx] {
            0
        } else {
            filled_map[map_idx] = true;
            1 + self.fill_map(filled_map, x - 1, y)
                + self.fill_map(filled_map, x + 1, y)
                + self.fill_map(filled_map, x, y - 1)
                + self.fill_map(filled_map, x, y + 1)
        }
    }
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let map = HeightMap::parse(input.text)?;
    let mut risk_level_sum = 0;
    let mut filled_map = vec![false; map.height_data.len()];
    let mut basin_sizes = Vec::new();

    for y in 0..map.height {
        for x in 0..map.width {
            let y = i32::from(y);
            let x = i32::from(x);
            let h = map.height_at(x, y);
            if h < map.height_at(x - 1, y)
                && h < map.height_at(x + 1, y)
                && h < map.height_at(x, y - 1)
                && h < map.height_at(x, y + 1)
            {
                // Low point found.
                if input.is_part_one() {
                    risk_level_sum += u32::from(h) + 1;
                } else {
                    let basin_size = map.fill_map(&mut filled_map, x, y);
                    basin_sizes.push(basin_size as u32);
                }
            }
        }
    }

    if input.is_part_one() {
        Ok(risk_level_sum)
    } else {
        basin_sizes.sort_unstable();
        Ok(basin_sizes.iter().rev().take(3).product())
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "2199943210
3987894921
9856789892
8767896789
9899965678";
    test_part_one!(example => 15);
    test_part_two!(example => 1134);

    let real_input = include_str!("day09_input.txt");
    test_part_one!(real_input => 504);
    test_part_two!(real_input => 1_558_722);
}
