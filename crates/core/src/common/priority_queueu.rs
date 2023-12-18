/// A binary heap implementation of a priority queue.
pub struct PriorityQueue<const MAX_SIZE: usize, H: Copy + Clone + Default + PartialOrd> {
    pub elements: [H; MAX_SIZE],
    len: usize,
}

impl<const MAX_SIZE: usize, H: Copy + Clone + Default + PartialOrd> PriorityQueue<MAX_SIZE, H> {
    pub fn new() -> Self {
        Self {
            elements: [Default::default(); MAX_SIZE],
            len: 0,
        }
    }

    pub fn push(&mut self, new_element: H) -> Result<(), String> {
        if self.len == MAX_SIZE {
            return Err("Too many elements pushed".to_string());
        }
        let mut current_index = self.len;
        self.len += 1;
        while current_index != 0 {
            let parent_index = (current_index - 1) / 2;
            if self.elements[parent_index] < new_element {
                break;
            }
            // newElement is < than parent element and should be placed higher up, so move element at parentIndex down.
            self.elements[current_index] = self.elements[parent_index];
            current_index = parent_index;
        }
        self.elements[current_index] = new_element;
        Ok(())
    }

    /// Remove the element with the minimum priority.
    pub fn pop(&mut self) -> Option<H> {
        if self.len == 0 {
            None
        } else {
            let min = self.elements[0];
            self.len -= 1;
            let tmp = self.elements[self.len];
            let mut hole = 0;
            loop {
                let mut child_index = hole * 2 + 1;
                if child_index >= self.len {
                    break;
                }
                // Check if right child exists and is smaller.
                let right_child_idx = child_index + 1;
                if right_child_idx < self.len
                    && self.elements[right_child_idx] < self.elements[child_index]
                {
                    child_index = right_child_idx;
                }
                if tmp < self.elements[child_index] {
                    break; // Smaller than smallest child, correct position.
                }
                self.elements[hole] = self.elements[child_index];
                hole = child_index;
            }
            self.elements[hole] = tmp;
            Some(min)
        }
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn priority_queue() {
    let mut q = PriorityQueue::<10, i32>::new();
    q.push(12).unwrap();
    q.push(4).unwrap();
    q.push(240).unwrap();
    q.push(2).unwrap();
    q.push(143).unwrap();
    q.push(244).unwrap();
    assert_eq!(q.len, 6);
    assert_eq!(q.pop(), Some(2));
    assert_eq!(q.len, 5);
    assert_eq!(q.pop(), Some(4));
    assert_eq!(q.len, 4);
    assert_eq!(q.pop(), Some(12));
    assert_eq!(q.len, 3);
    assert_eq!(q.pop(), Some(143));
    assert_eq!(q.len, 2);
    assert_eq!(q.pop(), Some(240));
    assert_eq!(q.len, 1);
    assert_eq!(q.pop(), Some(244));
    assert_eq!(q.len, 0);
    assert_eq!(q.pop(), None);
    assert_eq!(q.len, 0);
    assert_eq!(q.pop(), None);
}
