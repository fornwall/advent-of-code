use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut presents = [PresentShape { bits: 0 }; 6];
    let mut parts = input.text.split("\n\n");
    for present in presents.iter_mut() {
        let shape_str = parts.next().ok_or_else(on_error)?;
        for (line_idx, line) in shape_str.lines().skip(1).enumerate() {
            for (x, ch) in line.chars().enumerate() {
                present.bits |= u16::from(ch == '#') * (1 << (x as u16 + line_idx as u16 * 3));
            }
        }
    }

    let mut num_valid_regions = 0;

    let regions = parts.next().ok_or_else(|| "Missing regions".to_string())?;
    for region_str in regions.lines() {
        let (size_str, quantities_str) = region_str
            .split_once(": ")
            .ok_or_else(|| format!("Incorrect line format: {region_str}"))?;
        let (width_str, height_str) = size_str
            .split_once('x')
            .ok_or_else(|| format!("Incorrect size format: {size_str}"))?;
        let (width, height) = (
            width_str
                .parse::<u8>()
                .map_err(|_| format!("Invalid width: {}", width_str))?,
            height_str
                .parse::<u8>()
                .map_err(|_| format!("Invalid height: {}", height_str))?,
        );
        let mut quantities = [0u8; 6];
        for (i, qty_str) in quantities_str.split_whitespace().enumerate() {
            quantities[i] = qty_str
                .parse::<u8>()
                .map_err(|_| format!("Invalid quantity: {}", qty_str))?;
        }
        num_valid_regions += u64::from(does_presents_fit(presents, width, height, quantities));
    }

    Ok(num_valid_regions)
}

#[derive(Copy, Clone)]
struct PresentShape {
    bits: u16,
}

fn does_presents_fit(
    presents: [PresentShape; 6],
    width: u8,
    height: u8,
    quantities: [u8; 6],
) -> bool {
    let needed = presents
        .iter()
        .zip(quantities.iter())
        .map(|(shape, &quantity)| u32::from(quantity) * shape.bits.count_ones())
        .sum::<u32>();
    let available = u32::from(width) * u32::from(height);
    needed <= available
}

#[test]
pub fn tests() {
    use crate::input::test_part_one_no_allocations;
    let real_input = include_str!("day12_input.txt");
    test_part_one_no_allocations!(real_input => 440);
}
