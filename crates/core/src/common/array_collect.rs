pub fn collect_array<'a, I: Iterator>(
    iterator: &mut I,
    array: &'a mut [<I as Iterator>::Item],
) -> Option<&'a [<I as Iterator>::Item]> {
    let mut idx = 0;
    for element in iterator {
        if idx == array.len() {
            return None;
        }
        array[idx] = element;
        idx += 1;
    }
    Some(&array[..idx])
}
