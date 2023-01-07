use std::collections::VecDeque;
use std::num::NonZeroU32;

use crate::input::Input;

type MarbleValue = u32;

struct MarbleCircle {
    marbles: VecDeque<MarbleValue>,
}

impl MarbleCircle {
    fn new(size: u32) -> Self {
        Self {
            marbles: VecDeque::with_capacity(size as usize),
        }
    }

    fn add(&mut self, marble_number: MarbleValue) {
        self.marbles.push_front(marble_number);
    }

    fn move_clockwise(&mut self, steps: usize) {
        if self.marbles.len() > 1 {
            self.marbles.rotate_left(steps);
        }
    }

    fn move_counter_clockwise(&mut self, steps: usize) {
        self.marbles.rotate_right(steps);
    }

    fn take_current(&mut self) -> Option<MarbleValue> {
        self.marbles.pop_front()
    }
}

pub fn solve(input: &Input) -> Result<u32, String> {
    let last_marble_multiplier = input.part_values(1, 100);
    let parts: Vec<&str> = input.text.split_whitespace().collect();
    if parts.len() != 8 {
        return Err("Invalid input".to_string());
    }

    let num_players = u32::from(
        parts[0]
            .parse::<NonZeroU32>()
            .map_err(|_| "Invalid input")?,
    );
    let max_players = 999;
    if num_players > max_players {
        return Err(format!("Too many players (max: {})", max_players));
    }

    let last_marble_points = parts[6]
        .parse::<MarbleValue>()
        .map_err(|_| "Invalid input")?;
    let max_last_marble_points = 100_000;
    if last_marble_points > max_last_marble_points {
        return Err(format!(
            "Too high last marble value (max: {})",
            max_last_marble_points
        ));
    }
    let num_marbles = (last_marble_points) * last_marble_multiplier;

    let mut player_scores = vec![0_u32; num_players as usize];
    let mut marbles = MarbleCircle::new(num_marbles);

    // "First, the marble numbered 0 is placed in the circle."
    marbles.add(0);

    for marble_number in 1..=num_marbles {
        let normal_case = marble_number % 23 != 0;
        if normal_case {
            // "Then, each Elf takes a turn placing the lowest-numbered remaining marble into the circle
            // between the marbles that are 1 and 2 marbles clockwise of the current marble. (When the
            // circle is large enough, this means that there is one marble between the marble that was
            // just placed and the current marble.) The marble that was just placed then becomes the
            // current marble."
            marbles.move_clockwise(2);
            marbles.add(marble_number);
        } else {
            // "However, if the marble that is about to be placed has a number which is a multiple of 23,
            // something entirely different happens.

            // "First, the current player keeps the marble they would have placed, adding it to their score":
            let player_number = marble_number % num_players;
            // "In addition, the marble 7 marbles counter-clockwise
            // from the current marble is removed from the circle and also added to the current player's
            // score. The marble located immediately clockwise of the marble that was removed becomes
            // the new current marble."
            marbles.move_counter_clockwise(7);

            player_scores[player_number as usize] = player_scores[player_number as usize]
                .checked_add(marble_number + marbles.take_current().ok_or("No marble to pop")?)
                .ok_or("Aborting after too high score")?;
        };
    }

    player_scores
        .iter()
        .max()
        .ok_or_else(|| "No max value".to_string())
        .map(|value| *value)
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("9 players; last marble is worth 25 points" => 32);
    test_part_one!("10 players; last marble is worth 1618 points" => 8317);
    test_part_one!("13 players; last marble is worth 7999 points" => 146_373);
    test_part_one!("17 players; last marble is worth 1104 points" => 2764);
    test_part_one!("21 players; last marble is worth 6111 points" => 54718);
    test_part_one!("30 players; last marble is worth 5807 points" => 37305);
    test_part_one!("1 players; last marble is worth 22 points" => 0);

    let input = include_str!("day09_input.txt");
    test_part_one!(input => 423_717);
    test_part_two!(input => 3_553_108_197);
}
