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
    pub fn initialize() {
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
}
