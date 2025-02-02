use crate::circular_vec::CircularVec;

#[derive(Debug)]
pub struct ArrayDeque<T: Clone + PartialEq> {
    array: CircularVec<T>,
    len: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IndexOutOfBounds(usize),
}

impl<T: Clone + PartialEq> ArrayDeque<T> {
    pub fn initialize() -> Self {
        Self { array: CircularVec::new(vec![], 0), len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.array.len()
    }

    pub fn get(&self, i: usize) -> Result<T, Error> {
        if !self.is_out_of_index_bounds(i) {
            Ok(self.array[i].clone())
        }
        else {
            Err(Error::IndexOutOfBounds(i))
        }
    }

    pub fn set(&mut self, i: usize, x: T) -> Result<T, Error> {
        let y = self.get(i)?;
        self.array[i] = x;
        Ok(y)
    }

    pub fn add(&mut self, i: usize, x: T) -> Result<(), Error> {
        if self.is_out_of_insert_bounds(i) {
            Err(Error::IndexOutOfBounds(i))
        }
        else {
            if self.is_full() {
                self.array.resize((2 * self.len()).max(1), x.clone());
            }
            if i < self.len() / 2 {
                self.array.shift_head(-1);

                for j in 0..i {
                    self.array[j] = self.array[j + 1].clone();
                }
            }
            else {
                for j in (i + 1..=self.len()).rev() {
                    self.array[j] = self.array[j - 1].clone();
                }
            }
            self.array[i] = x;
            self.len += 1;
            Ok(())
        }
    }

    fn is_out_of_index_bounds(&self, i: usize) -> bool {
        i >= self.len()
    }

    fn is_out_of_insert_bounds(&self, i: usize) -> bool {
        i > self.len()
    }
}

impl<T: Clone + PartialEq> PartialEq for ArrayDeque<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() &&
        (0..self.len()).all(|i| self.array[i] == other.array[i])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let queue = ArrayDeque { 
            array: CircularVec::new(vec!['a', 'b', 'c'], 2),
            len: 2,
        };
        assert_eq!(queue.get(0), Ok('c'));
        assert_eq!(queue.get(1), Ok('a'));
        assert_eq!(queue.get(2), Err(Error::IndexOutOfBounds(2)));
    }

    #[test]
    fn set_check_output() {
        let mut queue = ArrayDeque {
            array: CircularVec::new(vec![1, 2, 3], 1),
            len: 2,
        };
        assert_eq!(queue.set(0, 20), Ok(2));
        assert_eq!(queue.set(1, 30), Ok(3));
        assert_eq!(queue.set(2, 10), Err(Error::IndexOutOfBounds(2)));
    }

    #[test]
    fn set_check_deque() {
        let mut q1 = ArrayDeque {
            array: CircularVec::new(vec![1, 2, 3], 1),
            len: 2,
        };
        let _ = q1.set(0, 20);
        let _ = q1.set(1, 30);
        let _ = q1.set(2, 10);

        let q2 = ArrayDeque {
            array: CircularVec::new(vec![1, 20, 30], 1),
            len: 2,
        };
        assert_eq!(q1, q2);
    }

    #[test]
    fn add_back_insertion() {
        let mut q1 = ArrayDeque {
            array: CircularVec::new(vec![], 0),
            len: 0,
        };
        let _ = q1.add(0, 'a');
        let _ = q1.add(1, 'b');
        let _ = q1.add(2, 'c');

        let q2 = ArrayDeque {
            array: CircularVec::new(vec!['a', 'b', 'c'], 0),
            len: 3,
        };
        assert_eq!(q1, q2);
    }

    #[test]
    fn add_front_insertion() {
        let mut q1 = ArrayDeque {
            array: CircularVec::new(vec![], 0),
            len: 0,
        };
        let _ = q1.add(0, 'a');
        let _ = q1.add(0, 'b');
        let _ = q1.add(0, 'c');

        let q2 = ArrayDeque {
            array: CircularVec::new(vec!['c', 'b', 'a'], 0),
            len: 3,
        };
        assert_eq!(q1, q2);
    }

    // #[test]
    // fn add_mid_insertion() {
    //     let mut q1 = ArrayDeque {
    //         array: CircularVec::new(vec![1, 2, 3], 1),
    //         len: 2,
    //     };
    //     q1.add(0, 10);
    //     q1.add(1, 20);

    //     let mut q1 = ArrayDeque {
    //         array: CircularVec::new(vec![1, 2, 3], 1),
    //         len: 2,
    //     };
    //     assert_eq!(q1.array, vec![10, 20, 2, 3]);
    // }

    #[test]
    fn partial_equivalence_full() {
        let q1 = ArrayDeque {
            array: CircularVec::new(vec![1, 2, 3, 4], 2),
            len: 3,
        };
        let q2 = ArrayDeque {
            array: CircularVec::new(vec![10, 3, 4, 1], 1),
            len: 3,
        };
        let q3 = ArrayDeque {
            array: CircularVec::new(vec![10, 3, 4, 1], 1),
            len: 2,
        };
        assert_eq!(q1, q2);
        assert_ne!(q2, q3);
    }

    #[test]
    fn partial_equivalence_empty() {
        let q1: ArrayDeque<i32> = ArrayDeque {
            array: CircularVec::new(vec![], 0),
            len: 0,
        };
        let q2 = ArrayDeque {
            array: CircularVec::new(vec![10, 3, 4, 1], 1),
            len: 0,
        };
        assert_eq!(q1, q2);
    }
}
