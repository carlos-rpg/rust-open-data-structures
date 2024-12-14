pub struct ArrayDeque<T> {
    array: Vec<T>,
    first: usize,
    len: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IndexOutOfBounds(usize),
}

impl<T: Clone> ArrayDeque<T> {
    pub fn initialize() -> Self {
        Self { array: Vec::new(), first: 0, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, i: usize) -> Result<T, Error> {
        if i < self.len() {
            let target = (self.first + i) % self.array.len();
            Ok(self.array[target].clone())
        }
        else {
            Err(Error::IndexOutOfBounds(i))
        }
    }

    pub fn set(&mut self, i: usize, x: T) -> Result<T, Error> {
        let y = self.get(i)?;
        let target = (self.first + i) % self.array.len();
        self.array[target] = x;
        Ok(y)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let queue = ArrayDeque { 
            array: vec!['a', 'b', 'c'],
            first: 2,
            len: 2,
        };
        assert_eq!(queue.get(0), Ok('c'));
        assert_eq!(queue.get(1), Ok('a'));
        assert_eq!(queue.get(2), Err(Error::IndexOutOfBounds(2)));
    }

    #[test]
    fn set_check_output() {
        let mut queue = ArrayDeque {
            array: vec![1, 2, 3],
            first: 1,
            len: 2,
        };
        assert_eq!(queue.set(0, 20), Ok(2));
        assert_eq!(queue.set(1, 30), Ok(3));
        assert_eq!(queue.set(2, 10), Err(Error::IndexOutOfBounds(2)));
    }

    #[test]
    fn set_check_array() {
        let mut queue = ArrayDeque {
            array: vec![1, 2, 3],
            first: 1,
            len: 2,
        };
        let _ = queue.set(0, 20);
        assert_eq!(queue.array, vec![1, 20, 3]);
        let _ = queue.set(1, 30);
        assert_eq!(queue.array, vec![1, 20, 30]);
        let _ = queue.set(2, 10);
        assert_eq!(queue.array, vec![1, 20, 30]);
    }
}
