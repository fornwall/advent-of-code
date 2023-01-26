use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    const MAX_ITERATIONS: usize = 1000;
    let mut region = Region::parse(input.text)?;
    for round in 1..MAX_ITERATIONS {
        if region.update_is_stable() {
            return Ok(round as u32);
        }
    }
    Err(format!("Did not stabilize in {MAX_ITERATIONS} iterations"))
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Location {
    Empty,
    CucumberFacingEast,
    CucumberFacingSouth,
}

pub struct Region {
    locations: Vec<Location>,
    new_locations: Vec<Location>,
    width: u16,
    height: u16,
}

impl Region {
    fn parse(text: &str) -> Result<Self, String> {
        let height = text.lines().count();
        let width = text.lines().next().unwrap_or_default().len();
        if height < 1 || width < 1 {
            return Err("Too small input".to_string());
        }

        let mut locations = vec![Location::Empty; width * height];
        for (y, line) in text.lines().enumerate() {
            if line.len() != width {
                return Err("Not all lines have equal length".to_string());
            }
            for (x, byte) in line.bytes().enumerate() {
                let location = match byte {
                    b'.' => Location::Empty,
                    b'>' => Location::CucumberFacingEast,
                    b'v' => Location::CucumberFacingSouth,
                    _ => {
                        return Err("Invalid input".to_string());
                    }
                };
                locations[x + y * width] = location;
            }
        }
        Ok(Self {
            new_locations: locations.clone(),
            locations,
            width: width as u16,
            height: height as u16,
        })
    }

    fn update_is_stable(&mut self) -> bool {
        self.new_locations.fill(Location::Empty);

        for y in 0..self.height {
            for x in 0..self.width {
                let existing_idx = (x + y * self.width) as usize;
                if Location::CucumberFacingEast == self.locations[existing_idx] {
                    let new_x = (x + 1) % self.width;
                    let new_idx = (new_x + y * self.width) as usize;
                    self.new_locations[if matches!(self.locations[new_idx], Location::Empty) {
                        new_idx
                    } else {
                        existing_idx
                    }] = Location::CucumberFacingEast;
                }
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let existing_idx = (x + y * self.width) as usize;
                if Location::CucumberFacingSouth == self.locations[existing_idx] {
                    let new_y = (y + 1) % self.height;
                    let new_idx = (x + new_y * self.width) as usize;
                    let is_new_location_empty =
                        matches!(self.new_locations[new_idx], Location::Empty);
                    self.new_locations[if is_new_location_empty
                        && !matches!(self.locations[new_idx], Location::CucumberFacingSouth)
                    {
                        new_idx
                    } else {
                        existing_idx
                    }] = Location::CucumberFacingSouth;
                }
            }
        }

        std::mem::swap(&mut self.locations, &mut self.new_locations);
        self.locations == self.new_locations
    }
}

#[test]
pub fn tests() {
    use crate::input::test_part_one;

    let example = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
    test_part_one!(example => 58);

    let real_input = include_str!("day25_input.txt");
    test_part_one!(real_input => 582);
}
