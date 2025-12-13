use crate::common::id_assigner::IdAssigner;
use crate::common::permutation::all_permutations;
use crate::common::tuple_window_iterator::TupleWindowIteratorExt;
use crate::input::Input;

const MAX_LOCATIONS: u16 = 10;

pub fn solve(input: &Input) -> Result<u32, String> {
    let mut id_assigner = IdAssigner::<{ MAX_LOCATIONS as usize }, str>::new("");

    let mut places = Vec::with_capacity(MAX_LOCATIONS as usize);
    let mut distances = [0; (MAX_LOCATIONS * MAX_LOCATIONS) as usize];
    for line in input.text.lines() {
        // "Faerun to Tristram = 58"
        let mut parts = line.split(' ');
        let from = id_assigner.id_of(parts.next().ok_or("Invalid input")?)?;
        let to = id_assigner.id_of(parts.nth(1).ok_or("Invalid input")?)?;
        let distance = parts
            .nth(1)
            .ok_or("Invalid input")?
            .parse::<u32>()
            .map_err(|_| "Invalid input")?;

        if !places.contains(&from) {
            places.push(from);
        }
        if !places.contains(&to) {
            places.push(to);
        }
        distances[(from + to * MAX_LOCATIONS) as usize] = distance;
        distances[(to + from * MAX_LOCATIONS) as usize] = distance;
    }

    let mut best_distance = input.part_values(u32::MAX, u32::MIN);
    let comparator = input.part_values(
        std::cmp::min as fn(_, _) -> _,
        std::cmp::max as fn(_, _) -> _,
    );

    all_permutations(&mut places, &mut |ordering| {
        let this_distance = ordering.iter().tuple_windows().fold(0, |acc, (p1, p2)| {
            acc + distances[(p1 + p2 * MAX_LOCATIONS) as usize]
        });
        best_distance = comparator(best_distance, this_distance);
        Ok(())
    })?;

    Ok(best_distance)
}

#[test]
pub fn tests() {
    let example_input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
    test_part_one!(example_input => 605);
    test_part_two!(example_input => 982);

    let real_input = include_str!("day09_input.txt");
    test_part_one!(real_input => 207);
    test_part_two!(real_input => 804);
}
