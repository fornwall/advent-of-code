use std::collections::HashSet;

struct DisjointSet {
    elements: Vec<i32>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            elements: vec![-1; size],
        }
    }

    fn find(&mut self, index: usize) -> usize {
        // Find root:
        let mut root_index = index;
        loop {
            let value = self.elements[root_index];
            if value < 0 {
                break;
            } else {
                root_index = value as usize;
            }
        }

        // Compress paths:
        let mut new_index = index;
        loop {
            let value = self.elements[new_index];
            if value < 0 {
                break;
            } else {
                self.elements[new_index] = root_index as i32;
                new_index = value as usize;
            }
        }

        root_index
    }

    fn join(&mut self, i: usize, j: usize) {
        let root1 = self.find(i);
        let root2 = self.find(j);

        if root1 == root2 {
            return;
        }

        let r1 = self.elements[root1];
        let r2 = self.elements[root2];

        // Join smaller tree with bigger:
        if r1 < r2 {
            self.elements[root1] += r2;
            self.elements[root2] = root1 as i32;
        } else {
            self.elements[root2] += r1;
            self.elements[root1] = root2 as i32;
        }
    }

    fn size(&mut self, i: usize) -> usize {
        let root = self.find(i);
        -self.elements[root] as usize
    }

    fn num_groups(&mut self) -> usize {
        let mut set = HashSet::new();
        for i in 0..self.elements.len() {
            set.insert(self.find(i));
        }
        set.len()
    }
}

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
fn test_disjoint_set() {
    let mut set = DisjointSet::new(10);
    assert_eq!(1, set.size(0));
    assert_eq!(10, set.num_groups());

    set.join(0, 1);
    assert_eq!(2, set.size(0));
    assert_eq!(2, set.size(1));
    assert_eq!(9, set.num_groups());

    set.join(2, 3);
    assert_eq!(2, set.size(0));
    assert_eq!(2, set.size(3));
    assert_eq!(8, set.num_groups());

    set.join(1, 3);
    assert_eq!(4, set.size(0));
    assert_eq!(4, set.size(3));
    assert_eq!(7, set.num_groups());
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
