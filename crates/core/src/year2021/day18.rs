use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
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

type RegularNumber = u8;

#[derive(Copy, Clone)]
struct SnailfishElement {
    depth: u8,
    value: RegularNumber,
}

#[derive(Clone)]
struct SnailfishNumber {
    elements: Vec<SnailfishElement>,
}

impl SnailfishNumber {
    fn parse(text: &str) -> Result<Self, String> {
        let mut depth = 0;
        let mut elements = Vec::with_capacity(text.len() / 2);
        for char in text.as_bytes() {
            match char {
                b'[' => {
                    depth += 1;
                }
                b']' => {
                    if depth == 0 {
                        return Err("Too many closing ]".to_string());
                    }
                    depth -= 1;
                }
                c => {
                    if c.is_ascii_digit() {
                        if depth == 0 {
                            return Err("Found not nested number".to_string());
                        }
                        elements.push(SnailfishElement {
                            depth,
                            value: c - b'0',
                        });
                    }
                }
            }
        }
        if elements.is_empty() {
            return Err("No elements".to_string());
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
            self.elements[idx] = SnailfishElement { value: 0, depth: 4 };
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
            self.elements[idx + 1] = SnailfishElement {
                value: element.value / 2 + (element.value % 2),
                depth: new_depth,
            };
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
        self.elements.extend_from_slice(&other.elements);
        for element in self.elements.iter_mut() {
            element.depth += 1;
        }
        self.reduce();
    }

    fn magnitude(&self) -> u32 {
        fn recursive_magnitude(number: &[SnailfishElement], index: &mut usize, depth: u8) -> u32 {
            if *index >= number.len() {
                return 0;
            }
            let left = if number[*index].depth == depth {
                *index += 1;
                u32::from(number[*index - 1].value)
            } else {
                recursive_magnitude(number, index, depth + 1)
            };

            if *index >= number.len() {
                return 0;
            }
            let right = if number[*index].depth == depth {
                *index += 1;
                u32::from(number[*index - 1].value)
            } else {
                recursive_magnitude(number, index, depth + 1)
            };

            3 * left + 2 * right
        }
        recursive_magnitude(&self.elements, &mut 0, 1)
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

    test_part_one!("[8]" => 0);
}
