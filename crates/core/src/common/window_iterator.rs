pub struct WindowIterator<I, T, const SIZE: usize>
where
    I: Iterator<Item = T>,
    T: Copy + Default,
{
    iterator: I,
    items: [T; SIZE],
    current_idx: usize,
}

impl<I, T, const SIZE: usize> Iterator for WindowIterator<I, T, SIZE>
where
    I: Iterator<Item = T>,
    T: Copy + Default,
{
    type Item = [T; SIZE];

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_idx < SIZE {
            self.items[self.current_idx] = self.iterator.next()?;
            self.current_idx += 1;
        }
        self.current_idx = 0;
        Some(self.items)
    }
}

pub trait WindowIteratorExt: Iterator {
    fn window<const SIZE: usize>(self) -> WindowIterator<Self, Self::Item, SIZE>
    where
        Self::Item: Copy + Default,
        Self: Sized,
    {
        WindowIterator {
            iterator: self,
            items: [Self::Item::default(); SIZE],
            current_idx: 0,
        }
    }
}

impl<I: Iterator> WindowIteratorExt for I {}

#[test]
fn test() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut it = a.into_iter().window::<3>();
    assert_eq!(it.next(), Some([1, 2, 3]));
    assert_eq!(it.next(), Some([4, 5, 6]));
    assert_eq!(it.next(), None);

    let a = [1, 2, 3, 4, 5, 6, 7];
    let mut it = a.into_iter().window::<3>();
    assert_eq!(it.next(), Some([1, 2, 3]));
    assert_eq!(it.next(), Some([4, 5, 6]));
    assert_eq!(it.next(), None);
}
