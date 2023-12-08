use crate::common::id_assigner::IdAssigner;
use crate::common::permutation::all_permutations;
use crate::input::Input;

const MAX_ATTENDEES: usize = 10;

pub fn solve(input: &Input) -> Result<i32, String> {
    let mut id_assigner = IdAssigner::<MAX_ATTENDEES, str>::new("");

    let mut happiness_changes = Vec::new();
    for line in input.text.lines() {
        // "Alice would lose 79 happiness units by sitting next to Carol."
        let words = line.split(' ').collect::<Vec<_>>();
        if words.len() != 11 {
            return Err("Invalid line not consisting of 11 words".to_string());
        }

        let person_name = words[0];
        let happiness_change = words[3]
            .parse::<i32>()
            .map_err(|_| "Invalid happiness change")?
            * if words[2] == "gain" { 1 } else { -1 };
        let other_name = &words[10]
            .strip_suffix('.')
            .ok_or_else(|| "Line not ending with a period".to_string())?;

        let person_id = id_assigner.id_of(person_name)? as usize;
        let other_id = id_assigner.id_of(other_name)? as usize;

        while person_id >= happiness_changes.len() {
            happiness_changes.push(Vec::new());
        }

        if other_id != happiness_changes[person_id].len() {
            // This person
            happiness_changes[person_id].push(0);
        }

        happiness_changes[person_id].push(happiness_change);
    }

    if input.is_part_two() {
        // Last person was not added 0 for.
        let last = happiness_changes.len() - 1;
        happiness_changes[last].push(0);

        let changes_for_me = vec![0; happiness_changes.len()];
        happiness_changes.push(changes_for_me);
        for change in happiness_changes.iter_mut() {
            change.push(0);
        }
    }

    let mut seating_arrangement = Vec::new();
    for i in 0..happiness_changes.len() {
        seating_arrangement.push(i);
    }

    let mut best_happiness = 0;
    all_permutations(&mut seating_arrangement, &mut |arrangement| {
        let mut this_happiness_change = 0;

        for i in 0..arrangement.len() {
            let this = arrangement[i];
            let next = arrangement[(i + 1) % arrangement.len()];
            this_happiness_change += happiness_changes[this][next] + happiness_changes[next][this];
        }

        best_happiness = std::cmp::max(best_happiness, this_happiness_change);
        Ok(())
    })?;

    Ok(best_happiness)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol." => 330);

    let real_input = include_str!("day13_input.txt");
    test_part_one!(real_input => 664);
    test_part_two!(real_input => 640);
}
