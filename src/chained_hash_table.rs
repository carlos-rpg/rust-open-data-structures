use std::mem;
use crate::hashers;


#[derive(Debug, Clone)]
pub struct ChainedHashTable<H: hashers::DimHasher> {
    dim: u32,
    table: Vec<Vec<u64>>,
    hasher: H,
    len: usize
}

#[derive(Debug, PartialEq)]
pub enum Error {
    KeyAlreadyExists,
    KeyNotFound,
}

impl<H: hashers::DimHasher> ChainedHashTable<H> {
    pub fn initialize(dim: u32, hasher: H) -> Self {
        assert!(dim > 0, "ChainedHashTable dimension must be greater than 0");
        let table = Self::new_table(dim);
        Self { dim, table, hasher, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn hash(&self, x: u64) -> usize {
        let y = self.hasher.hash(x, self.dim);
        y.try_into().expect("Unable to cast x's u64 into usize")
    }

    pub fn contains(&self, x: u64) -> bool {
        let i = self.hash(x);
        let row = &self.table[i];
        row.iter().any(|y| *y == x)
    }

    pub fn add(&mut self, x: u64) -> Result<(), Error> {
        if self.contains(x) {
            Err(Error::KeyAlreadyExists)
        }
        else {
            if !self.size_invarian_holds() {
                self.resize(self.dim + 1);
            }
            let i = self.hash(x);
            self.table[i].push(x);
            self.len += 1;
            Ok(())
        }
    }

    pub fn remove(&mut self, x: u64) -> Result<(), Error> {
        let i = self.hash(x);

        let j = self.table[i].iter()
            .position(|y| *y == x)
            .ok_or(Error::KeyNotFound)?;

        self.table[i].remove(j);
        self.len -= 1;

        if self.table_is_very_long() {
            self.resize(self.dim - 1);
        }
        Ok(())
    }

    fn resize(&mut self, to_dim: u32) {
        self.dim = to_dim;
        let mut table = Self::new_table(to_dim);
        mem::swap(&mut self.table, &mut table);

        for row in table {
            for x in row {
                let i = self.hash(x);
                self.table[i].push(x);
            }
        }
    }

    fn new_table(dim: u32) -> Vec<Vec<u64>> {
        vec![vec![]; 2usize.pow(dim)]
    }

    fn size_invarian_holds(&self) -> bool {
        self.len() <= self.table.len()
    }

    fn table_is_very_long(&self) -> bool {
        self.dim > 1 && self.len() * 3 < self.table.len()
    }
}

impl<H: hashers::DimHasher> PartialEq for ChainedHashTable<H> {
    fn eq(&self, other: &Self) -> bool {
        let is_in_other = |row: &Vec<u64>| row
            .iter()
            .all(|x| other.contains(*x));

        self.len() == other.len() && self.table.iter().all(is_in_other)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize() {
        let h1= hashers::Multiplicative::new();
        let chs1 = ChainedHashTable::initialize(1, h1);
        assert_eq!(chs1.dim, 1);
        assert_eq!(chs1.table.len(), 2);
        assert_eq!(chs1.len(), 0);

        let h2 = hashers::Multiplicative::new();
        let chs2 = ChainedHashTable::initialize(10, h2);
        assert_eq!(chs2.dim, 10);
        assert_eq!(chs2.table.len(), 1024);
        assert_eq!(chs2.len(), 0);
    }

    #[test]
    #[should_panic]
    fn initialize_dim_zero() {
        let h = hashers::Multiplicative::new();
        let _ = ChainedHashTable::initialize(0, h);
    }

    #[test]
    fn partial_eq_same_len() {
        let cht1 = ChainedHashTable {
            dim: 1, 
            table: vec![
                vec![0, 1013],
                vec![3, 43, 18446744073709551615],
            ],
            hasher: hashers::Multiplicative::with_seed(42),
            len: 5,
        };
        let cht2 = ChainedHashTable {
            dim: 2, 
            table: vec![
                vec![3, 0],
                vec![18446744073709551615],
                vec![43],
                vec![1013],
            ],
            hasher: hashers::Multiplicative::with_seed(7),
            len: 5,
        };
        let cht3 = ChainedHashTable {
            dim: 1, 
            table: vec![
                vec![0, 18446744073709551615],
                vec![3, 10, 2026],
            ],
            hasher: hashers::Multiplicative::with_seed(13),
            len: 5,
        };
        assert_eq!(cht1, cht2);
        assert_ne!(cht1, cht3);
        assert_ne!(cht2, cht3);
    }

    #[test]
    fn partial_eq_different_len() {
        let cht1 = ChainedHashTable {
            dim: 1, 
            table: vec![
                vec![0],
                vec![2450321026, 18446744073709551615],
            ],
            hasher: hashers::Multiplicative::with_seed(555),
            len: 3,
        };
        let cht2 = ChainedHashTable {
            dim: 2, 
            table: vec![
                vec![0, 1985059619],
                vec![18446744073709551615],
                vec![2450321026],
                vec![],
            ],
            hasher: hashers::Multiplicative::with_seed(777),
            len: 4,
        };
        assert_ne!(cht1, cht2);
    }
    
    #[test]
    fn add() {
        let mut cht1 = ChainedHashTable {
            dim: 2,
            table: vec![vec![]; 4],
            hasher: hashers::Multiplicative::with_seed(3141592),
            len: 0,
        };
        let out1 = cht1.add(42);
        let out2 = cht1.add(101);
        let out3 = cht1.add(0);
        let out4 = cht1.add(18446744073709551615);
        let out5 = cht1.add(0);

        assert_eq!(out1, Ok(()));
        assert_eq!(out2, Ok(()));
        assert_eq!(out3, Ok(()));
        assert_eq!(out4, Ok(()));
        assert_eq!(out5, Err(Error::KeyAlreadyExists));

        let cht2 = ChainedHashTable {
            dim: 1,
            table: vec![
                vec![0, 101],
                vec![42, 18446744073709551615],
            ],
            hasher: hashers::Multiplicative::with_seed(10101),
            len: 4,
        };
        assert_eq!(cht1, cht2);
    }

    #[test]
    fn remove() {
        let mut cht1 = ChainedHashTable {
            dim: 2,
            table: vec![
                vec![42, 101, 0],
                vec![],
                vec![],
                vec![18446744073709551615],
            ],
            hasher: hashers::Multiplicative::with_seed(8086),
            len: 4,
        };
        let out1 = cht1.remove(42);
        assert_eq!(out1, Ok(()));
        let out2 = cht1.remove(69);
        assert_eq!(out2, Err(Error::KeyNotFound));
        let out3 = cht1.remove(101);
        assert_eq!(out3, Ok(()));

        let cht2 = ChainedHashTable {
            dim: 1,
            table: vec![
                vec![0, 18446744073709551615],
                vec![],
            ],
            hasher: hashers::Multiplicative::with_seed(234),
            len: 2,
        };
        assert_eq!(cht1, cht2);
    }

    #[test]
    fn contains() {
        let cht1 = ChainedHashTable { 
            dim: 2,
            table: vec![
                vec![0],
                vec![18446744073709551615],
                vec![24503201026],
                vec![19850596],
            ],
            hasher: hashers::Multiplicative::with_seed(80085),
            len: 4,
        };
        assert!(cht1.contains(0));
        assert!(cht1.contains(18446744073709551615));
        assert!(cht1.contains(24503201026));
        assert!(!cht1.contains(42))
    }

    #[test]
    fn resize() {
        let cht1 = ChainedHashTable {
            dim: 1, 
            table: vec![
                vec![0, 3151, 3053, 1496],
                vec![18446744073709551615, 4253, 4161, 3199, 3057],
            ],
            hasher: hashers::Multiplicative::with_seed(1986),
            len: 9,
        };
        let mut cht2 = cht1.clone();

        cht2.resize(2);
        assert_eq!(cht1, cht2);
        assert_eq!(cht2.table.len(), 4);

        cht2.resize(3);
        assert_eq!(cht1, cht2);
        assert_eq!(cht2.table.len(), 8);

        cht2.resize(4);
        assert_eq!(cht1, cht2);
        assert_eq!(cht2.table.len(), 16);

        cht2.resize(3);
        assert_eq!(cht1, cht2);
        assert_eq!(cht2.table.len(), 8);

        cht2.resize(2);
        assert_eq!(cht1, cht2);
        assert_eq!(cht2.table.len(), 4);
    
        cht2.resize(1);
        assert_eq!(cht1, cht2);
        assert_eq!(cht2.table.len(), 2);
    }
}
