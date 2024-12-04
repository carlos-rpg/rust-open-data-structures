use std::usize;

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
}
