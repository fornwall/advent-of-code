use crate::{
    common::{array_deque::ArrayDeque, array_stack::ArrayStack},
    input::{Input, on_error},
};

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut total_presses = 0;
    'line: for line in input.text.lines() {
        let mut buttons = ArrayStack::<64, u16>::new();
        let mut joltage_requirements = ArrayStack::<64, u16>::new();
        let mut parts = line.split(' ');
        let screen_part = parts.next().ok_or_else(on_error)?;
        let screen_part = screen_part
            .strip_prefix('[')
            .ok_or_else(on_error)?
            .strip_suffix(']')
            .ok_or_else(on_error)?;
        let target_state = screen_part.bytes().enumerate().fold(0, |acc, (idx, ch)| {
            acc | (u16::from(ch == b'#') * (1 << idx))
        });
        for part in parts {
            if let Some(part) = part.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
                let mut button_bits = 0;
                for num_str in part.split(',') {
                    button_bits |= 1 << num_str.parse::<u16>().map_err(|_| on_error())?;
                }
                buttons.push(button_bits)?;
            } else if let Some(part) = part.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                for num_str in part.split(',') {
                    joltage_requirements.push(num_str.parse::<u16>().map_err(|_| on_error())?)?;
                }
            }
        }

        if input.is_part_one() {
            let mut stack = ArrayDeque::<4096, /*bitset, num presses*/ (u16, u16)>::new();
            stack.push_back((0, 0))?;
            let mut best = [u16::MAX; 0b1_111_111_111 + 1];
            while let Some((press_state, num_presses)) = stack.pop_front() {
                for &button in buttons.slice() {
                    let new_state = (press_state ^ button) & 0b1_111_111_111;
                    let new_presses = num_presses + 1;
                    if new_state == target_state {
                        total_presses += u64::from(new_presses);
                        continue 'line;
                    } else if new_presses < best[new_state as usize] {
                        best[new_state as usize] = new_presses;
                        stack.push_back((new_state, new_presses))?;
                    }
                }
            }
            return Err(format!("No solution found: {line}"));
        } else {
            total_presses +=
                solve_linear_programming(buttons.slice(), joltage_requirements.slice()) as u64;
        }
    }
    Ok(total_presses)
}

// Based on python solution by u/RussellDash332:
// https://www.reddit.com/r/adventofcode/comments/1pity70/comment/nt988z4/?context=3
// Adapted to rust by Fadi88:
// https://github.com/Fadi88/AoC/blob/master/2025/days/day10/src/lib.rs
const INF: f64 = f64::INFINITY;
const EPS: f64 = 1e-9;

fn solve_linear_programming(buttons: &'_ [u16], joltage_requirements: &'_ [u16]) -> i64 {
    let num_goals = joltage_requirements.len();
    let num_buttons = buttons.len();

    let rows = 2 * num_goals + num_buttons;
    let cols = num_buttons + 1;

    let mut matrix = vec![vec![0.0; cols]; rows];

    for (j, row) in matrix.iter_mut().rev().take(num_buttons).enumerate() {
        row[j] = -1.0;
    }

    for (j, &mask) in buttons.iter().enumerate() {
        for i in 0..num_goals {
            if (mask >> i) & 1 == 1 {
                matrix[i][j] = 1.0;
                matrix[i + num_goals][j] = -1.0;
            }
        }
    }

    for i in 0..num_goals {
        let val = joltage_requirements[i] as f64;
        matrix[i][cols - 1] = val;
        matrix[i + num_goals][cols - 1] = -val;
    }

    let obj_coeffs = vec![1.0; num_buttons];
    solve_integer_linear_programming_branch_and_bound(matrix, &obj_coeffs)
}

