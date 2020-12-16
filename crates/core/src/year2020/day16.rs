use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut parts = input.text.split("\n\n");

    let ticket_fields_str = parts.next().unwrap();
    let your_ticket_str = parts.next().unwrap();
    let nearby_tickets_str = parts.next().unwrap();

    let mut valid_ranges = Vec::new();
    for line in ticket_fields_str.lines() {
        let or_str = line.splitn(2, ": ").nth(1).unwrap();
        let mut line_parts = or_str.split(" or ");
        for _ in 0..2 {
            let range_str = line_parts.next().unwrap();
            let mut range_parts = range_str.split('-');
            let range_start = range_parts.next().unwrap().parse::<u32>().unwrap();
            let range_end = range_parts.next().unwrap().parse::<u32>().unwrap();
            valid_ranges.push(range_start..=range_end);
        }
    }

    let mut error_rate = 0;
    for line in nearby_tickets_str.lines().skip(1) {
        'outer: for field_value_str in line.split(',') {
            let field_value = field_value_str.parse::<u32>().unwrap();
            for range in valid_ranges.iter() {
                if range.contains(&field_value) {
                    //println!("Valid: {}", field_value);
                    continue 'outer;
                }
            }
            //println!("Invalid: {}", field_value);
            error_rate += field_value;
        }
    }

    Ok(error_rate)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    test_part_one!(example => 71);
    //test_part_two!(example => 175_594);

    let real_input = include_str!("day16_input.txt");
    test_part_one!(real_input => 29019);
    //test_part_two!(real_input => 48710);
}
