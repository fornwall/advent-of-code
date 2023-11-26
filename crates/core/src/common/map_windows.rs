use std::array;
use std::collections::VecDeque;

pub struct MapWindows<I: Iterator, F, T, const N: usize>
where
    F: FnMut([&I::Item; N]) -> T,
{
    iter: I,
    f: F,
    buf: VecDeque<I::Item>,
}

impl<I: Iterator, F, T, const N: usize> MapWindows<I, F, T, N>
where
    F: FnMut([&I::Item; N]) -> T,
{
    fn new(mut iter: I, f: F) -> Self {
        let buf: VecDeque<_> = iter.by_ref().take(N - 1).collect();
        Self { iter, f, buf }
    }
}

impl<I: Iterator, F, T, const N: usize> Iterator for MapWindows<I, F, T, N>
where
    F: FnMut([&I::Item; N]) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|next| {
            self.buf.push_back(next);
            let res = (self.f)(array::from_fn(|i| &self.buf[i]));
            self.buf.pop_front();
            res
        })
    }
}

pub trait MapWindowsIterator: Iterator {
    // https://github.com/rust-lang/rust/issues/87155 for stabilization.
    fn map_windows_stable<T, F, const N: usize>(self, f: F) -> MapWindows<Self, F, T, N>
    where
        Self: Sized,
        F: FnMut([&Self::Item; N]) -> T,
    {
        MapWindows::new(self, f)
    }
}

impl<I: Iterator> MapWindowsIterator for I {}

#[test]
#[allow(unstable_name_collisions)]
fn test_iterator() {
    let v = [1, 2, 3, 4]
        .iter()
        .map_windows_stable(|[a, b]| (**a, **b))
        .collect::<Vec<_>>();
    assert_eq!(vec![(1, 2), (2, 3), (3, 4)], v);

    let v = [1, 2, 3, 4]
        .iter()
        .map_windows_stable(|[a, b, c]| (**a, **b, **c))
        .collect::<Vec<_>>();
    assert_eq!(vec![(1, 2, 3), (2, 3, 4)], v);
    let v = [1, 2, 3, 4]
        .iter()
        .map_windows_stable(|[a, b, c, d, e]| (**a, **b, **c, **d, **e))
        .next();
    assert_eq!(None, v);
    let v = [1, 2, 3]
        .iter()
        .map_windows_stable(|[a, b, c, d, e]| (**a, **b, **c, **d, **e))
        .next();
    assert_eq!(None, v);
}
