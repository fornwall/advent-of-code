use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    const MAX: i32 = 24;
    const SIZE: i32 = MAX + 2;
    let mut grid = [[[false; SIZE as usize]; SIZE as usize]; SIZE as usize];

    let points = input
        .text
        .lines()
        .map(|line| {
            fn parse_coordinate(input: &str) -> Option<i32> {
                let num = input.parse::<i32>().ok()?;
                (0..MAX).contains(&num).then_some(num + 1)
            }
            let mut parts = line.split(',');
            let point = (
                parse_coordinate(parts.next()?)?,
                parse_coordinate(parts.next()?)?,
                parse_coordinate(parts.next()?)?,
            );
            grid[point.0 as usize][point.1 as usize][point.2 as usize] = true;
            Some(point)
        })
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| {
            format!("Invalid input - expected lines as 'N,N,N' where N is in [0,{MAX});")
        })?;

    if input.is_part_one() {
        Ok(points
            .iter()
            .map(|&point| {
                adjacent(point)
                    .filter(|p| !grid[(p.0) as usize][(p.1) as usize][(p.2) as usize])
                    .count()
            })
            .sum())
    } else {
        let mut water_points = [[[false; SIZE as usize]; SIZE as usize]; SIZE as usize];
        let mut points_to_fill = Vec::with_capacity(MAX as usize * MAX as usize * MAX as usize);

        points_to_fill.push((0, 0, 0));
        water_points[0][0][0] = true;
        let mut wet_sides = 0;

        while let Some(point) = points_to_fill.pop() {
            for adjacent in adjacent(point) {
                if (0..SIZE).contains(&adjacent.0)
                    && (0..SIZE).contains(&adjacent.1)
                    && (0..SIZE).contains(&adjacent.2) {
                    if grid[(adjacent.0) as usize][(adjacent.1) as usize][(adjacent.2) as usize] {
                        wet_sides += 1;
                    } else if !water_points[(adjacent.0) as usize][(adjacent.1) as usize]
                        [(adjacent.2) as usize]
                    {
                        water_points[(adjacent.0) as usize][(adjacent.1) as usize]
                            [(adjacent.2) as usize] = true;
                        points_to_fill.push(adjacent);
                    }
                }
            }
        }
        Ok(wet_sides)
    }
}

fn adjacent(point: (i32, i32, i32)) -> impl Iterator<Item=(i32, i32, i32)> {
    [-1_i32, 0, 1]
        .iter()
        .flat_map(|&dx| [-1_i32, 0, 1].iter().map(move |&dy| (dx, dy)))
        .flat_map(move |(dx, dy)| {
            [-1_i32, 0, 1].iter().filter_map(move |&dz| {
                (dx.abs() + dy.abs() + dz.abs() == 1).then_some((
                    point.0 + dx,
                    point.1 + dy,
                    point.2 + dz,
                ))
            })
        })
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "1,1,1\n2,1,1";
    test_part_one!(test_input => 10);
    test_part_two!(test_input => 10);

    let test_input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    test_part_one!(test_input => 64);
    test_part_two!(test_input => 58);

    let real_input = include_str!("day18_input.txt");
    test_part_one!(real_input => 3498);
    test_part_two!(real_input => 2008);
}
