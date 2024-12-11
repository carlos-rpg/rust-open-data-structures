use std::ops::Index;

pub struct ArrayStack<T: Clone> {
    array: Vec<T>,
}

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
