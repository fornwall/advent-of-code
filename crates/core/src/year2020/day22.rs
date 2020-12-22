use crate::input::Input;
use std::collections::{HashSet, VecDeque};

enum Winner {
    Player1,
    Player2,
}

fn parse_player_cards(input: &str) -> Option<VecDeque<u8>> {
    input
        .lines()
        .skip(1)
        .map(str::parse)
        .collect::<Result<_, _>>()
        .ok()
}

fn play_recursive(player_1_cards: &mut VecDeque<u8>, player_2_cards: &mut VecDeque<u8>) -> Winner {
    #![allow(clippy::unwrap_used)]
    let mut seen_1 = HashSet::new();
    let mut seen_2 = HashSet::new();

    loop {
        if !(seen_1.insert(player_1_cards.clone()) && seen_2.insert(player_2_cards.clone())) {
            return Winner::Player1;
        } else if player_1_cards.is_empty() {
            return Winner::Player2;
        } else if player_2_cards.is_empty() {
            return Winner::Player1;
        }

        let c1 = player_1_cards.pop_front().unwrap();
        let c2 = player_2_cards.pop_front().unwrap();

        let winner = if player_1_cards.len() >= c1 as usize && player_2_cards.len() >= c2 as usize {
            let mut p1_subgame_cards = player_1_cards
                .iter()
                .take(c1 as usize)
                .copied()
                .collect::<VecDeque<u8>>();
            let mut p2_subgame_cards = player_2_cards
                .iter()
                .take(c2 as usize)
                .copied()
                .collect::<VecDeque<u8>>();
            play_recursive(&mut p1_subgame_cards, &mut p2_subgame_cards)
        } else if c1 > c2 {
            Winner::Player1
        } else {
            Winner::Player2
        };

        match winner {
            Winner::Player1 => {
                player_1_cards.push_back(c1);
                player_1_cards.push_back(c2);
            }
            Winner::Player2 => {
                player_2_cards.push_back(c2);
                player_2_cards.push_back(c1);
            }
        }
    }
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    #![allow(clippy::unwrap_used)]
    let on_error = || "Invalid input".to_string();

    let mut parts = input.text.splitn(2, "\n\n");
    let mut player_1_cards =
        parse_player_cards(parts.next().ok_or_else(on_error)?).ok_or_else(on_error)?;
    let mut player_2_cards =
        parse_player_cards(parts.next().ok_or_else(on_error)?).ok_or_else(on_error)?;

    let winner = if input.is_part_one() {
        while !player_1_cards.is_empty() && !player_2_cards.is_empty() {
            let c1 = player_1_cards.pop_front().unwrap();
            let c2 = player_2_cards.pop_front().unwrap();
            if c1 > c2 {
                player_1_cards.push_back(c1);
                player_1_cards.push_back(c2);
            } else {
                player_2_cards.push_back(c2);
                player_2_cards.push_back(c1);
            }
        }
        if player_1_cards.is_empty() {
            Winner::Player2
        } else {
            Winner::Player1
        }
    } else {
        play_recursive(&mut player_1_cards, &mut player_2_cards)
    };

    let winner_cards = match winner {
        Winner::Player1 => player_1_cards,
        Winner::Player2 => player_2_cards,
    };

    Ok(winner_cards
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, &card)| (idx + 1) as u64 * u64::from(card))
        .sum())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";
    test_part_one!(example => 306);
    test_part_two!(example => 291);

    let real_input = include_str!("day22_input.txt");
    test_part_one!(real_input => 33098);
    test_part_two!(real_input => 35055);
}
