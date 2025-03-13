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

impl<H: DimHasher> LinearHashTable<H> {
    pub fn initialize(hasher: H) -> Self {
        let table = vec![Entry::Nil; 2];
        Self { dim: 1, table, q: 0, len: 0, hasher }
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
        let mut y = &self.table[i];

        while !y.is_nil() {
            if !y.is_del() && y.is_val(x) {
                return true;
            }
            i = (i + 1) % self.table.len();
            y = &self.table[i];
        }
        false
    }

}
