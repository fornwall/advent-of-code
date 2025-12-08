use crate::common::highest_values::HighestValues;
use std::collections::BTreeMap;

use crate::common::array_stack::ArrayStack;
use crate::input::{Input, on_error};
use crate::year2017::disjoint_set::DisjointSet;

pub fn solve(input: &Input) -> Result<u32, String> {
    const MAX_POINTS: usize = 1000;

    let num_circuits = input.text.lines().count();
    let mut circuits = DisjointSet::<MAX_POINTS>::new(num_circuits);
    let mut points = ArrayStack::<MAX_POINTS, Point>::new();

    for line in input.text.lines() {
        let mut parts = line.split(',');
        let x = parts
            .next()
            .ok_or_else(on_error)?
            .parse::<u32>()
            .map_err(|_| on_error())?;
        let y = parts
            .next()
            .ok_or_else(on_error)?
            .parse::<u32>()
            .map_err(|_| on_error())?;
        let z = parts
            .next()
            .ok_or_else(on_error)?
            .parse::<u32>()
            .map_err(|_| on_error())?;
        let point = Point { x, y, z };
        points.push(point)?;
    }

    let mut distances = BTreeMap::new();
    for (lower_idx, lower_point) in points.slice().iter().enumerate() {
        for (higher_idx, higher_point) in points.slice().iter().enumerate().skip(lower_idx + 1) {
            let distance = lower_point.distance_from(higher_point);
            if distances
                .insert(distance, (lower_idx, higher_idx))
                .is_some()
            {
                return Err(format!("Duplicated distance: {}", distance));
            }
        }
    }

    let mut connections_made = 0;
    for (lower_point, higher_point) in distances.into_values() {
        if circuits.join(lower_point, higher_point)
            && input.is_part_two()
            && circuits.num_groups() == 1
        {
            return Ok(points.slice()[lower_point].x * points.slice()[higher_point].x);
        }

        connections_made += 1;
        if input.is_part_one() && connections_made == 1000 {
            let mut inspected_groups = [false; MAX_POINTS];
            let mut biggest_groups = HighestValues::<3>::new();
            for idx in 0..num_circuits {
                let group = circuits.find(idx);
                if !inspected_groups[group] {
                    inspected_groups[group] = true;
                    let group_size = circuits.size(idx) as u32;
                    biggest_groups.on_value(group_size as u64);
                }
            }
            return Ok(biggest_groups.values.iter().product::<u64>() as u32);
        }
    }
    Ok(0)
}

#[derive(Debug, Clone, Copy, Default)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Point {
    fn distance_from(&self, other: &Self) -> u64 {
        u64::from(self.x).abs_diff(u64::from(other.x)).pow(2)
            + u64::from(self.y).abs_diff(u64::from(other.y)).pow(2)
            + u64::from(self.z).abs_diff(u64::from(other.z)).pow(2)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    test_part_two_no_allocations!(test_input => 25272);

    let real_input = include_str!("day08_input.txt");
    test_part_one_no_allocations!(real_input => 123_930);
    test_part_two_no_allocations!(real_input => 27_338_688);
}
