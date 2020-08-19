/// Generate all permutations of a sequence using Heap's algorithm.
pub fn all_permutations<F, T>(sequence: &mut Vec<T>, on_permutation: &mut F)
where
    F: FnMut(&Vec<T>),
{
    let size = sequence.len();
    all_permutations_internal(sequence, size, size, on_permutation);
}

fn all_permutations_internal<F, T>(
    sequence: &mut Vec<T>,
    size: usize,
    n: usize,
    on_permutation: &mut F,
) where
    F: FnMut(&Vec<T>),
{
    if size == 1 {
        on_permutation(sequence);
        return;
    }

    for i in 0..size {
        all_permutations_internal(sequence, size - 1, n, on_permutation);

        if size % 2 == 1 {
            // If size is odd, swap first and last element.
            sequence.swap(0, size - 1);
        } else {
            // If size is even, swap ith and last element.
            sequence.swap(i, size - 1);
        }
    }
}
