pub struct TripleWindowIterator<I, T>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    iterator: I,
    first_item: Option<T>,
    second_item: Option<T>,
}

impl<I, T> Iterator for TripleWindowIterator<I, T>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    type Item = (T, T, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.first_item {
                Some(first_item) => match self.second_item {
                    Some(second_item) => {
                        let next_element = self.iterator.next()?;
                        self.first_item = Some(second_item);
                        self.second_item = Some(next_element);
                        return Some((first_item, second_item, next_element));
                    }
                    None => {
                        self.second_item = Some(self.iterator.next()?);
                    }
                },
                None => {
                    self.first_item = Some(self.iterator.next()?);
                }
            }
        }
    }
}

impl<I, T> ExactSizeIterator for TripleWindowIterator<I, T>
where
    I: ExactSizeIterator<Item = T>,
    T: Copy,
{
    fn len(&self) -> usize {
        let result = usize::from(self.first_item.is_some())
            + usize::from(self.second_item.is_some())
            + self.iterator.len();
        if result > 0 { result - 2 } else { result }
    }
}

pub trait TripleWindowIteratorExt: Iterator {
    fn triple_windows(self) -> TripleWindowIterator<Self, Self::Item>
    where
        Self::Item: Copy,
        Self: Sized,
    {
        TripleWindowIterator {
            iterator: self,
            first_item: None,
            second_item: None,
        }
    }
}

impl<I: Iterator> TripleWindowIteratorExt for I {}

#[test]
fn test() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut it = a.into_iter().triple_windows();
    assert_eq!(it.len(), 4);
    assert_eq!(it.next(), Some((1, 2, 3)));
    assert_eq!(it.len(), 3);
    assert_eq!(it.next(), Some((2, 3, 4)));
    assert_eq!(it.len(), 2);
    assert_eq!(it.next(), Some((3, 4, 5)));
    assert_eq!(it.len(), 1);
    assert_eq!(it.next(), Some((4, 5, 6)));
    assert_eq!(it.len(), 0);
    assert_eq!(it.next(), None);
    assert_eq!(it.len(), 0);
}
