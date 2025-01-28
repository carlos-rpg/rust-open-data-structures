use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq)]
pub struct CircularVec<T> {
    storage: Vec<T>,
    head: usize,
}

impl<T> CircularVec<T> {
    pub fn new(storage: Vec<T>, head: usize) -> Self {
        if storage.is_empty() && head == 0 || head < storage.len() {
            Self { storage, head }
        }
        else {
            panic!("Incompatible `storage` length and `head` value");
        }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn shift_head(&mut self, n: isize) {
        let self_len: isize = match self.len().try_into() {
            Ok(val) => val,
            Err(_) => panic!("Unable to cast usize into isize"),
        };
        let n_equivalent = n.rem_euclid(self_len) as usize;
        self.head = self.circle_index(n_equivalent);
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
    fn new_circular_vector() {
        let empty: CircularVec<i32> = CircularVec::new(vec![], 0);
        assert_eq!(empty, CircularVec { storage: vec![], head: 0 });
        let one = CircularVec::new(vec!['a'], 0);
        assert_eq!(one, CircularVec { storage: vec!['a'], head: 0});
        let many = CircularVec::new(vec!['x', 'y'], 1);
        assert_eq!(many, CircularVec { storage: vec!['x', 'y'], head: 1});
    }

    #[test]
    #[should_panic]
    fn new_invalid() {
        let _invalid: CircularVec<i32> = CircularVec::new(vec![], 1);
    }

    #[test]
    #[should_panic]
    fn index_empty() {
        let cv: CircularVec<i32> = CircularVec { storage: Vec::new(), head: 0 };
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

    #[test]
    fn shift_head_right() {
        let mut cv = CircularVec {
            storage: vec!['a', 'b', 'c', 'd'],
            head: 1,
        };
        cv.shift_head(0);
        assert_eq!(cv.head, 1);
        cv.shift_head(1);
        assert_eq!(cv.head, 2);
        cv.shift_head(2);
        assert_eq!(cv.head, 0);
    }

    #[test]
    fn shift_head_left() {
        let mut cv = CircularVec {
            storage: vec!['a', 'b', 'c', 'd'],
            head: 2,
        };
        cv.shift_head(-1);
        assert_eq!(cv.head, 1);
        cv.shift_head(-2);
        assert_eq!(cv.head, 3);
    }

    #[test]
    #[should_panic]
    fn shift_head_empty() {
        let mut cv: CircularVec<i32> = CircularVec {
            storage: vec![],
            head: 0,
        };
        cv.shift_head(1);
    }
}