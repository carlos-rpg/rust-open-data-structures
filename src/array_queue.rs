pub struct ArrayQueue<T: Clone> {
    array: Vec<Option<T>>,
    first: Option<usize>,
    len: usize,
}

impl<T: Clone> ArrayQueue<T> {
    pub fn initialize(capacity: usize) -> Self {
        Self { array: vec![None; capacity], first: None, len: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_queue() {
        let queue: ArrayQueue<i32> = ArrayQueue::initialize(3);

        assert!(queue.array.iter().all(|x| x.is_none()));
        assert!(queue.first.is_none());
        assert_eq!(queue.array.len(), 3);
    }
}
