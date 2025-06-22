pub struct ArrayQueue<T> {
    storage: Vec<Option<T>>,
    head: usize,
    size: usize,
}

impl<T> ArrayQueue<T> {
    pub fn initialize() -> Self {
        Self { storage: vec![None], head: 0, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn add(&mut self, x: T) {
        if self.is_full() {
            self.grow(self.storage.len());
        }
        let index = self.storage_index(self.size);
        self.storage[index] = Some(x);
        self.size += 1;
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let element = std::mem::take(&mut self.storage[self.head]);
        self.head = self.storage_index(1);
        self.size -= 1;
        if self.is_too_large() {
            self.shrink(self.storage.len() / 2);
        }
        element
    }

    fn is_full(&self) -> bool {
        self.size() == self.storage.len()
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn is_too_large(&self) -> bool {
        self.storage.len() >= self.size() * 3 && self.storage.len() > 1
    }

    fn grow(&mut self, by: usize) {
        self.storage.rotate_left(self.head);
        self.head = 0;
        let nones = (0..by).map(|_| None);
        self.storage.extend(nones);
    }

    fn shrink(&mut self, to: usize) {
        self.storage.rotate_left(self.head);
        self.head = 0;
        self.storage.truncate(to);
        self.storage.shrink_to(to);
    }

    fn storage_index(&self, index: usize) -> usize {
        (self.head + index) % self.storage.len()
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { queue: self, index: 0 }
    }
}


impl<T> IntoIterator for ArrayQueue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(ArrayQueue<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let element = std::mem::take(&mut self.0.storage[self.0.head]);
        self.0.head = self.0.storage_index(1);
        element
    }
}

pub struct Iter<'a, T> {
    queue: &'a ArrayQueue<T>,
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.queue.storage.len() {
            return None;
        }
        let i = self.queue.storage_index(self.index);
        let item = self.queue.storage[i].as_ref();
        self.index += 1;
        item
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_has_size_zero() {
        let queue = ArrayQueue::<i32>::initialize();
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn initialize_returns_empty_queue() {
        let queue = ArrayQueue::<i32>::initialize();
        assert_eq!(queue.iter().count(), 0);
    }

    #[test]
    fn add_from_empty_updates_size() {
        let mut queue = ArrayQueue::initialize();
        queue.add(0);
        assert_eq!(queue.size(), 1);
        queue.add(1);
        assert_eq!(queue.size(), 2);
        queue.add(2);
        assert_eq!(queue.size(), 3);
    }

    #[test]
    fn add_from_empty_updates_queue() {
        let mut queue = ArrayQueue::initialize();
        queue.add(0);
        assert_eq!(queue.iter().collect::<Vec<&i32>>(), [&0]);
        queue.add(1);
        assert_eq!(queue.iter().collect::<Vec<&i32>>(), [&0, &1]);
        queue.add(2);
        assert_eq!(queue.iter().collect::<Vec<&i32>>(), [&0, &1, &2]);
    }

    #[test]
    fn remove_from_empty_keeps_size_zero() {
        let mut queue = ArrayQueue::<i32>::initialize();
        queue.remove();
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn remove_from_empty_returns_none() {
        let mut queue = ArrayQueue::<i32>::initialize();
        assert!(queue.remove().is_none());
    }

    #[test]
    fn remove_updates_size() {
        let mut queue = ArrayQueue::initialize();
        queue.add(0);
        queue.add(1);
        queue.add(2);

        queue.remove();
        assert_eq!(queue.size(), 2);
        queue.remove();
        assert_eq!(queue.size(), 1);
        queue.remove();
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn remove_returns_some() {
        let mut queue = ArrayQueue::initialize();
        queue.add(0);
        queue.add(1);
        queue.add(2);

        assert_eq!(queue.remove(), Some(0));
        assert_eq!(queue.remove(), Some(1));
        assert_eq!(queue.remove(), Some(2));
    }

    #[test]
    fn into_iter_from_empty_returns_queue() {
        let queue = ArrayQueue::<i32>::initialize();
        assert_eq!(queue.into_iter().collect::<Vec<i32>>(), []);
    }

    #[test]
    fn into_iter_returns_queue() {
        let mut queue = ArrayQueue::initialize();
        queue.add(0);
        queue.add(1);
        queue.add(2);
        assert_eq!(queue.into_iter().collect::<Vec<i32>>(), [0, 1, 2]);
    }
}
