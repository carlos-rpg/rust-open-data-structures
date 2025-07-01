pub struct ArrayDeque<T> {
    storage: Vec<Option<T>>,
    head: usize,
    size: usize,
}

impl<T: std::fmt::Debug> ArrayDeque<T> {
    pub fn initialize() -> Self {
        Self { storage: vec![None], head: 0, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if self.is_out_of_indexing_bounds(i) {
            return None;
        }
        self.storage[self.storage_index(i)].as_ref()
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        if self.is_out_of_indexing_bounds(i) {
            return None;
        }
        let index =self.storage_index(i);
        self.storage[index].as_mut()
    }

    pub fn add(&mut self, i: usize, x: T) {
        if self.is_out_of_inserting_bounds(i) {
            panic!("Index out of bounds: {i}");
        }
        if self.is_full() {
            self.grow(self.storage.len());
        }
        if i < self.size() / 2 {
            self.shift_head_back();
            for j in 0..i {
                let a = self.storage_index(j);
                let b = self.storage_index(j + 1);
                self.storage.swap(a, b);
            }
        } else {
            for j in (i..self.size()).rev() {
                let a = self.storage_index(j);
                let b = self.storage_index(j + 1);
                self.storage.swap(a, b);
            }
        }
        let j = self.storage_index(i);
        self.storage[j] = Some(x);
        self.size += 1;
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        if self.is_out_of_indexing_bounds(i) {
            return None;
        }
        let j = self.storage_index(i);
        let element = std::mem::take(&mut self.storage[j]);

        if i < self.size() / 2 {
            for j in (0..i).rev() {
                let a = self.storage_index(j);
                let b = self.storage_index(j + 1);
                self.storage.swap(a, b);
            }
            self.shift_head_forth();
        } else {
            for j in i..self.size() - 1 {
                let a = self.storage_index(j);
                let b = self.storage_index(j + 1);
                self.storage.swap(a, b);
            }
        }
        self.size -= 1;
        if self.is_too_large() {
            self.shrink(self.storage.len() / 2);
        }
        element
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { deque: self, index: 0 }
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

    fn is_full(&self) -> bool {
        self.size() == self.storage.len()
    }

    fn is_out_of_indexing_bounds(&self, i: usize) -> bool {
        i >= self.size()
    }

    fn is_out_of_inserting_bounds(&self, i: usize) -> bool {
        i > self.size()
    }

    fn shift_head_back(&mut self) {
        self.head = if self.head > 0 {
            self.head - 1
        } else {
            self.storage.len() - 1
        }
    }

    fn shift_head_forth(&mut self) {
        self.head = self.storage_index(1);
    }

    fn is_too_large(&self) -> bool {
        self.storage.len() >= self.size() * 3 && self.storage.len() > 1
    }
}


pub struct Iter<'a, T> {
    deque: &'a ArrayDeque<T>,
    index: usize,
}

impl<'a, T: std::fmt::Debug> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.deque.storage.len() {
            return None;
        }
        let i = self.deque.storage_index(self.index);
        let item = self.deque.storage[i].as_ref();
        self.index += 1;
        item
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_has_size_zero() {
        let deque = ArrayDeque::<i32>::initialize();
        assert_eq!(deque.size(), 0);
    }

    #[test]
    fn initialize_returns_empty_deque() {
        let deque = ArrayDeque::<i32>::initialize();
        assert_eq!(deque.iter().count(), 0);
    }

    #[test]
    #[should_panic]
    fn add_empty_out_of_bounds_panics() {
        let mut deque = ArrayDeque::initialize();
        deque.add(1, 0);
    }

    #[test]
    fn add_updates_size() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 0);
        assert_eq!(deque.size(), 1);
        deque.add(1, 1);
        assert_eq!(deque.size(), 2);
        deque.add(0, 2);
        assert_eq!(deque.size(), 3);
        deque.add(2, 3);
        assert_eq!(deque.size(), 4);
    }

    #[test]
    fn add_tail_updates_storage() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 0);
        assert_eq!(deque.iter().collect::<Vec<&i32>>(), [&0]);
        deque.add(1, 1);
        assert_eq!(deque.iter().collect::<Vec<&i32>>(), [&0, &1]);
        deque.add(2, 2);
        assert_eq!(deque.iter().collect::<Vec<&i32>>(), [&0, &1, &2]);
    }

    #[test]
    fn add_head_updates_storage() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 0);
        assert_eq!(deque.iter().collect::<Vec<&i32>>(), [&0]);
        deque.add(0, 1);
        assert_eq!(deque.iter().collect::<Vec<&i32>>(), [&1, &0]);
        deque.add(0, 2);
        assert_eq!(deque.iter().collect::<Vec<&i32>>(), [&2, &1, &0]);
    }

    #[test]
    #[should_panic]
    fn add_non_empty_out_of_bounds_panics() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 'a');
        deque.add(1, 'b');
        deque.add(0, 'c');
        
        deque.add(10, 'x');
    }

    #[test]
    fn add_mid_updates_storage() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 'a');
        assert_eq!(deque.iter().collect::<Vec<&char>>(), [&'a']);
        deque.add(1, 'b');
        assert_eq!(deque.iter().collect::<Vec<&char>>(), [&'a', &'b']);
        deque.add(0, 'c');
        assert_eq!(deque.iter().collect::<Vec<&char>>(), [&'c', &'a', &'b']);
        deque.add(1, 'd');
        assert_eq!(deque.iter().collect::<Vec<&char>>(), [&'c', &'d', &'a', &'b']);
        deque.add(2, 'e');
        assert_eq!(deque.iter().collect::<Vec<&char>>(), [&'c', &'d', &'e', &'a', &'b']);
    }

    #[test]
    fn get_returns_some_reference() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 'a');
        deque.add(0, 'b');
        deque.add(2, 'c');

        assert_eq!(deque.get(0), Some(&'b'));
        assert_eq!(deque.get(1), Some(&'a'));
        assert_eq!(deque.get(2), Some(&'c'));
    }

    #[test]
    fn get_out_of_bounds_returns_none() {
        let mut deque = ArrayDeque::initialize();
        assert!(deque.get(0).is_none());
        deque.add(0, 'a');
        assert!(deque.get(1).is_none());
        deque.add(0, 'b');
        assert!(deque.get(2).is_none());
        deque.add(2, 'c');
        assert!(deque.get(3).is_none());
    }

    #[test]
    fn get_mut_returns_some_mutable_reference() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 'a');
        deque.add(0, 'b');
        deque.add(2, 'c');

        assert_eq!(deque.get_mut(0), Some(&mut 'b'));
        assert_eq!(deque.get_mut(1), Some(&mut 'a'));
        assert_eq!(deque.get_mut(2), Some(&mut 'c'));
    }

    #[test]
    fn get_mut_out_of_bounds_returns_none() {
        let mut deque = ArrayDeque::initialize();
        assert!(deque.get_mut(0).is_none());
        deque.add(0, 'a');
        assert!(deque.get_mut(1).is_none());
        deque.add(0, 'b');
        assert!(deque.get_mut(2).is_none());
        deque.add(2, 'c');
        assert!(deque.get_mut(3).is_none());
    }

    #[test]
    fn remove_from_empty_returns_none() {
        let mut deque = ArrayDeque::<i32>::initialize();
        assert!(deque.remove(0).is_none());
    }

    #[test]
    fn remove_from_empty_leaves_empty_storage() {
        let mut deque = ArrayDeque::<i32>::initialize();
        deque.remove(0);
        assert_eq!(deque.iter().count(), 0);
    }

    #[test]
    fn remove_updates_size() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 'a');
        deque.add(1, 'b');
        deque.add(2, 'c');

        deque.remove(1);
        assert_eq!(deque.size(), 2);
        deque.remove(1);
        assert_eq!(deque.size(), 1);
        deque.remove(0);
        assert_eq!(deque.size(), 0);
    }

    #[test]
    fn remove_tail_updates_storage() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 'a');
        deque.add(1, 'b');
        deque.add(2, 'c');

        deque.remove(2);
        assert_eq!(deque.iter().collect::<Vec<&char>>(), [&'a', &'b']);
        deque.remove(1);
        assert_eq!(deque.iter().collect::<Vec<&char>>(), [&'a']);
        deque.remove(0);
        assert_eq!(deque.iter().count(), 0);
    }

    #[test]
    fn remove_tail_returns_some() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 'a');
        deque.add(1, 'b');
        deque.add(2, 'c');

        assert_eq!(deque.remove(2), Some('c'));
        assert_eq!(deque.remove(1), Some('b'));
        assert_eq!(deque.remove(0), Some('a'));
    }

    #[test]
    fn remove_head_updates_storage() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 'a');
        deque.add(1, 'b');
        deque.add(2, 'c');

        deque.remove(0);
        assert_eq!(deque.iter().collect::<Vec<&char>>(), [&'b', &'c']);
        deque.remove(0);
        assert_eq!(deque.iter().collect::<Vec<&char>>(), [&'c']);
        deque.remove(0);
        assert_eq!(deque.iter().count(), 0);
    }

    #[test]
    fn remove_head_returns_some() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 'a');
        deque.add(1, 'b');
        deque.add(2, 'c');

        assert_eq!(deque.remove(0), Some('a'));
        assert_eq!(deque.remove(0), Some('b'));
        assert_eq!(deque.remove(0), Some('c'));
    }

    #[test]
    fn remove_mid_updates_storage() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 'a');
        deque.add(1, 'b');
        deque.add(2, 'c');
        deque.add(3, 'd');
        deque.add(4, 'e');

        deque.remove(2);
        assert_eq!(deque.iter().collect::<Vec<&char>>(), [&'a', &'b', &'d', &'e']);
        deque.remove(2);
        assert_eq!(deque.iter().collect::<Vec<&char>>(), [&'a', &'b', &'e']);
        deque.remove(1);
        assert_eq!(deque.iter().collect::<Vec<&char>>(), [&'a', &'e']);
    }

    #[test]
    fn remove_mid_returns_some() {
        let mut deque = ArrayDeque::initialize();
        deque.add(0, 'a');
        deque.add(1, 'b');
        deque.add(2, 'c');
        deque.add(3, 'd');
        deque.add(4, 'e');

        assert_eq!(deque.remove(2), Some('c'));
        assert_eq!(deque.remove(2), Some('d'));
        assert_eq!(deque.remove(1), Some('b'));
    }
}
