use std::usize;
use std::ops::Index;

pub struct ArrayQueue<T> {
    array: Vec<T>,
    first: usize,
    len: usize,
}

impl<T> ArrayQueue<T> {
    pub fn initialize() -> Self {
        Self { array: Vec::new(), first: usize::MAX, len: 0 }
    }

    fn starts_at_zero(&self) -> bool {
        self.first == 0
    }
}

impl<T> Index<usize> for ArrayQueue<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("Index out of bounds");
        }
        &self.array[(self.first + index) % self.len]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_queue() {
        let queue: ArrayQueue<i32> = ArrayQueue::initialize();

        assert!(queue.array.is_empty());
        assert_eq!(queue.first, usize::MAX);
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
}
