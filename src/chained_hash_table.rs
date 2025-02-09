use rand::random_range;

pub struct ChainedHashTable {
    dim: u32,
    table: Vec<Vec<u64>>,
    odd: u64,
    len: usize
}

impl ChainedHashTable {
    pub fn initialize(dim: u32) -> Self {
        assert!(dim > 0, "ChainedHashTable dimension must be greater than 0");
        let table_len = 2usize.pow(dim);
        let table = vec![vec![]; table_len];
        let odd = 2 * random_range(0..u64::MAX / 2) + 1;
        Self { dim, table, odd, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn find(&self, x: u64) -> Option<u64> {
        let row = &self.table[self.hash(x)];
        row.iter().find(|&y| *y == x).copied()
    }

    pub fn hash(&self, x: u64) -> usize {
        let y = self.odd.overflowing_mul(x).0 >> (u64::BITS - self.dim);
        y.try_into().expect("Unable to cast x's type into usize")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize() {
        let chs1 = ChainedHashTable::initialize(1);
        assert_eq!(chs1.dim, 1);
        assert_eq!(chs1.table.len(), 2);
        assert_eq!(chs1.odd % 2, 1);
        assert_eq!(chs1.len(), 0);

        let chs2 = ChainedHashTable::initialize(2);
        assert_eq!(chs2.dim, 2);
        assert_eq!(chs2.table.len(), 4);
        assert_eq!(chs2.odd % 2, 1);
        assert_eq!(chs2.len(), 0);
    }

    #[test]
    #[should_panic]
    fn initialize_wrong_dim() {
        let _chs = ChainedHashTable::initialize(0);
    }
}
