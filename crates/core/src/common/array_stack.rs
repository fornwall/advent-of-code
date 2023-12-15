#[derive(Clone)]
pub struct ArrayStack<const MAX_SIZE: usize, H: Eq + Copy + Clone + Default> {
    pub elements: [H; MAX_SIZE],
    len: usize,
}

impl<const MAX_SIZE: usize, H: Eq + Copy + Clone + Default> ArrayStack<MAX_SIZE, H> {
    pub fn new() -> Self {
        Self {
            elements: [Default::default(); MAX_SIZE],
            len: 0,
        }
    }

    pub const fn new_const(initial: H) -> Self {
        Self {
            elements: [initial; MAX_SIZE],
            len: 0,
        }
    }

    pub fn push(&mut self, element: H) -> Result<(), String> {
        if self.len == MAX_SIZE {
            return Err("Too many elements pushed".to_string());
        }
        self.elements[self.len] = element;
        self.len += 1;
        Ok(())
    }

    pub fn slice(&self) -> &[H] {
        &self.elements[0..self.len]
    }

    pub fn slice_mut(&mut self) -> &mut [H] {
        &mut self.elements[0..self.len]
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    #[allow(clippy::unwrap_used)]
    pub fn pop_unwrap(&mut self) -> H {
        let result = self.elements[self.len - 1];
        self.len -= 1;
        result
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&H) -> bool,
    {
        let mut i = 0;
        while i < self.len {
            if f(&self.elements[i]) {
                i += 1;
            } else {
                self.elements.copy_within(i + 1..self.len, i);
                self.len -= 1;
            }
        }
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn retain() {
    let mut a = ArrayStack::<10, u32>::new();
    a.push(1).unwrap();
    a.push(2).unwrap();
    a.push(3).unwrap();
    a.push(4).unwrap();
    a.retain(|&b| b % 2 == 0);
    assert_eq!(2, a.len());
    assert_eq!(&[2, 4], a.slice());
}
