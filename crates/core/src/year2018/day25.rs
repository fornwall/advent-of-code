use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    let mut points: Vec<(i32, i32, i32, i32, usize)> = input
        .text
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let line_number = i + 1;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 4 {
                return Err(format!(
                    "Invalid input at line {line_number} - not 4 comma-separated values"
                ));
            }
            let error = |e| format!("Invalid input at line {line_number}: {e}");
            Ok((
                parts[0].parse::<i32>().map_err(error)?,
                parts[1].parse::<i32>().map_err(error)?,
                parts[2].parse::<i32>().map_err(error)?,
                parts[3].parse::<i32>().map_err(error)?,
                i,
            ))
        })
        .collect::<Result<_, _>>()?;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let a = points[i];
            let b = points[j];

            if ((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()) <= 3
            {
                for p in points.iter_mut().filter(|p| p.4 == b.4) {
                    p.4 = a.4;
                }
            }
        }
    }

    points.sort_by(|a, b| a.4.cmp(&b.4));
    points.dedup_by(|a, b| a.4 == b.4);
    Ok(points.len())
}

#[test]
fn tests() {
    use crate::input::test_part_one;
    test_part_one!("0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0" => 2);
    test_part_one!("-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0" => 4);
    test_part_one!("1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2" => 3);
    test_part_one!("1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2" => 8);

    let input = include_str!("day25_input.txt");
    test_part_one!(input => 399);
}
