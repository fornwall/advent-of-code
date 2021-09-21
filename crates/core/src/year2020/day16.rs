use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u64, String> {
    const MAX_FIELD_VALUE: u32 = 1024;

    let mut parts = input.text.splitn(3, "\n\n");
    let on_error = || "Invalid input".to_string();
    let map_error = |_| on_error();

    let ticket_fields_str = parts.next().ok_or_else(on_error)?;
    let your_ticket_str = parts.next().ok_or_else(on_error)?;
    let nearby_tickets_str = parts.next().ok_or_else(on_error)?;

    let mut your_ticket_values = Vec::new();
    for part in your_ticket_str
        .lines()
        .nth(1)
        .ok_or_else(on_error)?
        .split(',')
    {
        your_ticket_values.push(part.parse::<u32>().map_err(map_error)?);
    }

    let mut departure_fields = Vec::with_capacity(6);
    let mut field_ranges = Vec::new();
    for line in ticket_fields_str.lines() {
        let mut field_range = vec![false; (MAX_FIELD_VALUE + 1) as usize];

        let mut line_parts = line.splitn(2, ": ");

        let field_name = line_parts.next().ok_or_else(on_error)?;
        if field_name.starts_with("departure") {
            departure_fields.push(field_ranges.len() as u32);
        }

        let or_str = line_parts.next().ok_or_else(on_error)?;
        let mut line_parts = or_str.splitn(2, " or ");
        for _ in 0..2 {
            let range_str = line_parts.next().ok_or_else(on_error)?;
            let mut range_parts = range_str.splitn(2, '-');

            let range_start = range_parts
                .next()
                .ok_or_else(on_error)?
                .parse::<u32>()
                .map_err(map_error)?;
            let range_end = range_parts
                .next()
                .ok_or_else(on_error)?
                .parse::<u32>()
                .map_err(map_error)?;

            if range_start >= range_end {
                return Err(format!("Invalid range: {}-{}", range_start, range_end));
            } else if range_end > MAX_FIELD_VALUE {
                return Err(format!(
                    "Too high field range (max: {}): {}",
                    MAX_FIELD_VALUE, range_end,
                ));
            }

            for value in range_start..=range_end {
                field_range[value as usize] = true;
            }
        }
        field_ranges.push(field_range);
    }

    if your_ticket_values.len() != field_ranges.len() {
        return Err(format!(
            "Your ticket contains {} fields, but {} fields are specified",
            your_ticket_values.len(),
            field_ranges.len()
        ));
    } else if field_ranges.len() > 32 {
        return Err(format!(
            "Max 32 fields supported (input had {})",
            field_ranges.len()
        ));
    }

    // Set the lowest field_ranges.len() bits:
    let possibilities_bitmask: u32 = (1 << field_ranges.len()) as u32 - 1;
    let mut possible_fields_for_position = vec![possibilities_bitmask; your_ticket_values.len()];

    let mut error_rate = 0;
    for line in nearby_tickets_str.lines().skip(1) {
        'outer: for (field_position, value_str) in line.split(',').enumerate() {
            let value = value_str.parse::<u32>().map_err(map_error)?;
            if value > MAX_FIELD_VALUE {
                return Err(format!("Invalid field value: {}", value));
            }
            let valid_ticket = field_ranges.iter().any(|range| range[value as usize]);
            if valid_ticket {
                if input.is_part_one() {
                    continue 'outer;
                } else {
                    for (field_idx, range) in field_ranges.iter().enumerate() {
                        if !range[value as usize] {
                            possible_fields_for_position[field_position] &= !(1 << field_idx);
                        }
                    }
                }
            }
            error_rate += value;
        }
    }

    if input.is_part_one() {
        Ok(u64::from(error_rate))
    } else {
        let mut departure_values_multiplied = 1_u64;
        let mut identified_positions = vec![false; field_ranges.len()];

        loop {
            let mut any_change = false;
            for position in 0..your_ticket_values.len() {
                let possible_fields = possible_fields_for_position[position];
                if possible_fields.count_ones() == 1 && !identified_positions[position] {
                    identified_positions[position] = true;
                    any_change = true;

                    let field_idx = possible_fields.trailing_zeros();
                    if departure_fields.contains(&field_idx) {
                        departure_values_multiplied *=
                            u64::from(your_ticket_values[position as usize]);
                    }

                    let clear_possibility_bitmask = !(1 << field_idx);
                    for (idx, possible_fields) in
                        possible_fields_for_position.iter_mut().enumerate()
                    {
                        if idx != position {
                            *possible_fields &= clear_possibility_bitmask;
                        }
                    }
                }
            }
            if !any_change {
                break;
            }
        }

        Ok(departure_values_multiplied)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example_part_one = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    test_part_one!(example_part_one => 71);

    let example_part_two = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
    test_part_two!(example_part_two => 1);

    let real_input = include_str!("day16_input.txt");
    test_part_one!(real_input => 29019);
    test_part_two!(real_input => 517_827_547_723);
}
