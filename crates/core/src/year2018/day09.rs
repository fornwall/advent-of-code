use std::collections::VecDeque;

fn solve(input_string: &str, last_marble_multiplier: usize) -> Result<usize, String> {
    let parts: Vec<&str> = input_string.split_whitespace().collect();
    let num_players = parts[0].parse::<usize>().unwrap();
    let num_marbles = (parts[6].parse::<usize>().unwrap() + 1) * last_marble_multiplier; // 0 based.

    let mut player_scores = vec![0; num_players];

    // A double-ended queue representing the circular structure with the front
    // of the queue being the current marble and the ordering from front to back
    // of the queue represents clockwise ordering of the marbles. This means the
    // following operations can be implemented efficiently:
    // - Inserting at the current position: push_front().
    // - Moving clockwise: push_back(pop_front()).
    // - Moving counter-clockwise: push_front(pop_back()).
    let mut marbles = VecDeque::with_capacity(num_marbles);

    // "First, the marble numbered 0 is placed in the circle."
    marbles.push_back(0);

    for marble_number in 1..=num_marbles {
        let normal_case = marble_number % 23 != 0;
        if normal_case {
            // "Then, each Elf takes a turn placing the lowest-numbered remaining marble into the circle
            // between the marbles that are 1 and 2 marbles clockwise of the current marble. (When the
            // circle is large enough, this means that there is one marble between the marble that was
            // just placed and the current marble.) The marble that was just placed then becomes the
            // current marble."
            for _ in 0..2 {
                let popped = marbles.pop_front().unwrap();
                marbles.push_back(popped);
            }
            marbles.push_front(marble_number);
        } else {
            // "However, if the marble that is about to be placed has a number which is a multiple of 23,
            // something entirely different happens.

            // "First, the current player keeps the marble they would have placed, adding it to their score":
            let player_number = marble_number % num_players;
            player_scores[player_number] += marble_number;

            // "In addition, the marble 7 marbles counter-clockwise
            // from the current marble is removed from the circle and also added to the current player's
            // score. The marble located immediately clockwise of the marble that was removed becomes
            // the new current marble."
            for _ in 0..7 {
                let popped = marbles.pop_back().unwrap();
                marbles.push_front(popped);
            }
            player_scores[player_number] += marbles.pop_front().unwrap();
        };
    }

    Ok(*player_scores.iter().max().unwrap())
}

pub fn part1(input_string: &str) -> Result<usize, String> {
    solve(input_string, 1)
}

pub fn part2(input_string: &str) -> Result<usize, String> {
    solve(input_string, 100)
}

#[test]
fn tests_part1() {
    assert_eq!(Ok(32), part1("9 players; last marble is worth 25 points"));
    assert_eq!(
        Ok(8317),
        part1("10 players; last marble is worth 1618 points")
    );
    assert_eq!(
        Ok(146_373),
        part1("13 players; last marble is worth 7999 points")
    );
    assert_eq!(
        Ok(2764),
        part1("17 players; last marble is worth 1104 points")
    );
    assert_eq!(
        Ok(54718),
        part1("21 players; last marble is worth 6111 points")
    );
    assert_eq!(
        Ok(37305),
        part1("30 players; last marble is worth 5807 points")
    );

    assert_eq!(Ok(423_717), part1(include_str!("day09_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(3_553_108_197), part2(include_str!("day09_input.txt")));
}
