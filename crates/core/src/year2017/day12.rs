use crate::year2017::disjoint_set::DisjointSet;
use crate::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    let num_programs = input.text.lines().count();
    let mut program_groups = DisjointSet::new(num_programs);

    for (line_index, line) in input.text.lines().enumerate() {
        let error_message = || {
            format!(
                "Invalid input at line {}: Expected 'ID <-> ID[, ID]'",
                line_index + 1
            )
        };
        let parts = line.split(" <-> ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(error_message());
        }
        let first = parts[0].parse::<usize>().map_err(|_| error_message())?;
        for other_str in parts[1].split(", ") {
            let other = other_str.parse::<usize>().map_err(|_| error_message())?;
            program_groups.join(first, other);
        }
    }

    Ok(if input.is_part_one() {
        program_groups.size(0)
    } else {
        program_groups.num_groups()
    })
}

#[test]
fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!(
            "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"
        => 6
    );

    let real_input = include_str!("day12_input.txt");
    test_part_one!(real_input => 152);
    test_part_two!(real_input => 186);
}
