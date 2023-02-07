pub struct TupleWindowIterator<I, T>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    iterator: I,
    first_item: Option<T>,
}

impl<I, T> Iterator for TupleWindowIterator<I, T>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.first_item {
                Some(item) => {
                    let next_element = self.iterator.next()?;
                    self.first_item = Some(next_element);
                    return Some((item, next_element));
                }
                None => {
                    self.first_item = Some(self.iterator.next()?);
                }
            }
        }
    }
}

impl<I, T> ExactSizeIterator for TupleWindowIterator<I, T>
where
    I: ExactSizeIterator<Item = T>,
    T: Copy,
{
    fn len(&self) -> usize {
        let result = usize::from(self.first_item.is_some()) + self.iterator.len();
        if result > 0 {
            result - 1
        } else {
            result
        }
    }
}

pub trait TupleWindowIteratorExt: Iterator {
    fn tuple_windows(self) -> TupleWindowIterator<Self, Self::Item>
    where
        Self::Item: Copy,
        Self: Sized,
    {
        TupleWindowIterator {
            iterator: self,
            first_item: None,
        }
    }
}

impl<I: Iterator> TupleWindowIteratorExt for I {}

#[test]
fn test() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut it = a.into_iter().tuple_windows();
    assert_eq!(it.len(), 5);
    assert_eq!(it.next(), Some((1, 2)));
    assert_eq!(it.len(), 4);
    assert_eq!(it.next(), Some((2, 3)));
    assert_eq!(it.len(), 3);
    assert_eq!(it.next(), Some((3, 4)));
    assert_eq!(it.len(), 2);
    assert_eq!(it.next(), Some((4, 5)));
    assert_eq!(it.len(), 1);
    assert_eq!(it.next(), Some((5, 6)));
    assert_eq!(it.len(), 0);
    assert_eq!(it.next(), None);
    assert_eq!(it.len(), 0);
}