fn simplex(lhs: &[Vec<f64>], c: &[f64]) -> (f64, Option<Vec<f64>>) {
    let m = lhs.len();
    let n = lhs[0].len() - 1;

    let mut n_indices: Vec<i32> = (0..n as i32).collect();
    n_indices.push(-1);

    let mut b_indices: Vec<i32> = (n as i32..(n + m) as i32).collect();
    let mut d = vec![vec![0.0; n + 2]; m + 2];

    for (d_row, lhs_row) in d.iter_mut().zip(lhs.iter()) {
        d_row[..=n].copy_from_slice(lhs_row);
        d_row[n + 1] = -1.0;
    }

    for row in d.iter_mut().take(m) {
        row.swap(n, n + 1);
    }

    d[m][..n].copy_from_slice(&c[..n]);
    d[m + 1][n] = 1.0;

    let pivot =
        |d: &mut Vec<Vec<f64>>, b_idx: &mut Vec<i32>, n_idx: &mut Vec<i32>, r: usize, s: usize| {
            let k = 1.0 / d[r][s];

            for i in 0..m + 2 {
                if i == r {
                    continue;
                }
                for j in 0..n + 2 {
                    if j != s {
                        d[i][j] -= d[r][j] * d[i][s] * k;
                    }
                }
            }

            for val in d[r].iter_mut() {
                *val *= k;
            }
            for row in d.iter_mut() {
                row[s] *= -k;
            }
            d[r][s] = k;

            std::mem::swap(&mut b_idx[r], &mut n_idx[s]);
        };

    let find =
        |d: &mut Vec<Vec<f64>>, b_idx: &mut Vec<i32>, n_idx: &mut Vec<i32>, p_idx: usize| -> bool {
            loop {
                let mut best_s = usize::MAX;
                let mut best_val = (INF, i32::MAX);

                for i in 0..=n {
                    if p_idx != 0 || n_idx[i] != -1 {
                        let val = d[m + p_idx][i];
                        let key = (val, n_idx[i]);
                        if best_s == usize::MAX
                            || key.0 < best_val.0 - EPS
                            || ((key.0 - best_val.0).abs() <= EPS && key.1 < best_val.1)
                        {
                            best_s = i;
                            best_val = key;
                        }
                    }
                }
                let s = best_s;

                if d[m + p_idx][s] > -EPS {
                    return true;
                }

                let mut best_r = usize::MAX;
                let mut best_r_key = (INF, i32::MAX);

                for i in 0..m {
                    if d[i][s] > EPS {
                        let ratio = d[i][n + 1] / d[i][s];
                        let key = (ratio, b_idx[i]);
                        if best_r == usize::MAX
                            || key.0 < best_r_key.0 - EPS
                            || ((key.0 - best_r_key.0).abs() <= EPS && key.1 < best_r_key.1)
                        {
                            best_r = i;
                            best_r_key = key;
                        }
                    }
                }
                let r = best_r;

                if r == usize::MAX {
                    return false;
                }

                pivot(d, b_idx, n_idx, r, s);
            }
        };

    let mut split_r = 0;
    let mut min_val = d[0][n + 1];
    for (i, row) in d.iter().enumerate().take(m).skip(1) {
        if row[n + 1] < min_val {
            min_val = row[n + 1];
            split_r = i;
        }
    }

    if d[split_r][n + 1] < -EPS {
        pivot(&mut d, &mut b_indices, &mut n_indices, split_r, n);
        if !find(&mut d, &mut b_indices, &mut n_indices, 1) || d[m + 1][n + 1] < -EPS {
            return (-INF, None);
        }
        for i in 0..m {
            if b_indices[i] == -1 {
                let mut best_s = 0;
                let mut best_key = (d[i][0], n_indices[0]);
                for j in 1..n {
                    let key = (d[i][j], n_indices[j]);
                    if key.0 < best_key.0 - EPS
                        || ((key.0 - best_key.0).abs() <= EPS && key.1 < best_key.1)
                    {
                        best_s = j;
                        best_key = key;
                    }
                }
                pivot(&mut d, &mut b_indices, &mut n_indices, i, best_s);
            }
        }
    }

    if find(&mut d, &mut b_indices, &mut n_indices, 0) {
        let mut x = vec![0.0; n];
        for i in 0..m {
            if b_indices[i] >= 0 && (b_indices[i] as usize) < n {
                x[b_indices[i] as usize] = d[i][n + 1];
            }
        }
        let mut sum_val = 0.0;
        for i in 0..n {
            sum_val += c[i] * x[i];
        }
        return (sum_val, Some(x));
    }

    (-INF, None)
}

fn solve_integer_linear_programming_branch_and_bound(
    initial_a: Vec<Vec<f64>>,
    obj_coeffs: &[f64],
) -> i64 {
    let mut best_val = INF;
    let mut stack = Vec::new();
    stack.push(initial_a);

    while let Some(current_a) = stack.pop() {
        let (val, x_opt) = simplex(&current_a, obj_coeffs);

        if val == -INF || val >= best_val - EPS {
            continue;
        }

        let mut fractional_idx = None;
        let mut fractional_val = 0.0;

        if let Some(x) = x_opt {
            for (i, &xv) in x.iter().enumerate() {
                if (xv - xv.round()).abs() > EPS {
                    fractional_idx = Some(i);
                    fractional_val = xv;
                    break;
                }
            }

            if let Some(idx) = fractional_idx {
                let floor_v = fractional_val.floor();
                let n_cols = current_a[0].len();

                let mut row1 = vec![0.0; n_cols];
                row1[idx] = 1.0;
                row1[n_cols - 1] = floor_v;
                let mut a1 = current_a.clone();
                a1.push(row1);
                stack.push(a1);

                let ceil_v = fractional_val.ceil();
                let mut row2 = vec![0.0; n_cols];
                row2[idx] = -1.0;
                row2[n_cols - 1] = -ceil_v;
                let mut a2 = current_a.clone();
                a2.push(row2);
                stack.push(a2);
            } else if val < best_val {
                best_val = val;
            }
        }
    }

    if best_val == INF {
        0
    } else {
        best_val.round() as i64
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two};

    let test_input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    test_part_one_no_allocations!(test_input => 7);
    test_part_two!(test_input => 33);

    let real_input = include_str!("day10_input.txt");
    test_part_one_no_allocations!(real_input => 524);
    test_part_two!(real_input => 21_696);
}
