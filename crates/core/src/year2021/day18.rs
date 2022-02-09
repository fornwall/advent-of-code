use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut lines = input.text.lines();
    if input.is_part_one() {
        let mut sum = SnailfishNumber::parse(lines.next().ok_or("Empty input")?)?;
        for line in lines {
            let number = SnailfishNumber::parse(line)?;
            sum.add(&number);
        }
        Ok(sum.magnitude())
    } else {
        let mut highest = 0;
        let numbers = lines
            .map(SnailfishNumber::parse)
            .collect::<Result<Vec<_>, _>>()?;
        for (idx1, n1) in numbers.iter().enumerate() {
            for (idx2, n2) in numbers.iter().enumerate() {
                if idx1 != idx2 {
                    let mut n = (*n1).clone();
                    n.add(n2);
                    highest = std::cmp::max(highest, n.magnitude());
                }
            }
        }
        Ok(highest)
    }
}

type RegularNumber = u32;

#[derive(Copy, Clone)]
struct SnailfishElement {
    depth: u16,
    value: RegularNumber,
}

#[derive(Clone)]
struct SnailfishNumber {
    elements: Vec<SnailfishElement>,
}

impl SnailfishNumber {
    fn parse(text: &str) -> Result<Self, String> {
        let mut depth = 0;
        let mut elements = Vec::new();
        for char in text.chars() {
            match char {
                '[' => {
                    depth += 1;
                }
                ']' => {
                    if depth == 0 {
                        return Err("Too many closing ]".to_string());
                    }
                    depth -= 1;
                }
                c => {
                    if let Some(value) = c.to_digit(10) {
                        elements.push(SnailfishElement { depth, value });
                    }
                }
            }
        }
        Ok(Self { elements })
    }

    fn check_explode(&mut self, idx: usize) -> bool {
        let element = self.elements[idx];

        if element.depth > 4 {
            if idx != 0 {
                // "The pair's left value is added to the first regular number to the left of the exploding pair (if any)".
                self.elements[idx - 1].value += element.value;
            }
            if idx + 2 < self.elements.len() {
                // "The pair's right value is added to the first regular number to the right of the exploding pair (if any)."
                self.elements[idx + 2].value += self.elements[idx + 1].value;
            }

            // "Then, the entire exploding pair is replaced with the regular number 0.":
            self.elements.remove(idx);
            self.elements[idx].value = 0;
            self.elements[idx].depth = 4;
            true
        } else {
            false
        }
    }

    fn check_split(&mut self, idx: usize) -> bool {
        let element = self.elements[idx];
        if element.value > 9 {
            // "To split a regular number, replace it with a pair; the left element of the pair should be the regular number
            // divided by two and rounded down, while the right element of the pair should be the regular number divided by
            // two and rounded up".
            let new_depth = element.depth + 1;
            self.elements.insert(
                idx,
                SnailfishElement {
                    depth: new_depth,
                    value: element.value / 2,
                },
            );
            self.elements[idx + 1].value = element.value / 2 + (element.value % 2);
            self.elements[idx + 1].depth = new_depth;
            true
        } else {
            false
        }
    }

    fn reduce(&mut self) {
        loop {
            if (0..self.elements.len()).any(|idx| self.check_explode(idx)) {
                continue;
            }
            if (0..self.elements.len()).any(|idx| self.check_split(idx)) {
                continue;
            }
            break;
        }
    }

    fn add(&mut self, other: &Self) {
        self.elements.extend(other.elements.iter());
        for element in self.elements.iter_mut() {
            element.depth += 1;
        }
        self.reduce();
    }

    fn magnitude(&mut self) -> u32 {
        for depth in (1..=4).rev() {
            while let Some(idx) = self.elements.iter().position(|e| e.depth == depth) {
                let a = self.elements.remove(idx);
                let b = self.elements.remove(idx);
                self.elements.insert(
                    idx,
                    SnailfishElement {
                        depth: depth - 1,
                        value: 3 * a.value + 2 * b.value,
                    },
                );
            }
        }
        self.elements.first().map(|n| n.value).unwrap_or_default()
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("[[1,2],[[3,4],5]]" => 143);
    test_part_one!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]" => 1384);
    test_part_one!("[[[[1,1],[2,2]],[3,3]],[4,4]]" => 445);
    test_part_one!("[[[[3,0],[5,3]],[4,4]],[5,5]]" => 791);
    test_part_one!("[[[[5,0],[7,4]],[5,5]],[6,6]]" => 1137);
    test_part_one!("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]" => 3488);
    test_part_one!("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]" => 4140);

    let example = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
    test_part_one!(example => 4140);
    test_part_two!(example => 3993);

    let real_input = include_str!("day18_input.txt");
    test_part_one!(real_input => 3793);
    test_part_two!(real_input => 4695);
}
