use std::usize;
use std::ops::Index;

pub struct ArrayQueue<T: Clone> {
    array: Vec<T>,
    first: usize,
    len: usize,
}

impl<T: Clone> ArrayQueue<T> {
    pub fn initialize() -> Self {
        Self { array: Vec::new(), first: 0, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn add(&mut self, x: T) {
        if self.len() == self.array.len() {
            self.rotate();
            self.array.push(x);
        }
        else {
            let target = self.array_index(self.len());
            self.array[target] = x;
        }
        self.len += 1;
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.len() > 0 {
            let item = self[0].clone();
            self.first = self.array_index(1);
            self.len -= 1;

            if self.array.capacity() >= 3 * self.len() {
                self.rotate();
                self.array.truncate(2 * self.len());
                self.array.shrink_to_fit();
            }
            Some(item)
        }
        else {
            None
        }
    }

    fn array_index(&self, queue_index: usize) -> usize {
        (self.first + queue_index) % self.array.len()
    }

    fn rotate(&mut self) {
        self.array.rotate_left(self.first);
        self.first = 0;
    }
}

impl<T: Clone> Index<usize> for ArrayQueue<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len() {
            panic!("Index out of bounds");
        }
        &self.array[self.array_index(index)]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_queue() {
        let queue: ArrayQueue<i32> = ArrayQueue::initialize();

        assert!(queue.array.is_empty());
        assert_eq!(queue.first, 0);
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
            first: 0,
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
            first: 1,
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
            first: 2,
            len: 3,
        };
        queue[3];
    }

    #[test]
    fn index_when_array_and_queue_are_different_len() {
        let queue = ArrayQueue {
            array: vec!['a', 'b', 'c', 'w', 'k'],
            first: 3,
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
            first: 1,
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
            first: 2,
            len: 4,
        };
        queue.add('e');
        assert_eq!(queue.array, vec!['c', 'd', 'a', 'b', 'e']);
    }

    #[test]
    fn remove_until_empty() {
        let mut queue = ArrayQueue {
            array: vec![1, 2, 3],
            first: 0,
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
            first: 2,
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
            first: 4,
            len: 6,
        };
        assert_eq!(queue.array.capacity(), 6);
        queue.remove();
        queue.remove();
        queue.remove();
        queue.remove();
        assert_eq!(queue.array.capacity(), 4);
    }
}
