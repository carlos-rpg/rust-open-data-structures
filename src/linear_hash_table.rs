use crate::hashers::DimHasher;


pub struct LinearHashTable<H: DimHasher> {
    dim: u32,
    table: Vec<Entry<u64>>,
    q: usize,
    len: usize,
    hasher: H,
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

}
