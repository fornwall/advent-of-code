use crate::input::Input;

pub fn solve(input: &Input) -> Result<i64, String> {
    let iterations = input.part_values(1, 10);
    let input_multiplier = input.part_values(1, 811_589_153);

    let numbers = input
        .text
        .lines()
        .map(|line| Some(i64::from(line.parse::<i16>().ok()?) * input_multiplier))
        .collect::<Option<Vec<_>>>()
        .ok_or("Invalid input - lines are not numbers in the range [-32,768, 32_767]")?;

    if numbers.len() < 3 {
        return Err("Input must have at least three numbers".to_string());
    } else if numbers.len() > MAX_LENGTH {
        return Err(format!("Too many numbers - max {MAX_LENGTH}"));
    }

    let zero_idx = numbers
        .iter()
        .position(|&n| n == 0)
        .ok_or("No zero value in input")?;

    // Set up buckets containing indices, so that a rotation (that needs to lookup a position
    // from an index) does not need to search through and update numbers.len() entries.
    let bucket_size = (numbers.len() as f64).sqrt() as usize;
    let mut buckets = Vec::with_capacity(numbers.len() / bucket_size);
    for i in (0..numbers.len()).step_by(bucket_size) {
        let range_end = (i + bucket_size).min(numbers.len());
        buckets.push((i..range_end).collect::<Vec<_>>());
    }

    // Setup lookup table from an index to the bucket containing that index:
    let mut idx_to_bucket = (0..numbers.len())
        .map(|i| i / bucket_size)
        .collect::<Vec<_>>();

    for _ in 0..iterations {
        for idx_to_shift in 0..numbers.len() {
            // rem_euclid() is necessary as there are negative numbers.
            //
            // Subtract 1 from the array length as we are  removing the
            // element from the circle and inserting it into the
            // `numbers.len() - 1` sized list.
            //
            // Example: [1, 2, 3]
            // When rotating 3, we actually remove it and insert it into [1, 2]:
            //   (1) [1, 3, 2]
            //   (2) [1, 2, 3]
            //   (3) [1, 3, 2]
            let shift_at_idx =
                (numbers[idx_to_shift]).rem_euclid(numbers.len() as i64 - 1) as usize;

            // Look up old bucket and offset within that bucket:
            let old_bucket_containing_idx = idx_to_bucket[idx_to_shift];
            let old_offset_in_bucket = buckets[old_bucket_containing_idx]
                .iter()
                .position(|&n| n == idx_to_shift)
                .unwrap_or_default();

            // Remove the index from the old bucket:
            buckets[old_bucket_containing_idx].remove(old_offset_in_bucket);

            // Insert the index into the new bucket:
            let (new_bucket_containing_idx, new_offset_in_bucket) = find_bucket_and_offset(
                &buckets,
                old_bucket_containing_idx,
                old_offset_in_bucket + shift_at_idx,
            );
            buckets[new_bucket_containing_idx].insert(new_offset_in_bucket, idx_to_shift);
            idx_to_bucket[idx_to_shift] = new_bucket_containing_idx;
        }
    }

    let mut bucket_containing_number = idx_to_bucket[zero_idx];
    let mut number_offset_in_bucket = buckets[bucket_containing_number]
        .iter()
        .position(|&n| n == zero_idx)
        .unwrap_or_default();

    Ok(std::iter::from_fn(|| {
        (bucket_containing_number, number_offset_in_bucket) = find_bucket_and_offset(
            &buckets,
            bucket_containing_number,
            number_offset_in_bucket + 1000,
        );
        let number_idx = buckets[bucket_containing_number][number_offset_in_bucket];
        Some(numbers[number_idx])
    })
    .take(3)
    .sum())
}

fn find_bucket_and_offset(
    buckets: &[Vec<usize>],
    mut bucket: usize,
    mut offset_relative_to_bucket: usize,
) -> (usize, usize) {
    // Buckets are of different length initially as the input
    // might not split evenly into buckets, and later on we do
    // not balance buckets on insertion.
    while offset_relative_to_bucket >= buckets[bucket].len() {
        offset_relative_to_bucket -= buckets[bucket].len();
        bucket = (bucket + 1) % buckets.len();
    }
    (bucket, offset_relative_to_bucket)
}

const MAX_LENGTH: usize = 10_000;

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};

    let test_input = "1\n2\n-3\n3\n-2\n0\n4";
    test_part_one!(test_input => 3);
    test_part_two!(test_input => 1_623_178_306);

    let real_input = include_str!("day20_input.txt");
    test_part_one!(real_input => 7_225);
    test_part_two!(real_input => 548_634_267_428);

    let test_input = "0\n1\n2";
    test_part_one!(test_input => 3);
    let test_input = "0\n1";
    test_part_one_error!(test_input => "Input must have at least three numbers");
}
