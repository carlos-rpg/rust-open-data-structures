//! A stack backed by an array as internal storage.
//! 
//! In this implementation, the element located in `loc = 0` correspond to the 
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

    /// Returns the number of elements stored in `self`.
    pub fn size(&self) -> usize {
        self.storage.len()
    }

    /// Returns a shared reference to the element in the position `loc` 
    /// inside `self`, `None` if `loc` is out of bounds of `self`.
    pub fn get(&self, i: usize) -> Option<&T> {
        self.storage.get(i)
    }

    /// Returns a mutable reference to the element in the position `loc` 
    /// inside `self`, `None` if `loc` is out of bounds of `self`.
    pub fn get_mut(&mut self, loc: usize) -> Option<&mut T> {
        self.storage.get_mut(loc)
    }

    /// Inserts `value` in the position `loc`, shifting up all other values above 
    /// `loc`. Returns `false` if `loc` if out of bounds of `self`, `true` 
    /// otherwise.
    /// 
    /// Notice that `loc = self.size()` is a valid location and is equivalent to 
    /// inserting at the top of the stack.
    pub fn add(&mut self, loc: usize, value: T) -> bool {
        if loc > self.size() {
            return false;
        }
        self.storage.insert(loc, value);
        true
    }

    /// Removes the value in `loc`, shifting down all other values above `loc`. 
    /// Returns the value in `loc` if it is not out of bounds, `None` otherwise.
    pub fn remove(&mut self, loc: usize) -> Option<T> {
        if loc >= self.size() {
            return None;
        }
        let value = self.storage.remove(loc);

        if self.is_too_large() {
            self.resize();
        }
        Some(value)
    }
    
    /// Inserts `value` at the top of the stack.
    pub fn push(&mut self, value: T) {
        self.storage.push(value);
    }

    /// Removes the value at the top of the stack. Returns the value if `self` 
    /// is not empty, otherwise `None`.
    pub fn pop(&mut self) -> Option<T> {
        let value = self.storage.pop()?;

        if self.is_too_large() {
            self.resize();
        }
        Some(value)
    }

    fn is_too_large(&self) -> bool {
        self.storage.capacity() >= 3 * self.size()
    }

    fn resize(&mut self) {
        self.storage.shrink_to(2 * self.size());
    }

    pub fn iter(&self) -> Iter<T> {
        self.storage.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
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
    fn get_empty_invalid_index() {
        let stack: ArrayStack<i32> = ArrayStack { 
            storage: vec![],
        };
        assert_eq!(stack.get(0), None);
        assert_eq!(stack.get(3), None);
    }

    #[test]
    fn get_non_empty_invalid_index() {
        let stack = ArrayStack { 
            storage: vec![10, 20, 30],
        };
        assert_eq!(stack.get(3), None);
        assert_eq!(stack.get(5), None);
    }

    #[test]
    fn get_valid_index() {
        let stack = ArrayStack {
            storage: vec![10, 20, 30],
        };
        assert_eq!(stack.get(0), Some(&10));
        assert_eq!(stack.get(1), Some(&20));
        assert_eq!(stack.get(2), Some(&30));
    }

    #[test]
    fn get_mut_empty_invalid_index() {
        let mut stack: ArrayStack<i32> = ArrayStack { 
            storage: vec![],
        };
        assert_eq!(stack.get_mut(0), None);
        assert_eq!(stack.get_mut(4), None);
    }

    #[test]
    fn get_mut_sets_value() {
        let mut stack = ArrayStack {
            storage: vec![10, 20, 30],
        };
        assert_eq!(stack.get_mut(0), Some(&mut 10));
        let elem_0 = stack.get_mut(0).unwrap();
        *elem_0 = 1;
        assert_eq!(stack.get_mut(0), Some(&mut 1));
    }

    #[test]
    fn add_from_empty_returns_outcome() {
        let mut stack = ArrayStack { 
            storage: vec![],
        };
        assert!(stack.add(0, 10));
        assert!(!stack.add(2, 10));
        assert!(stack.add(1, 20));
        assert!(!stack.add(3, 30));
        assert!(stack.add(1, 200));
    }

    #[test]
    fn add_from_empty_stores_values() {
        let mut stack = ArrayStack { 
            storage: vec![],
        };
        stack.add(0, 10);
        assert_eq!(stack.storage, vec![10]);
        stack.add(2, 10);
        assert_eq!(stack.storage, vec![10]);
        stack.add(1, 20);
        assert_eq!(stack.storage, vec![10, 20]);
        stack.add(3, 30);
        assert_eq!(stack.storage, vec![10, 20]);
        stack.add(1, 15);
        assert_eq!(stack.storage, vec![10, 15, 20]);
    }

    #[test]
    fn remove_returns_values() {
        let mut stack = ArrayStack { 
            storage: vec!['x', 'a', 'b', 'y', 'z'],
        };
        assert_eq!(stack.remove(4), Some('z'));
        assert_eq!(stack.remove(4), None);
        assert_eq!(stack.remove(0), Some('x'));
        assert_eq!(stack.remove(0), Some('a'));
        assert_eq!(stack.remove(1), Some('y'));
        assert_eq!(stack.remove(1), None);
    }

    #[test]
    fn push() {
        let mut stack = ArrayStack { 
            storage: vec![],
        };
        stack.push(0);
        assert_eq!(stack.storage, vec![0]);
        stack.push(1);
        assert_eq!(stack.storage, vec![0, 1]);
        stack.push(2);
        assert_eq!(stack.storage, vec![0, 1, 2]);
    }

    #[test]
    fn pop_returns_values() {
        let mut stack = ArrayStack { 
            storage: vec![10, 20],
        };
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.pop(), None);
    }
}
