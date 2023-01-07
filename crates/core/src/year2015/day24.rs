use crate::common::parser::parse_lines;
use crate::input::Input;

// arr[]  ---> Input Array
// data[] ---> Temporary array to store current combination
// start & end ---> Staring and Ending indexes in arr[]
// index  ---> Current index in data[]
// r ---> Size of a combination to be printed */
fn visit_subset_internal<F>(
    input: &[u8],
    output: &mut [u8],
    input_idx: usize,
    output_idx: usize,
    on_subset: &mut F,
) where
    F: FnMut(&[u8]),
{
    if output_idx == output.len() {
        on_subset(output);
        return;
    } else if input_idx >= input.len() {
        return;
    }

    // Include input[input_idx]:
    output[output_idx] = input[input_idx];
    visit_subset_internal(input, output, input_idx + 1, output_idx + 1, on_subset);
    // Skip input[input_idx]:
    visit_subset_internal(input, output, input_idx + 1, output_idx, on_subset);
}

fn visit_subsets<F>(input: &[u8], subset_size: usize, on_subsete: &mut F)
where
    F: FnMut(&[u8]),
{
    let mut output = vec![0; subset_size];
    visit_subset_internal(input, &mut output, 0, 0, on_subsete);
}

pub fn solve(input: &Input) -> Result<u128, String> {
    let weights = parse_lines::<u8>(input.text)?;

    let sum: u32 = weights.iter().map(|&w| u32::from(w)).sum();
    let group_weight = sum / input.part_values(3, 4);

    for subset_size in 1..weights.len() {
        let mut result = None;
        visit_subsets(&weights, subset_size, &mut |subset: &[u8]| {
            if subset.iter().map(|&w| u32::from(w)).sum::<u32>() == group_weight {
                let product = subset.iter().map(|&w| u128::from(w)).product();
                if product < result.unwrap_or(u128::MAX) {
                    result = Some(product);
                }
            }
        });
        if let Some(product) = result {
            return Ok(product);
        }
    }

    Err("No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day24_input.txt");
    test_part_one!(real_input => 10_723_906_903);
    test_part_two!(real_input => 74_850_409);
}
