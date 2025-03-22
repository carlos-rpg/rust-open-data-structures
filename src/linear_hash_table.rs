use std::mem;
use crate::hashers::DimHasher;


#[derive(Debug)]
pub struct LinearHashTable<H: DimHasher> {
    dim: u32,
    table: Vec<Entry<u64>>,
    q: usize,
    len: usize,
    hasher: H,
}

#[derive(Clone, PartialEq, Debug)]
enum Entry<T> {
    Val(T),
    Nil,
    Del,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    KeyAlreadyExists,
    KeyNotFound,
}

impl<H: DimHasher> LinearHashTable<H> {
    pub fn initialize(hasher: H) -> Self {
        Self { dim: 1, table: Self::new_table(1), q: 0, len: 0, hasher }
    }

    fn new_table(dim: u32) -> Vec<Entry<u64>> {
        assert!(dim > 0, "dim == 0");
        vec![Entry::Nil; 2usize.pow(dim)]
    }

    pub fn hash(&self, x: u64) -> usize {
        let y = self.hasher.hash(x, self.dim);
        y.try_into().expect("Unable to fit u64 into usize")
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn contains(&self, x: u64) -> bool {
        let mut i = self.hash(x);
        loop {
            match &self.table[i] {
                Entry::Val(y) => if *y == x { return true; },
                Entry::Nil => return false,
                Entry::Del => (),
            }
            i = self.loop_index(i + 1);
        }
    }

    pub fn add(&mut self, x: u64) -> Result<(), Error> {
        if self.contains(x) {
            Err(Error::KeyAlreadyExists)
        }
        else {
            if !self.grow_invariant_holds() {
                self.resize();
            }
            if let Entry::Nil = self.insert(x) {
                self.q += 1;
            }
            self.len += 1;
            Ok(())
        }
    }

    pub fn remove(&mut self, x: u64) -> Result<(), Error> {
        let mut i = self.hash(x);
        loop {
            match self.table[i] {
                Entry::Val(y) => if y == x {
                    self.table[i] = Entry::Del;
                    self.len -= 1;
                    if !self.shrink_invariant_holds() {
                        self.resize();
                    }
                    return Ok(());
                },
                Entry::Nil => return Err(Error::KeyNotFound),
                Entry::Del => (),
            }
            i = self.loop_index(i + 1);
        }
    }

    pub fn iter(&self) -> LinearHashTableIterator<H> {
        LinearHashTableIterator { ref_to: self, index: 0}
    }

    fn resize(&mut self) {
        let mut new_dim = 1;
        while 2usize.pow(new_dim) < 3 * self.len() {
            new_dim += 1;
        }
        let mut table = Self::new_table(new_dim);
        self.dim = new_dim;
        mem::swap(&mut self.table, &mut table);

        for x in table {
            if let Entry::Val(y) = x {
                let _ = self.insert(y);
            }
        }
    }

    fn grow_invariant_holds(&self) -> bool {
        self.table.len() >= 2 * (self.q + 1)
    }

    fn shrink_invariant_holds(&self) -> bool {
        self.table.len() <= 8 * self.len()
    }

    fn insert(&mut self, x: u64) -> Entry<u64> {
        let mut i = self.hash(x);
        while let Entry::Val(_) = &self.table[i] {
            i = self.loop_index(i + 1);
        }
        let entry = self.table[i].clone();
        self.table[i] = Entry::Val(x);
        entry
    }

    fn loop_index(&self, i: usize) -> usize {
        i % self.table.len()
    }

}

pub struct LinearHashTableIterator<'a, H: DimHasher> {
    ref_to: &'a LinearHashTable<H>,
    index: usize,
}

impl<'a, H: DimHasher> Iterator for LinearHashTableIterator<'a, H> {
    type Item = &'a u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.ref_to.table.len() {
            let entry = &self.ref_to.table[self.index];
            self.index += 1;
            if let Entry::Val(x) = entry { return Some(x); }
        }
        None
    }
}

impl<H: DimHasher> PartialEq for LinearHashTable<H> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().all(|x| other.contains(*x))
    }
}


#[cfg(test)]
mod tests {
    use crate::hashers;
    use super::*;

