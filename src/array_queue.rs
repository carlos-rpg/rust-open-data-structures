use std::usize;
use std::ops::Index;

pub struct ArrayQueue<T> {
    array: Vec<T>,
    first: usize,
    len: usize,
}

impl<T> ArrayQueue<T> {
    pub fn initialize() -> Self {
        Self { array: Vec::new(), first: 0, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn add(&mut self, x: T) {
        if self.len() == self.array.len() {
            self.array.rotate_left(self.first);
            self.first = 0;
            self.array.push(x);
        }
        else {
            let target = (self.first + self.len()) % self.array.len();
            self.array[target] = x;
        }
        self.len += 1;
    }

    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len() {
            panic!("Index out of bounds");
        }
        &self.array[(self.first + index) % self.array.len()]
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
}
