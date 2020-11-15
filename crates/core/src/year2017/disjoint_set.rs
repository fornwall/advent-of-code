use std::collections::HashSet;

pub struct DisjointSet {
    elements: Vec<i32>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        Self {
            elements: vec![-1; size],
        }
    }

    pub fn find(&mut self, index: usize) -> usize {
        // Find root:
        let mut root_index = index;
        loop {
            let value = self.elements[root_index];
            if value < 0 {
                break;
            } else {
                root_index = value as usize;
            }
        }

        // Compress paths:
        let mut new_index = index;
        loop {
            let value = self.elements[new_index];
            if value < 0 {
                break;
            } else {
                self.elements[new_index] = root_index as i32;
                new_index = value as usize;
            }
        }

        root_index
    }

    pub fn join(&mut self, i: usize, j: usize) {
        let root1 = self.find(i);
        let root2 = self.find(j);

        if root1 == root2 {
            return;
        }

        let r1 = self.elements[root1];
        let r2 = self.elements[root2];

        // Join smaller tree with bigger:
        if r1 < r2 {
            self.elements[root1] += r2;
            self.elements[root2] = root1 as i32;
        } else {
            self.elements[root2] += r1;
            self.elements[root1] = root2 as i32;
        }
    }

    pub fn size(&mut self, i: usize) -> usize {
        let root = self.find(i);
        -self.elements[root] as usize
    }

    pub fn num_groups(&mut self) -> usize {
        let mut set = HashSet::new();
        for i in 0..self.elements.len() {
            set.insert(self.find(i));
        }
        set.len()
    }
}

#[test]
fn test_disjoint_set() {
    let mut set = DisjointSet::new(10);
    assert_eq!(1, set.size(0));
    assert_eq!(10, set.num_groups());

    set.join(0, 1);
    assert_eq!(2, set.size(0));
    assert_eq!(2, set.size(1));
    assert_eq!(9, set.num_groups());

    set.join(2, 3);
    assert_eq!(2, set.size(0));
    assert_eq!(2, set.size(3));
    assert_eq!(8, set.num_groups());

    set.join(1, 3);
    assert_eq!(4, set.size(0));
    assert_eq!(4, set.size(3));
    assert_eq!(7, set.num_groups());
}
