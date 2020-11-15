use crate::year2017::disjoint_set::DisjointSet;

fn solution(input_string: &str, part1: bool) -> Result<usize, String> {
    let num_programs = input_string.lines().count();
    let mut program_groups = DisjointSet::new(num_programs);

    for (line_index, line) in input_string.lines().enumerate() {
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

    Ok(if part1 {
        program_groups.size(0)
    } else {
        program_groups.num_groups()
    })
}

pub fn part1(input_string: &str) -> Result<usize, String> {
    solution(input_string, true)
}

pub fn part2(input_string: &str) -> Result<usize, String> {
    solution(input_string, false)
}

#[test]
fn test_part1() {
    assert_eq!(
        Ok(6),
        part1(
            "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"
        )
    );
    assert_eq!(Ok(152), part1(include_str!("day12_input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(Ok(186), part2(include_str!("day12_input.txt")));
}
