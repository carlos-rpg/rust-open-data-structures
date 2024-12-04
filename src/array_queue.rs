pub struct ArrayQueue<T: Clone> {
    array: Vec<Option<T>>,
    first: Option<usize>,
    len: usize,
}

impl<T: Clone> ArrayQueue<T> {
    pub fn initialize() -> Self {
        Self { array: Vec::new(), first: None, len: 0 }
    }

    pub fn initialize(capacity: usize) -> Self {
        Self { array: vec![None; capacity], first: None, len: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_queue() {
        let queue: ArrayQueue<i32> = ArrayQueue::initialize();

        assert!(queue.array.is_empty());
        assert!(queue.first.is_none());
        assert_eq!(queue.len, 0);
    }
}
