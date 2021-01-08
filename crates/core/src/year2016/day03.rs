use crate::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    fn is_triangle_possible(n1: u16, n2: u16, n3: u16) -> bool {
        n1 + n2 > n3 && n1 + n3 > n2 && n2 + n3 > n1
    }

    let mut possible_triangles = 0;

    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    let mut v3 = Vec::new();

    for (line_idx, line) in input.text.lines().enumerate() {
        let on_error = || format!("Line {}: Invalid input", line_idx + 1);

        let mut parts = line.split_ascii_whitespace();
        let n1 = parts.next().ok_or_else(on_error)?.parse::<u16>().unwrap();
        let n2 = parts.next().ok_or_else(on_error)?.parse::<u16>().unwrap();
        let n3 = parts.next().ok_or_else(on_error)?.parse::<u16>().unwrap();

        if input.is_part_one() {
            if is_triangle_possible(n1, n2, n3) {
                possible_triangles += 1;
            }
        } else {
            v1.push(n1);
            v2.push(n2);
            v3.push(n3);
            if v1.len() == 3 {
                if is_triangle_possible(v1[0], v1[1], v1[2]) {
                    possible_triangles += 1;
                }
                if is_triangle_possible(v2[0], v2[1], v2[2]) {
                    possible_triangles += 1;
                }
                if is_triangle_possible(v3[0], v3[1], v3[2]) {
                    possible_triangles += 1;
                }
                v1.clear();
                v2.clear();
                v3.clear();
            }
        }
    }
    Ok(possible_triangles)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day03_input.txt");
    test_part_one!(real_input => 1050);
    test_part_two!(real_input => 1921);
}