    #[test]
    fn iter() {
        let hasher = hashers::Multiplicative::new();

        let lht1 = LinearHashTable { 
            dim: 1, table: vec![Entry::Nil, Entry::Nil], q: 0, len: 0, hasher: hasher.clone()
        };
        assert_eq!(lht1.iter().collect::<Vec<&u64>>().len(), 0);

        let lht2 = LinearHashTable { 
            dim: 2, 
            table: vec![Entry::Nil, Entry::Val(3), Entry::Nil, Entry::Val(14)], 
            q: 2, 
            len: 2, 
            hasher: hasher.clone()
        };
        assert_eq!(lht2.iter().collect::<Vec<&u64>>(), vec![&3, &14]);

        let lht3 = LinearHashTable { 
            dim: 2,
            table: vec![Entry::Nil, Entry::Val(3), Entry::Del, Entry::Val(14)], 
            q: 3, 
            len: 2, 
            hasher: hasher.clone()
        };
        assert_eq!(lht3.iter().collect::<Vec<&u64>>(), vec![&3, &14]);
    }

    #[test]
    fn partial_eq() {
        let lhs1 = LinearHashTable { 
            dim: 3,
            table: vec![
                Entry::Val(0), Entry::Nil, Entry::Val(18446744073709551615), Entry::Nil,
                Entry::Nil, Entry::Nil, Entry::Nil, Entry::Val(1234567890),
            ],
            q: 3,
            len: 3,
            hasher: hashers::Multiplicative::with_seed(105),
        };
        let lhs2 = LinearHashTable { 
            dim: 3,
            table: vec![
                Entry::Val(0), Entry::Val(1234567890), Entry::Val(18446744073709551615), 
                Entry::Nil, Entry::Nil, Entry::Nil, Entry::Nil, Entry::Nil,
            ],
            q: 3,
            len: 3,
            hasher: hashers::Multiplicative::with_seed(11),
        };
        let lhs3 = LinearHashTable { 
            dim: 1,
            table: vec![Entry::Nil, Entry::Nil],
            q: 0,
            len: 0,
            hasher: hashers::Multiplicative::with_seed(1),
        };
        assert_eq!(lhs1, lhs1);
        assert_eq!(lhs1, lhs2);
        assert_ne!(lhs1, lhs3);
        assert_eq!(lhs3, lhs3);
    }

    #[test]
    fn initialize() {
        let h = hashers::Multiplicative::with_seed(32);
        let lhs = LinearHashTable::initialize(h);
        assert_eq!(lhs.dim, 1);
        assert_eq!(lhs.table.len(), 2);
        assert_eq!(lhs.len, 0);
        assert!(lhs.table.iter().all(|entry| *entry == Entry::Nil));
    }

    #[test]
    fn contains() {
        let lhs1 = LinearHashTable { 
            dim: 3,
            table: vec![
                Entry::Val(0), Entry::Del, Entry::Nil, Entry::Nil, 
                Entry::Nil, Entry::Nil, Entry::Val(1234567890), Entry::Nil,
            ],
            q: 3,
            len: 2,
            hasher: hashers::Multiplicative::with_seed(101325),
        };
        assert!(lhs1.contains(0));
        assert!(lhs1.contains(1234567890));
        assert!(!lhs1.contains(18446744073709551615));
        assert!(!lhs1.contains(151));
    }
    #[test]
    fn add() {
        let hasher = hashers::Multiplicative::with_seed(42);
        let mut lhs = LinearHashTable { 
            dim: 1, table: vec![Entry::Nil, Entry::Nil], q: 0, len: 0, hasher
        };
        assert_eq!(lhs.add(0), Ok(()));
        assert_eq!(lhs.len(), 1);
        assert!(lhs.contains(0));

        assert_eq!(lhs.add(101054), Ok(()));
        assert_eq!(lhs.len(), 2);
        assert!(lhs.contains(101054));

        assert_eq!(lhs.add(101054), Err(Error::KeyAlreadyExists));
        assert_eq!(lhs.len(), 2);
        assert!(lhs.contains(101054));

        assert_eq!(lhs.add(18446744073709551615), Ok(()));
        assert_eq!(lhs.len(), 3);
        assert!(lhs.contains(18446744073709551615));
    }
}