use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<String, String> {
    println!("ABOUT TO SOLVE");
    let input_bytes = input.text.bytes();
    if input_bytes.len() != 9 {
        return Err(format!(
            "Format invalid input length - expected 9 characters, was {}",
            input_bytes.len()
        ));
    }

    let mut current_idx = 0_i32;
    let number_of_cups = input.part_values(9, 1_000_000);
    let max_cup_value = number_of_cups as i32;
    let mut cups = vec![0_u32; number_of_cups];
    let mut next_cups = vec![0_u32; number_of_cups];

    let crab_moves = input.part_values(100, 10_000_000);

    for (idx, byte) in input_bytes.enumerate() {
        if !byte.is_ascii_digit() {
            return Err("Invalid input - not all ASCII digits".to_string());
        }
        cups[idx as usize] = (byte - b'0') as u32;
    }

    if input.is_part_two() {
        for i in 9..number_of_cups {
            cups[i] = i as u32 + 1;
        }
    }

    let mut current_move = 1;
    loop {
        let current_cup = cups[current_idx as usize];
        if current_move < 5 {
            if input.is_part_one() {
                println!("move {}: cups = {:?}", current_move, cups);
            } else {
                println!("move {}: cups = {:?}", current_move, &cups[0..20]);
            }
            println!("Current cup {}", current_cup);
        }

        let mut destination_cup = if current_cup == 1 {
            max_cup_value as u32
        } else {
            current_cup - 1
        };
        while destination_cup == cups[((current_idx + 1) % max_cup_value) as usize]
            || destination_cup == cups[((current_idx + 2) % max_cup_value) as usize]
            || destination_cup == cups[((current_idx + 3) % max_cup_value) as usize]
        {
            if destination_cup == 1 {
                destination_cup = max_cup_value as u32;
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
            % (number_of_cups as i32);
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
        next_cups[(destination_idx + 1 - 4).rem_euclid(number_of_cups as i32) as usize] =
            cups[(destination_idx + 1 - 1).rem_euclid(number_of_cups as i32) as usize];

        // Bring over dropped cups:
        /*
        println!(
            "First drop after: {}",
            (destination_idx + 1 - 3).rem_euclid(9)
        );
         */
        next_cups[(destination_idx + 1 - 3).rem_euclid(max_cup_value) as usize] =
            cups[((current_idx + 1) % max_cup_value) as usize];
        next_cups[(destination_idx + 2 - 3).rem_euclid(max_cup_value) as usize] =
            cups[((current_idx + 2) % max_cup_value) as usize];
        next_cups[(destination_idx + 3 - 3).rem_euclid(max_cup_value) as usize] =
            cups[((current_idx + 3) % max_cup_value) as usize];

        // Bring over cups after destination:
        let mut i = 1;
        loop {
            let idx_after_destination = (destination_idx + i) % max_cup_value;
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
            let idx_after_picked_up = (current_idx + i) % max_cup_value;
            if (idx_after_picked_up + 3) % max_cup_value == destination_idx {
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
                    cups[((idx_after_picked_up + 3) % max_cup_value) as usize];
            }
            i += 1;
        }

        std::mem::swap(&mut cups, &mut next_cups);

        if current_move % 1000 == 0 {
            println!("Move {}", current_move);
        }

        if current_move == crab_moves {
            break;
        }
        current_idx = (current_idx + 1) % max_cup_value;
        current_move += 1;
    }

    let one_idx = cups
        .iter()
        .enumerate()
        .find_map(|(idx, &cup)| if cup == 1 { Some(idx) } else { None })
        .unwrap();

    Ok(if input.is_part_one() {
        cups.iter()
            .cycle()
            .skip((one_idx + 1) % number_of_cups)
            .take(8)
            .map(|&b| (b as u8 + b'0') as char)
            .collect()
    } else {
        (cups[(one_idx + 1) % number_of_cups] as u64 + cups[(one_idx + 1) % number_of_cups] as u64)
            .to_string()
    })
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "389125467";
    test_part_one!(example => "67384529".to_string());
    //test_part_two!(example => "149245887792".to_string());

    let real_input = include_str!("day23_input.txt");
    test_part_one!(real_input => "65432978".to_string());
    // test_part_two!(real_input => 0);
}
