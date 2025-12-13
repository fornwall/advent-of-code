use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    const MAX: i32 = 24;
    const SIZE: i32 = MAX + 2;

    const AIR_VALUE: u8 = 0;
    const LAVA_VALUE: u8 = 1;
    const WATER_VALUE: u8 = 2;

    let mut grid = [[[AIR_VALUE; SIZE as usize]; SIZE as usize]; SIZE as usize];

    input
        .text
        .lines()
        .try_for_each(|line| {
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
            grid[point.0 as usize][point.1 as usize][point.2 as usize] = LAVA_VALUE;
            Some(())
        })
        .ok_or_else(|| {
            format!("Invalid input - expected lines as 'N,N,N' where N is in [0,{MAX});")
        })?;

    if input.is_part_one() {
        Ok(grid
            .iter()
            .enumerate()
            .flat_map(move |(x, rx)| {
                rx.iter().enumerate().flat_map(move |(y, ry)| {
                    ry.iter().enumerate().filter_map(move |(z, &value)| {
                        (value == LAVA_VALUE).then_some(Some((x as i32, y as i32, z as i32)))
                    })
                })
            })
            .flatten()
            .map(|point| {
                adjacent(point)
                    .filter(|p| grid[(p.0) as usize][(p.1) as usize][(p.2) as usize] != LAVA_VALUE)
                    .count()
            })
            .sum())
    } else {
        let mut points_to_fill = Vec::with_capacity(MAX as usize * MAX as usize * MAX as usize);

        points_to_fill.push((0, 0, 0));
        let mut wet_sides = 0;

        while let Some(point_to_fill) = points_to_fill.pop() {
            for adjacent in adjacent(point_to_fill) {
                if (0..SIZE).contains(&adjacent.0)
                    && (0..SIZE).contains(&adjacent.1)
                    && (0..SIZE).contains(&adjacent.2)
                {
                    let cell = &mut grid[(adjacent.0) as usize][(adjacent.1) as usize]
                        [(adjacent.2) as usize];
                    match *cell {
                        LAVA_VALUE => {
                            wet_sides += 1;
                        }
                        AIR_VALUE => {
                            *cell = WATER_VALUE;
                            points_to_fill.push(adjacent);
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(wet_sides)
    }
}

fn adjacent(point: (i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32)> {
    [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ]
    .iter()
    .map(move |d| (d.0 + point.0, d.1 + point.1, d.2 + point.2))
}

#[test]
pub fn tests() {
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

#[cfg(feature = "count-allocations")]
#[test]
pub fn no_memory_allocations() {
    use crate::input::{test_part_one, test_part_two};
    let real_input = include_str!("day18_input.txt");
    let info = allocation_counter::measure(|| {
        test_part_one!(real_input => 3498);
        test_part_two!(real_input => 2008);
    });
    assert_eq!(info.count_total, 1);
}
