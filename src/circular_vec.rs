use std::ops::{Index, IndexMut};

pub struct CircularVec<T> {
    storage: Vec<T>,
    head: usize,
}

impl<T> CircularVec<T> {
    pub fn new() -> Self {
        Self { storage: Vec::new(), head: 0 }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    fn circle_index(&self, i: usize) -> usize {
        (self.head + i) % self.len()
    }
}

impl<T> Index<usize> for CircularVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.storage[self.circle_index(index)]
    }
}

impl<T> IndexMut<usize> for CircularVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let i = self.circle_index(index);
        &mut self.storage[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn index_empty() {
        let cv: CircularVec<i32> = CircularVec::new();
        cv[0];
    }

    #[test]
    fn index_single_element() {
        let cv = CircularVec { 
            storage: vec!['a'],
            head: 0,
        };
        assert_eq!(cv[0], 'a');
        assert_eq!(cv[1], 'a');
        assert_eq!(cv[20], 'a');
    }

    #[test]
    fn index_multiple_elements() {
        let cv = CircularVec { 
            storage: vec!['a', 'b', 'c'],
            head: 2,
        };
        assert_eq!(cv[0], 'c');
        assert_eq!(cv[1], 'a');
        assert_eq!(cv[2], 'b');
        assert_eq!(cv[4], 'a');
    }

    #[test]
    fn index_mutate_contents() {
        let mut cv = CircularVec { 
            storage: vec![3, 1, 2],
            head: 1,
        };
        cv[0] = 10;
        assert_eq!(cv.storage[1], 10);
        cv[1] = 20;
        assert_eq!(cv.storage[2], 20);
        cv[2] = 30;
        assert_eq!(cv.storage[0], 30);
    }
}