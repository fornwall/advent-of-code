use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<String, String> {
    let input_bytes = input.text.bytes();
    if input_bytes.len() != 9 {
        return Err(format!(
            "Format invalid input length - expected 9 characters, was {}",
            input_bytes.len()
        ));
    }

    let mut current_idx = 0_i32;
    let mut cups = [0_u8; 9];
    let mut next_cups = [0_u8; 9];

    for (idx, byte) in input_bytes.enumerate() {
        if !byte.is_ascii_digit() {
            return Err("Invalid input - not all ASCII digits".to_string());
        }
        cups[idx as usize] = byte - b'0';
    }

    let mut current_move = 1;
    loop {
        let current_cup = cups[current_idx as usize];
        //println!("move {}: cups = {:?}", current_move, cups);
        //println!("Current cup {}", current_cup);

        let mut destination_cup = if current_cup == 1 { 9 } else { current_cup - 1 };
        while destination_cup == cups[((current_idx + 1) % 9) as usize]
            || destination_cup == cups[((current_idx + 2) % 9) as usize]
            || destination_cup == cups[((current_idx + 3) % 9) as usize]
        {
            if destination_cup == 1 {
                destination_cup = 9;
            } else {
                destination_cup -= 1;
            }
            //println!("checking for destination..");
        }

        //println!("Searching for destination cup: {}", destination_cup);
        let destination_idx = (cups
            .iter()
            .enumerate()
            .find_map(|(idx, &value)| {
                if value == destination_cup {
                    Some(idx)
                } else {
                    None
                }
            })
            .unwrap()) as i32
            % 9;
        //println!("Destination cup value: {}", destination_cup);
        //println!("Destination cup index: {}", destination_idx);

        // Bring over current cup:
        next_cups[current_idx as usize] = cups[current_idx as usize];

        // Bring over target cups:
        /*
        println!(
            "Bring over target cup to {} from {}",
            (destination_idx + 1 - 4).rem_euclid(9),
            (destination_idx + 1 - 1).rem_euclid(9)
        );
         */
        next_cups[(destination_idx + 1 - 4).rem_euclid(9) as usize] =
            cups[(destination_idx + 1 - 1).rem_euclid(9) as usize];

        // Bring over dropped cups:
        /*
        println!(
            "First drop after: {}",
            (destination_idx + 1 - 3).rem_euclid(9)
        );
         */
        next_cups[(destination_idx + 1 - 3).rem_euclid(9) as usize] =
            cups[((current_idx + 1) % 9) as usize];
        next_cups[(destination_idx + 2 - 3).rem_euclid(9) as usize] =
            cups[((current_idx + 2) % 9) as usize];
        next_cups[(destination_idx + 3 - 3).rem_euclid(9) as usize] =
            cups[((current_idx + 3) % 9) as usize];

        // Bring over cups after destination:
        let mut i = 1;
        loop {
            let idx_after_destination = (destination_idx + i) % 9;
            if idx_after_destination == current_idx {
                break;
            } else {
                next_cups[idx_after_destination as usize] = cups[idx_after_destination as usize];
            }

            i += 1;
        }

        // Bring over cups after three picked up, but before destination:
        let mut i = 1;
        loop {
            let idx_after_picked_up = (current_idx + i) % 9;
            if (idx_after_picked_up + 3) % 9 == destination_idx {
                break;
            } else {
                /*
                println!(
                    "Bring over cups after three picked up, but before destination - to {} from {}",
                    idx_after_picked_up,
                    (idx_after_picked_up + 3) % 9,
                );
                 */
                next_cups[idx_after_picked_up as usize] =
                    cups[((idx_after_picked_up + 3) % 9) as usize];
            }
            i += 1;
        }

        std::mem::swap(&mut cups, &mut next_cups);

        // CURRENT b c d [...] DESTINATION [...]
        // CURRENT [....] DESTINATION b c d [...]
        if current_move == 100 {
            break;
        }
        println!("");
        current_idx = (current_idx + 1) % 9;
        current_move += 1;
    }

    let one_idx = cups
        .iter()
        .enumerate()
        .find_map(|(idx, &cup)| if cup == 1 { Some(idx) } else { None })
        .unwrap();
    println!("cups at end: {:?}", cups);
    println!("Index of one: {}", one_idx);
    Ok(cups
        .iter()
        .cycle()
        .skip((one_idx + 1) % 9)
        .take(8)
        .map(|b| (b + b'0') as char)
        .collect())
}

#[test]
pub fn tests() {
    use crate::test_part_one; //, test_part_two};

    let example_part_one = "389125467";
    test_part_one!(example_part_one => "67384529".to_string());
    //let example_part_two = "";
    //test_part_two!(example_part_two => 0);

    let real_input = include_str!("day23_input.txt");
    test_part_one!(real_input => "65432978".to_string());
    // test_part_two!(real_input => 0);
}
