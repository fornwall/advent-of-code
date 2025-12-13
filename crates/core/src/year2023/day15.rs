use crate::common::array_stack::ArrayStack;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    const LENSES: ArrayStack<8, Lens> = ArrayStack::new_const(Lens {
        label: 0,
        focal_length: 0,
    });

    if input.is_part_one() {
        return Ok(input
            .text
            .split(',')
            .map(|word| hash(word.as_bytes()))
            .sum());
    }

    let mut boxes = [LENSES; 256];

    for step in input.text.split(',') {
        let (label, operation) = step.strip_suffix('-').map_or_else(
            || {
                let label = &step[..step.len() - 2];
                let focal_length = u32::from(step.as_bytes()[step.len() - 1] - b'0');
                (label, Operation::Put(focal_length))
            },
            |label| (label, Operation::RemoveLabel),
        );

        let lenses = &mut boxes[hash(label.as_bytes()) as usize];
        let label = label_value(label.as_bytes());

        match operation {
            Operation::RemoveLabel => {
                lenses.retain(|x| x.label != label);
            }
            Operation::Put(focal_length) => {
                if !lenses.slice_mut().iter_mut().any(|lens| {
                    if lens.label == label {
                        lens.focal_length = focal_length;
                        true
                    } else {
                        false
                    }
                }) {
                    lenses.push(Lens {
                        label,
                        focal_length,
                    })?;
                }
            }
        }
    }

    Ok(boxes
        .iter()
        .enumerate()
        .map(|(box_number, lenses)| {
            (box_number + 1) as u32
                * lenses
                    .slice()
                    .iter()
                    .enumerate()
                    .map(|(slot_idx, lens)| (slot_idx + 1) as u32 * lens.focal_length)
                    .sum::<u32>()
        })
        .sum())
}

enum Operation {
    RemoveLabel,
    Put(u32),
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
struct Lens {
    label: u32,
    focal_length: u32,
}

fn hash(word: &[u8]) -> u32 {
    word.iter()
        .fold(0, |acc, &x| ((acc + u32::from(x)) * 17) & 0xff)
}

fn label_value(label: &[u8]) -> u32 {
    label.iter().fold(0, |acc, &x| {
        acc * u32::from(b'z' - b'a') + u32::from(x - b'a')
    })
}

#[test]
pub fn tests() {
    let test_input = "HASH";
    test_part_one_no_allocations!(test_input => 52);
    let test_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    test_part_one_no_allocations!(test_input => 1320);
    test_part_two_no_allocations!(test_input => 145);

    let real_input = include_str!("day15_input.txt");
    test_part_one_no_allocations!(real_input => 513_158);
    test_part_two_no_allocations!(real_input => 200_277);
}
