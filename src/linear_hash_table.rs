use std::mem;
use crate::hashers::DimHasher;


pub struct LinearHashTable<H: DimHasher> {
    dim: u32,
    table: Vec<Entry<u64>>,
    q: usize,
    len: usize,
    hasher: H,
}

#[derive(Clone, PartialEq)]
enum Entry<T> {
    Val(T),
    Nil,
    Del,
}

impl<T: PartialEq> Entry<T> {
    fn is_val(&self, x: T) -> bool {
        Self::Val(x) == *self
    }

    fn is_nil(&self) -> bool {
        Self::Nil == *self
    }

    fn is_del(&self) -> bool {
        Self::Del == *self
    }
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
            i = (i + 1) % self.table.len();
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
            let mut i = self.hash(x);
            while let Entry::Val(_) = &self.table[i] {
                i = (i + 1) % self.table.len();
            }
            if let Entry::Nil = &self.table[i] {
                self.q += 1;
            }
            self.len += 1;
            self.table[i] = Entry::Val(x);
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
            i = (i + 1) % self.table.len();
        }
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
                let mut i = self.hash(y);
                while !self.table[i].is_nil() {
                    i = (i + 1) % self.table.len();
                }
                self.table[i] = x;
            }
        }
    }

    fn grow_invariant_holds(&self) -> bool {
        self.table.len() >= 2 * self.q
    }

    fn shrink_invariant_holds(&self) -> bool {
        self.table.len() <= 8 * self.len()
    }

}
