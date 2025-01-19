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
            let i_array = self.array_index(i);
            Ok(self.array[i_array].clone())
        }
        else {
            Err(Error::IndexOutOfBounds(i))
        }
    }

    pub fn set(&mut self, i: usize, x: T) -> Result<T, Error> {
        let y = self.get(i)?;
        let i_array = self.array_index(i);
        self.array[i_array] = x;
        Ok(y)
    }

    pub fn add(&mut self, i: usize, x: T) {
        if self.len() == self.array.capacity() {
            self.resize();
        }
        let i_array = self.array_index(i);

        if i < self.len() / 2 {
            for j in 0..self.len() / 2 {
                let j_array = self.array_index(j);
                self.array[j_array - 1] = self.array[j_array].clone();
            }
            self.array[i_array] = x;
        }
        else if i >= self.len() / 2 {
            self.array.insert(i_array, x);
        }
        self.len += 1;
    }

    fn resize(&mut self) {
        self.array.rotate_left(self.first);
        self.first = 0;
        self.array.reserve(1);
    }

    fn array_index(&self, i: usize) -> usize {
        (self.first + i) % self.array.len()
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
