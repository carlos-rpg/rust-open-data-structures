//! A stack backed by an array as internal storage.
//! 
//! In this implementation, the element located at index 0 correspond to the 
//! bottom of the stack. Iteration follows this convention as well so it will 
//! return elements from bottom to top.

use std::slice::{Iter, IterMut};


/// A stack backed by an array as internal storage.
pub struct ArrayStack<T> {
    storage: Vec<T>,
}


impl<T> ArrayStack<T> {
    /// Returns a new, empty `ArrayStack`.
    pub fn initialize() -> Self {
        ArrayStack { storage: Vec::new() }
    }

    /// Returns the number of elements stored.
    pub fn size(&self) -> usize {
        self.storage.len()
    }

    /// Returns a shared reference to the element in the position `i`, or `None` 
    /// if `i` is out of bounds.
    pub fn get(&self, i: usize) -> Option<&T> {
        self.storage.get(i)
    }

    /// Replaces the element in the position `i` with `x` and returns the 
    /// original element. Returns `None` if `i` is out of bounds.
    pub fn set(&mut self, i: usize, x: T) -> Option<T> {
        if self.is_out_of_indexing_bounds(i) {
            return None;
        }
        let y = std::mem::replace(&mut self.storage[i], x);
        Some(y)
    }

    /// Inserts `x` in the position `i`, shifting up all other values above 
    /// `i`. Panics if `i` if out of bounds.
    /// 
    /// Notice that `i = self.size()` is a valid location and is equivalent to 
    /// inserting at the top of the stack.
    pub fn add(&mut self, i: usize, x: T) {
        self.storage.insert(i, x);
    }

    /// Removes the value in `i`, shifting down all other values above `i`. 
    /// Returns the value in `i` if it is not out of bounds, `None` otherwise.
    pub fn remove(&mut self, i: usize) -> Option<T> {
        if self.is_out_of_indexing_bounds(i) {
            return None;
        }
        let y = self.storage.remove(i);

        if self.is_too_large() {
            self.resize();
        }
        Some(y)
    }

    fn is_out_of_indexing_bounds(&self, i: usize) -> bool {
        i >= self.size()
    }

    fn is_too_large(&self) -> bool {
        self.storage.capacity() >= 3 * self.size()
    }

    fn resize(&mut self) {
        self.storage.shrink_to(2 * self.size());
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.storage.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.storage.iter_mut()
    }
}


impl<T> IntoIterator for ArrayStack<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.storage.into_iter()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_has_size_zero() {
        let stack = ArrayStack::<i32>::initialize();
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn initialize_returns_empty_stack() {
        let stack = ArrayStack::<i32>::initialize();
        assert_eq!(stack.iter().count(), 0);
    }

    #[test]
    fn add_updates_size() {
        let mut stack = ArrayStack::initialize();
        stack.add(0, 'a');
        assert_eq!(stack.size(), 1);
        stack.add(0, 'b');
        assert_eq!(stack.size(), 2);
        stack.add(0, 'c');
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn add_head_updates_storage() {
        let mut stack = ArrayStack::initialize();
        stack.add(0, 'a');
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'a']);
        stack.add(1, 'b');
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'a', &'b']);
        stack.add(2, 'c');
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'a', &'b', &'c']);
    }

    #[test]
    fn add_tail_updates_storage() {
        let mut stack = ArrayStack::initialize();
        stack.add(0, 'a');
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'a']);
        stack.add(0, 'b');
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'b', &'a']);
        stack.add(0, 'c');
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'c', &'b', &'a']);
    }

    #[test]
    fn get_returns_shared_reference() {
        let mut stack = ArrayStack::initialize();
        stack.add(0, 'a');
        stack.add(1, 'b');
        stack.add(1, 'c');

        assert_eq!(stack.get(0), Some(&'a'));
        assert_eq!(stack.get(1), Some(&'c'));
        assert_eq!(stack.get(2), Some(&'b'));
    }

    #[test]
    fn get_out_of_bounds_returns_none() {
        let stack = ArrayStack::<i32>::initialize();
        assert!(stack.get(1).is_none());
        assert!(stack.get(2).is_none());
    }

    #[test]
    fn set_mutates_storage() {
        let mut stack = ArrayStack::initialize();
        stack.add(0, 'a');
        stack.add(1, 'b');
        stack.add(1, 'c');

        stack.set(0, 'x');
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'x', &'c', &'b']);
        stack.set(2, 'z');
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'x', &'c', &'z']);
    }

    #[test]
    fn set_returns_prior_element() {
        let mut stack = ArrayStack::initialize();
        stack.add(0, 'a');
        stack.add(1, 'b');
        stack.add(1, 'c');

        assert_eq!(stack.set(0, 'x'), Some('a'));
        assert_eq!(stack.set(2, 'z'), Some('b'));
    }

    #[test]
    fn set_out_of_bounds_returns_none() {
        let mut stack = ArrayStack::initialize();
        assert!(stack.set(1, 'a').is_none());
        assert!(stack.set(2, 'a').is_none());
    }

    #[test]
    fn remove_updates_size() {
        let mut stack = ArrayStack::initialize();
        stack.add(0, 'a');
        stack.add(1, 'b');
        stack.add(2, 'c');

        stack.remove(1);
        assert_eq!(stack.size(), 2);
        stack.remove(1);
        assert_eq!(stack.size(), 1);
        stack.remove(0);
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn remove_out_of_bounds_returns_none() {
        let mut stack = ArrayStack::<i32>::initialize();
        assert!(stack.remove(0).is_none());
        assert!(stack.remove(1).is_none());
    }

    #[test]
    fn remove_head_updates_storage() {
        let mut stack = ArrayStack::initialize();
        stack.add(0, 'a');
        stack.add(1, 'b');
        stack.add(2, 'c');
        stack.add(3, 'd');

        stack.remove(3);
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'a', &'b', &'c']);
        stack.remove(2);
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'a', &'b']);
        stack.remove(1);
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'a']);
    }

    #[test]
    fn remove_tail_updates_storage() {
        let mut stack = ArrayStack::initialize();
        stack.add(0, 'a');
        stack.add(1, 'b');
        stack.add(2, 'c');
        stack.add(3, 'd');

        stack.remove(0);
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'b', &'c', &'d']);
        stack.remove(0);
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'c', &'d']);
        stack.remove(0);
        assert_eq!(stack.iter().collect::<Vec<&char>>(), [&'d']);
    }

    #[test]
    fn remove_returns_value() {
        let mut stack = ArrayStack::initialize();
        stack.add(0, 'a');
        stack.add(1, 'b');
        stack.add(2, 'c');
        stack.add(3, 'd');

        assert_eq!(stack.remove(1), Some('b'));
        assert_eq!(stack.remove(2), Some('d'));
        assert_eq!(stack.remove(0), Some('a'));
    }
}
