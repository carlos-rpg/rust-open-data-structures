use std::usize;
use std::ops::Index;

pub struct ArrayQueue<T: Clone> {
    array: Vec<T>,
    head: usize,
    len: usize,
}

impl<T: Clone> ArrayQueue<T> {
    pub fn initialize() -> Self {
        Self { array: Vec::new(), head: 0, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.array.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, x: T) {
        if self.is_full() {
            self.reset_array();
            self.array.push(x);
        }
        else {
            let i_array = self.index_array(self.len());
            self.array[i_array] = x;
        }
        self.len += 1;
    }

    pub fn remove(&mut self) -> Option<T> {
        if !self.is_empty() {
            let item = self[0].clone();
            self.head = self.index_array(1);
            self.len -= 1;

            if self.array.capacity() >= 3 * self.len() {
                self.reset_array();
                self.array.truncate(2 * self.len());
                self.array.shrink_to_fit();
            }
            Some(item)
        }
        else {
            None
        }
    }

    fn index_array(&self, i: usize) -> usize {
        (self.head + i) % self.array.len()
    }

    fn reset_array(&mut self) {
        self.array.rotate_left(self.head);
        self.head = 0;
    }
}

impl<T: Clone> Index<usize> for ArrayQueue<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len() {
            panic!("Index out of bounds");
        }
        &self.array[self.index_array(index)]
    }
}

impl<T: Clone> From<Vec<T>> for ArrayQueue<T> {
    fn from(value: Vec<T>) -> Self {
        let value_len = value.len();
        Self { array: value, head: 0, len: value_len }
    }
}

impl<T: Clone> Into<Vec<T>> for ArrayQueue<T> {
    fn into(mut self) -> Vec<T> {
        self.reset_array();
        self.array[..self.len()].to_vec()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_queue() {
        let queue: ArrayQueue<i32> = ArrayQueue::initialize();

        assert!(queue.array.is_empty());
        assert_eq!(queue.head, 0);
        assert_eq!(queue.len, 0);
    }

    #[test]
    #[should_panic]
    fn index_empty_queue() {
        let queue: ArrayQueue<i32> = ArrayQueue::initialize();
        queue[0];
    }

    #[test]
    fn index_starts_at_zero() {
        let queue = ArrayQueue {
            array: vec!['a', 'b', 'c'],
            head: 0,
            len: 3,
        };
        assert_eq!(queue[0], 'a');
        assert_eq!(queue[1], 'b');
        assert_eq!(queue[2], 'c');
    }

    #[test]
    fn index_starts_at_nonzero() {
        let queue = ArrayQueue {
            array: vec!['a', 'b', 'c'],
            head: 1,
            len: 3,
        };
        assert_eq!(queue[0], 'b');
        assert_eq!(queue[1], 'c');
        assert_eq!(queue[2], 'a');
    }

    #[test]
    #[should_panic]
    fn index_out_of_queue_bounds() {
        let queue = ArrayQueue {
            array: vec!['a', 'b', 'c', 'w', 'k'],
            head: 2,
            len: 3,
        };
        queue[3];
    }

    #[test]
    fn index_when_array_and_queue_are_different_len() {
        let queue = ArrayQueue {
            array: vec!['a', 'b', 'c', 'w', 'k'],
            head: 3,
            len: 3,
        };
        assert_eq!(queue[0], 'w');
        assert_eq!(queue[1], 'k');
        assert_eq!(queue[2], 'a');
    }

    #[test]
    fn add_to_empty_queue() {
        let mut queue = ArrayQueue::initialize();
        queue.add(1);
        assert_eq!(queue.array, vec![1]);
        queue.add(2);
        assert_eq!(queue.array, vec![1, 2]);
        queue.add(3);
        assert_eq!(queue.array, vec![1, 2, 3]);
    }

    #[test]
    fn add_with_overwrite() {
        let mut queue = ArrayQueue {
            array: vec!['a', 'b', 'c', 'd'],
            head: 1,
            len: 2,
        };
        queue.add('e');
        assert_eq!(queue.array, vec!['a', 'b', 'c', 'e']);
        queue.add('f');
        assert_eq!(queue.array, vec!['f', 'b', 'c', 'e']);
    }

    #[test]
    fn add_with_reallocation() {
        let mut queue = ArrayQueue {
            array: vec!['a', 'b', 'c', 'd'],
            head: 2,
            len: 4,
        };
        queue.add('e');
        assert_eq!(queue.array, vec!['c', 'd', 'a', 'b', 'e']);
    }

    #[test]
    fn remove_until_empty() {
        let mut queue = ArrayQueue {
            array: vec![1, 2, 3],
            head: 0,
            len: 3,
        };
        assert_eq!(queue.remove(), Some(1));
        assert_eq!(queue.remove(), Some(2));
        assert_eq!(queue.remove(), Some(3));
        assert_eq!(queue.remove(), None);
        assert_eq!(queue.remove(), None);
    }

    #[test]
    fn remove_from_mid_array() {
        let mut queue = ArrayQueue {
            array: vec!['a', 'b', 'c'],
            head: 2,
            len: 2,
        };
        assert_eq!(queue.remove(), Some('c'));
        assert_eq!(queue.remove(), Some('a'));
        assert_eq!(queue.remove(), None);
    }

    #[test]
    fn remove_with_dealocation() {
        let mut queue = ArrayQueue {
            array: vec![1, 2, 3, 4, 5, 6],
            head: 4,
            len: 6,
        };
        assert_eq!(queue.array.capacity(), 6);
        queue.remove();
        queue.remove();
        queue.remove();
        queue.remove();
        assert_eq!(queue.array.capacity(), 4);
    }

    #[test]
    fn from_vector() {
        let queue = ArrayQueue::from(vec![true, false, true]);
        assert_eq!(queue.array, vec![true, false, true]);
        assert_eq!(queue.head, 0);
        assert_eq!(queue.len, 3);
    }

    #[test]
    fn to_vector() {
        let queue = ArrayQueue {
            array: vec![1, 2, 3],
            head: 2,
            len: 2,
        };
        let vector: Vec<i32> = queue.into();
        assert_eq!(vector, vec![3, 1]);
    }
}
