pub fn part1(input_string: &str) -> String {
    let mut deck = Vec::new();
    for i in 0..10_007 {
        deck.push(i);
    }

    for line in input_string.lines() {
        if line.starts_with("deal into") {
            deck = deck.iter().rev().copied().collect();
        } else if line.starts_with("cut") {
            let how_many = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let how_many = if how_many > 0 {
                how_many
            } else {
                (10_007 - how_many.abs())
            } as usize;
            deck = [&deck[how_many..], &deck[..how_many]].concat();
        } else if line.starts_with("deal with") {
            let increment = line
                .split_whitespace()
                .nth(3)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let old_deck = deck.clone();
            let mut current_index = 0;
            for &card_at_front in old_deck.iter() {
                deck[current_index] = card_at_front;
                current_index = (current_index + increment) % deck.len();
            }
        } else {
            panic!("Invalid line: {}", line);
        }
    }

    deck.iter()
        .position(|&card| card == 2019)
        .unwrap()
        .to_string()
}

pub fn part2(_input_string: &str) -> String {
    String::from("")
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day22_input.txt")), "6526");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(""), "");

    // assert_eq!(part2(include_str!("day22_input.txt")), "");
}
