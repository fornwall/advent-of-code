pub struct ChunksIterator<I, T, const SIZE: usize>
where
    I: Iterator<Item = T>,
    T: Copy + Default,
{
    iterator: I,
    items: [T; SIZE],
    current_idx: usize,
}

impl<I, T, const SIZE: usize> Iterator for ChunksIterator<I, T, SIZE>
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

impl<I, T, const SIZE: usize> ExactSizeIterator for ChunksIterator<I, T, SIZE>
where
    I: ExactSizeIterator<Item = T>,
    T: Copy + Default,
{
    fn len(&self) -> usize {
        (self.current_idx + self.iterator.len()) / SIZE
    }
}

pub trait ChunkIteratorExt: Iterator {
    fn chunks_exact<const SIZE: usize>(self) -> ChunksIterator<Self, Self::Item, SIZE>
    where
        Self::Item: Copy + Default,
        Self: Sized,
    {
        ChunksIterator {
            iterator: self,
            items: [Self::Item::default(); SIZE],
            current_idx: 0,
        }
    }
}

impl<I: Iterator> ChunkIteratorExt for I {}

#[test]
fn test() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut it = a.into_iter().chunks_exact::<3>();
    assert_eq!(2, it.len());
    assert_eq!(it.next(), Some([1, 2, 3]));
    assert_eq!(1, it.len());
    assert_eq!(it.next(), Some([4, 5, 6]));
    assert_eq!(0, it.len());
    assert_eq!(it.next(), None);
    assert_eq!(0, it.len());

    let a = [1, 2, 3, 4, 5, 6, 7];
    let mut it = a.into_iter().chunks_exact::<3>();
    assert_eq!(2, it.len());
    assert_eq!(it.next(), Some([1, 2, 3]));
    assert_eq!(1, it.len());
    assert_eq!(it.next(), Some([4, 5, 6]));
    assert_eq!(0, it.len());
    assert_eq!(it.next(), None);
    assert_eq!(0, it.len());
}
