pub struct ArrayStack<const MAX_SIZE: usize, H: Ord + Eq + Copy + Clone + Default> {
    elements: [H; MAX_SIZE],
    len: usize,
}

impl<const MAX_SIZE: usize, H: Ord + Eq + Copy + Clone + Default> ArrayStack<MAX_SIZE, H> {
    pub fn new() -> Self {
        Self {
            elements: [Default::default(); MAX_SIZE],
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
}
