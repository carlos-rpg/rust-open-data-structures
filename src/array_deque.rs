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
            let i_array = self.index_array(i);
            Ok(self.array[i_array].clone())
        }
        else {
            Err(Error::IndexOutOfBounds(i))
        }
    }

    pub fn set(&mut self, i: usize, x: T) -> Result<T, Error> {
        let y = self.get(i)?;
        let i_array = self.index_array(i);
        self.array[i_array] = x;
        Ok(y)
    }

    pub fn add(&mut self, i: usize, x: T) {
        if i > self.len() {
            panic!("Index out of bounds");
        }
        if self.len() == self.array.len() {
            self.reset_array();
            self.array.insert(i, x);
        }
        else {
            for j in (i..self.len()).rev() {
                let j_array = self.index_array(j);
                let jp1_array = self.index_array(j + 1);
                self.array[jp1_array] = self.array[j_array].clone();
            }
            let i_array = self.index_array(i);
            self.array[i_array] = x;
        }
        self.len += 1;
    }

    fn index_array(&self, i: usize) -> usize {
        (self.first + i) % self.array.len()
    }

    fn reset_array(&mut self) {
        self.array.rotate_left(self.first);
        self.first = 0;
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

    #[test]
    fn add_as_push() {
        let mut queue = ArrayDeque {
            array: vec![],
            first: 0,
            len: 0,
        };
        queue.add(0, 'a');
        queue.add(1, 'b');
        queue.add(2, 'c');
        assert_eq!(queue.array, vec!['a', 'b', 'c']);
        assert_eq!(queue.first, 0);
        assert_eq!(queue.len, 3);
    }

    #[test]
    fn add_as_front_insertion() {
        let mut queue = ArrayDeque {
            array: vec![],
            first: 0,
            len: 0,
        };
        queue.add(0, 'a');
        queue.add(0, 'b');
        queue.add(0, 'c');
        assert_eq!(queue.array, vec!['c', 'b', 'a']);
        assert_eq!(queue.first, 0);
        assert_eq!(queue.len, 3);
    }

    #[test]
    fn add_within_array() {
        let mut queue = ArrayDeque {
            array: vec![1, 2, 3],
            first: 1,
            len: 2,
        };
        queue.add(0, 10);
        assert_eq!(queue.array, vec![3, 10, 2]);
        assert_eq!(queue.first, 1);
        assert_eq!(queue.len, 3);
        queue.add(1, 20);
        assert_eq!(queue.array, vec![10, 20, 2, 3]);
        assert_eq!(queue.first, 0);
        assert_eq!(queue.len, 4);
    }

    #[test]
    fn add_as_append() {
        let mut queue = ArrayDeque {
            array: vec![1, 2, 3],
            first: 1,
            len: 2,
        };
        queue.add(2, 10);
        assert_eq!(queue.array, vec![10, 2, 3]);
        assert_eq!(queue.first, 1);
        assert_eq!(queue.len, 3);
        queue.add(3, 20);
        assert_eq!(queue.array, vec![2, 3, 10, 20]);
        assert_eq!(queue.first, 0);
        assert_eq!(queue.len, 4);
        queue.add(4, 30);
        assert_eq!(queue.array, vec![2, 3, 10, 20, 30]);
        assert_eq!(queue.first, 0);
        assert_eq!(queue.len, 5);
    }

    #[test]
    #[should_panic]
    fn add_out_of_bounds() {
        let mut queue = ArrayDeque {
            array: vec![1, 2, 3],
            first: 1,
            len: 2,
        };
        queue.add(3, 10);
    }
}
