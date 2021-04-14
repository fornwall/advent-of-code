use crate::Input;

struct Reindeer {
    speed: i32,
    duration: i32,
    rest: i32,
    points: i32,
    distance: i32,
}

pub fn solve(input: &mut Input) -> Result<i32, String> {
    const RACE_LENGTH_SECONDS: i32 = 2503;

    let mut reindeers = Vec::new();
    for line in input.text.lines() {
        // "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."
        let words = line.split(' ').collect::<Vec<_>>();
        if words.len() != 15 {
            return Err("Invalid reindeer line - not 14 words".to_string());
        }
        let speed = words[3]
            .parse::<i32>()
            .map_err(|_| "Invalid deer speed".to_string())?;
        let speed_duration = words[6]
            .parse::<i32>()
            .map_err(|_| "Invalid deer duration".to_string())?;
        let rest_duration = words[13]
            .parse::<i32>()
            .map_err(|_| "Invalid rest duration".to_string())?;
        reindeers.push(Reindeer {
            speed,
            duration: speed_duration,
            rest: rest_duration,
            points: 0,
            distance: 0,
        });
    }

    for second in input.part_values(RACE_LENGTH_SECONDS, 1)..=RACE_LENGTH_SECONDS {
        let mut best_distance = 0;
        for deer in reindeers.iter_mut() {
            let cycle_duration = deer.duration + deer.rest;
            let cycle_distance = deer.speed * deer.duration;
            let distance_from_full_cycles = (second / cycle_duration) * cycle_distance;
            let remaining_seconds = second % cycle_duration;
            let speed_during_remaining_seconds = std::cmp::min(remaining_seconds, deer.duration);
            let distance_during_remaining_seconds = deer.speed * speed_during_remaining_seconds;

            deer.distance = distance_from_full_cycles + distance_during_remaining_seconds;
            best_distance = std::cmp::max(best_distance, deer.distance);
        }

        for deer in reindeers.iter_mut() {
            if deer.distance == best_distance {
                deer.points += 1;
            }
        }
    }

    Ok(reindeers
        .iter()
        .map(|deer| input.part_values(deer.distance, deer.points))
        .max()
        .unwrap_or_default())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day14_input.txt");
    test_part_one!(real_input => 2696);
    test_part_two!(real_input => 1084);
}
