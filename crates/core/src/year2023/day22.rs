use crate::common::array_deque::ArrayDeque;
use crate::common::array_stack::ArrayStack;
use crate::input::{on_error, Input};
use std::cmp::Ordering;

pub fn solve(input: &Input) -> Result<u32, String> {
    let mut bricks = ArrayStack::<1500, Brick>::new();

    for line in input.text.lines() {
        let (from, to) = line.split_once('~').ok_or_else(on_error)?;
        let mut seq = from.split(',').chain(to.split(','));
        let mut next_seq = || {
            seq.next()
                .ok_or_else(on_error)?
                .parse()
                .map_err(|_| on_error())
        };
        bricks.push(Brick {
            x1: next_seq()?,
            y1: next_seq()?,
            z1: next_seq()?,
            x2: next_seq()?,
            y2: next_seq()?,
            z2: next_seq()?,
            supported_by_count: 0,
            original_supported_by_count: 0,
        })?;
    }

    // Sort in ascending order - from lowest brick to highest.
    bricks.slice_mut().sort_unstable_by_key(|b| b.z1);

    // Settle bricks by letting them fall down until they
    // rest on a lower settled brick or the ground.
    for brick_idx in 0..bricks.len() {
        let brick = bricks.elements[brick_idx];
        let mut z_rest = 0;
        let mut supported_by_count = 0;
        for &lower_brick in bricks.elements[0..brick_idx].iter() {
            if brick.can_rest_on(lower_brick) {
                match lower_brick.z2.cmp(&z_rest) {
                    Ordering::Less => {}
                    Ordering::Equal => supported_by_count += 1,
                    Ordering::Greater => {
                        supported_by_count = 1;
                        z_rest = lower_brick.z2;
                    }
                }
            }
        }
        let brick = &mut bricks.elements[brick_idx];
        let brick_height = brick.z2 - brick.z1;
        brick.z1 = z_rest + 1;
        brick.z2 = z_rest + 1 + brick_height;
        brick.supported_by_count = supported_by_count;
        brick.original_supported_by_count = supported_by_count;
    }

    Ok(if input.is_part_one() {
        (0..bricks.len())
            .map(|i| {
                let this = bricks.elements[i];
                u32::from(!bricks.elements[(i + 1)..bricks.len()].iter().any(|other| {
                    other.z1 == this.z2 + 1
                        && other.can_rest_on(this)
                        && other.supported_by_count == 1
                }))
            })
            .sum()
    } else {
        let mut sum = 0;
        for i in 0..bricks.len() {
            let this = bricks.elements[i];
            let mut to_delete = ArrayDeque::<32, u16>::new();
            for j in (i + 1)..bricks.len() {
                let other = &mut bricks.elements[j];
                if other.z1 == this.z2 + 1
                    && other.can_rest_on(this)
                    && other.supported_by_count == 1
                {
                    to_delete.push_back(j as u16)?;
                }
            }
            while let Some(deleted_brick_idx) = to_delete.pop_front() {
                sum += 1;
                let this = bricks.elements[deleted_brick_idx as usize];
                for j in (deleted_brick_idx as usize + 1)..bricks.len() {
                    let other = &mut bricks.elements[j];
                    if other.z1 == this.z2 + 1 && other.can_rest_on(this) {
                        other.supported_by_count -= 1;
                        if other.supported_by_count == 0 {
                            to_delete.push_back(j as u16)?;
                        }
                    }
                }
            }
            let bricks_len = bricks.len();
            for j in &mut bricks.elements[(i + 1)..bricks_len] {
                j.supported_by_count = j.original_supported_by_count;
            }
        }
        sum as u32
    })
}

#[derive(Clone, Copy, Default)]
struct Brick {
    x1: u16,
    x2: u16,
    y1: u16,
    y2: u16,
    z1: u16,
    z2: u16,
    supported_by_count: u16,
    original_supported_by_count: u16,
}

impl Brick {
    const fn can_rest_on(&self, other: Self) -> bool {
        self.x2 >= other.x1 && self.x1 <= other.x2 && self.y2 >= other.y1 && self.y1 <= other.y2
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    test_part_one_no_allocations!(test_input => 5);
    test_part_two_no_allocations!(test_input => 7);

    let real_input = include_str!("day22_input.txt");
    test_part_one_no_allocations!(real_input => 443);
    test_part_two_no_allocations!(real_input => 69_915);
}
