pub struct ArrayDeque<T> {
    array: Vec<T>,
    first: usize,
    len: usize,
}

impl<T> ArrayDeque<T> {
    pub fn initialize() {
        Self { array: Vec::new(), first: 0, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
