use std::ops::{Index, IndexMut};

pub struct ArrayDeque<T> {
    array: Vec<T>,
    head: usize,
    len: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IndexOutOfBounds(usize),
}

impl<T: Clone> ArrayDeque<T> {
    pub fn initialize() -> Self {
        Self { array: Vec::new(), head: 0, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.array.len()
    }

    pub fn get(&self, i: usize) -> Result<T, Error> {
        if !self.is_out_of_index_bounds(i) {
            Ok(self[i].clone())
        }
        else {
            Err(Error::IndexOutOfBounds(i))
        }
    }

    pub fn set(&mut self, i: usize, x: T) -> Result<T, Error> {
        let y = self.get(i)?;
        self[i] = x;
        Ok(y)
    }

    pub fn add(&mut self, i: usize, x: T) {
        if self.is_out_of_insert_bounds(i) {
            panic!("Insertion index out of bounds");
        }
        if self.is_full() {
            self.reset_array();
            self.array.insert(i, x);
        }
        else {
            for j in (i + 1..=self.len()).rev() {
                let j_array = self.index_array(j);
                let jm1_array = self.index_array(j - 1);
                self.array[j_array] = self.array[jm1_array].clone();
            }
            let i_array = self.index_array(i);
            self.array[i_array] = x;
        }
        self.len += 1;
    }

    fn is_out_of_index_bounds(&self, i: usize) -> bool {
        i >= self.len()
    }

    fn is_out_of_insert_bounds(&self, i: usize) -> bool {
        i > self.len()
    }

    fn index_array(&self, i: usize) -> usize {
        (self.head + i) % self.array.len()
    }

    fn reset_array(&mut self) {
        self.array.rotate_left(self.head);
        self.head = 0;
    }
}

impl<T: Clone> Index<usize> for ArrayDeque<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if self.is_out_of_index_bounds(index) {
            panic!("Index out of bounds");
        }
        &self.array[self.index_array(index)]
    }
}

impl<T: Clone> IndexMut<usize> for ArrayDeque<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.is_out_of_index_bounds(index) {
            panic!("Index out of bounds");
        }
        let i = self.index_array(index);
        &mut self.array[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_get_and_set() {
        let mut queue = ArrayDeque { 
            array: vec!['a', 'b', 'c'],
            head: 2,
            len: 2,
        };
        assert_eq!(queue[0], 'c');
        assert_eq!(queue[1], 'a');
        queue[0] = 'x';
        queue[1] = 'y';
        assert_eq!(queue.array, vec!['y', 'b', 'x']);
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds() {
        let queue = ArrayDeque { 
            array: vec!['a', 'b', 'c'],
            head: 2,
            len: 2,
        };
        queue[2];
    }

    #[test]
    #[should_panic]
    fn index_mut_out_of_bounds() {
        let mut queue = ArrayDeque { 
            array: vec!['a', 'b', 'c'],
            head: 2,
            len: 2,
        };
        queue[2] = 'x';
    }

    #[test]
    fn get() {
        let queue = ArrayDeque { 
            array: vec!['a', 'b', 'c'],
            head: 2,
            len: 2,
        };
        assert_eq!(queue.get(0), Ok('c'));
        assert_eq!(queue.get(1), Ok('a'));
        assert_eq!(queue.get(2), Err(Error::IndexOutOfBounds(2)));
    }

    #[test]
    fn set_check_output() {
        let mut queue = ArrayDeque {
            array: vec![1, 2, 3],
            head: 1,
            len: 2,
        };
        assert_eq!(queue.set(0, 20), Ok(2));
        assert_eq!(queue.set(1, 30), Ok(3));
        assert_eq!(queue.set(2, 10), Err(Error::IndexOutOfBounds(2)));
    }

    #[test]
    fn set_check_array() {
        let mut queue = ArrayDeque {
            array: vec![1, 2, 3],
            head: 1,
            len: 2,
        };
        let _ = queue.set(0, 20);
        assert_eq!(queue.array, vec![1, 20, 3]);
        let _ = queue.set(1, 30);
        assert_eq!(queue.array, vec![1, 20, 30]);
        let _ = queue.set(2, 10);
        assert_eq!(queue.array, vec![1, 20, 30]);
    }

    #[test]
    fn add_as_push() {
        let mut queue = ArrayDeque {
            array: vec![],
            head: 0,
            len: 0,
        };
        queue.add(0, 'a');
        queue.add(1, 'b');
        queue.add(2, 'c');
        assert_eq!(queue.array, vec!['a', 'b', 'c']);
        assert_eq!(queue.head, 0);
        assert_eq!(queue.len, 3);
    }

    #[test]
    fn add_as_front_insertion() {
        let mut queue = ArrayDeque {
            array: vec![],
            head: 0,
            len: 0,
        };
        queue.add(0, 'a');
        queue.add(0, 'b');
        queue.add(0, 'c');
        assert_eq!(queue.array, vec!['c', 'b', 'a']);
        assert_eq!(queue.head, 0);
        assert_eq!(queue.len, 3);
    }

    #[test]
    fn add_within_array() {
        let mut queue = ArrayDeque {
            array: vec![1, 2, 3],
            head: 1,
            len: 2,
        };
        queue.add(0, 10);
        assert_eq!(queue.array, vec![3, 10, 2]);
        assert_eq!(queue.head, 1);
        assert_eq!(queue.len, 3);
        queue.add(1, 20);
        assert_eq!(queue.array, vec![10, 20, 2, 3]);
        assert_eq!(queue.head, 0);
        assert_eq!(queue.len, 4);
    }

    #[test]
    fn add_as_append() {
        let mut queue = ArrayDeque {
            array: vec![1, 2, 3],
            head: 1,
            len: 2,
        };
        queue.add(2, 10);
        assert_eq!(queue.array, vec![10, 2, 3]);
        assert_eq!(queue.head, 1);
        assert_eq!(queue.len, 3);
        queue.add(3, 20);
        assert_eq!(queue.array, vec![2, 3, 10, 20]);
        assert_eq!(queue.head, 0);
        assert_eq!(queue.len, 4);
        queue.add(4, 30);
        assert_eq!(queue.array, vec![2, 3, 10, 20, 30]);
        assert_eq!(queue.head, 0);
        assert_eq!(queue.len, 5);
    }

    #[test]
    #[should_panic]
    fn add_out_of_bounds() {
        let mut queue = ArrayDeque {
            array: vec![1, 2, 3],
            head: 1,
            len: 2,
        };
        queue.add(3, 10);
    }
}
