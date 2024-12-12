use std::ops::Index;

pub struct ArrayStack<T: Clone> {
    array: Vec<T>,
}

#[derive(PartialEq, Debug)]
pub struct IndexOutOfBounds;

impl<T: Clone> ArrayStack<T> {
    pub fn initialize() -> Self {
        ArrayStack { array: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn get(&self, i: usize) -> Option<T> {
        Some(self.array.get(i)?.clone())
    }

    pub fn set(&mut self, i: usize, x: T) -> Option<T> {
        let y = self.get(i)?;
        self.array[i] = x;
        Some(y)
    }

    pub fn add(&mut self, i: usize, x: T) -> Result<(), IndexOutOfBounds> {
        if i <= self.len() {
            self.array.insert(i, x);
            Ok(())
        }
        else {
            Err(IndexOutOfBounds)
        }
    }

    pub fn remove(&mut self, i: usize) -> Result<T, IndexOutOfBounds> {
        if i < self.len() {
            let y = self.array.remove(i);
            self.try_shrink();
            Ok(y)
        }
        else {
            Err(IndexOutOfBounds)
        }
    }
    
    pub fn push(&mut self, x: T) {
        self.array.push(x);
    }

    pub fn pop(&mut self) -> Option<T> {
        let y = self.array.pop()?;
        self.try_shrink();
        Some(y)
    }

    fn try_shrink(&mut self) -> bool {
        if self.array.capacity() >= 3 * self.len() {
            self.array.shrink_to(2 * self.len());
            true
        }
        else {
            false
        }
    }
}

impl<T: Clone> Index<usize> for ArrayStack<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_values() {
        let stack = ArrayStack { 
            array: vec![10, 20, 30],
        };
        assert_eq!(stack.get(1), Some(20));
        assert_eq!(stack.get(3), None);
    }

    #[test]
    fn set_values() {
        let mut stack = ArrayStack { 
            array: vec![10, 20, 30],
        };
        assert_eq!(stack.set(0, 1), Some(10));
        assert_eq!(stack.array, vec![1, 20, 30]);
        assert_eq!(stack.set(3, 4), None);
        assert_eq!(stack.array, vec![1, 20, 30]);
    }

    #[test]
    fn add_values() {
        let mut stack = ArrayStack { 
            array: vec!['a', 'b'],
        };
        assert_eq!(stack.add(0, 'x'), Ok(()));
        assert_eq!(stack.array, vec!['x', 'a', 'b']);
        assert_eq!(stack.add(3, 'y'), Ok(()));
        assert_eq!(stack.array, vec!['x', 'a', 'b', 'y']);
        assert_eq!(stack.add(5, 'z'), Err(IndexOutOfBounds));
        assert_eq!(stack.array, vec!['x', 'a', 'b', 'y'])
    }

    #[test]
    fn remove_values() {
        let mut stack = ArrayStack { 
            array: vec!['x', 'a', 'b', 'y'],
        };
        assert_eq!(stack.remove(2), Ok('b'));
        assert_eq!(stack.array, vec!['x', 'a', 'y']);
        assert_eq!(stack.remove(3), Err(IndexOutOfBounds));
        assert_eq!(stack.array, vec!['x', 'a', 'y']);
    }

    #[test]
    fn push_values() {
        let mut stack = ArrayStack { 
            array: vec![],
        };
        stack.push("foo");
        assert_eq!(stack.array, vec!["foo"]);
        stack.push("bar");
        assert_eq!(stack.array, vec!["foo", "bar"]);
    }

    #[test]
    fn pop_values() {
        let mut stack = ArrayStack { 
            array: vec!["baz"],
        };
        assert_eq!(stack.pop(), Some("baz"));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn shrink_array() {
        let mut stack = ArrayStack { 
            array: vec![1, 2, 3],
        };
        assert!(!stack.try_shrink());
        assert_eq!(stack.array.capacity(), 3);
        stack.array.pop();
        stack.array.pop();
        assert!(stack.try_shrink());
        assert_eq!(stack.array.capacity(), 2);
    }
}
