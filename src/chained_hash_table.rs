use rand::random_range;
use std::mem::swap;

pub struct ChainedHashTable {
    dim: u32,
    table: Vec<Vec<u32>>,
    odd: u32,
    len: usize
}

pub enum Error {
    KeyAlreadyExists,
    TableIsFull,
}

impl ChainedHashTable {
    pub fn initialize(dim: u32) -> Self {
        assert!(dim > 0, "ChainedHashTable dimension must be greater than 0");
        let table_len = 2usize.pow(dim);
        let table = vec![vec![]; table_len];
        let odd = 2 * random_range(u32::MIN..u32::MAX / 2) + 1;
        Self { dim, table, odd, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn find(&self, x: u32) -> Option<u32> {
        let row = &self.table[self.hash(x)];
        row.iter().find(|&y| *y == x).copied()
    }

    pub fn add(&mut self, x: u32) -> Result<(), Error> {
        if self.len() >= self.table.len() {
            Err(Error::TableIsFull)
        }
        else if !self.find(x).is_none() {
            Err(Error::KeyAlreadyExists)
        }
        else {
            let i = self.hash(x);
            self.table[i].push(x);
            self.len += 1;
            Ok(())
        }
    }

    fn resize(&mut self, dim: u32) {
        let mut other = Self::initialize(dim);
        swap(self, &mut other);

        for row in other.table {
            for x in row {
                let i = self.hash(x);
                self.table[i].push(x);
            }
        }
    }

    pub fn hash(&self, x: u32) -> usize {
        let y = self.odd.overflowing_mul(x).0 >> (u32::BITS - self.dim);
        y.try_into().expect("Unable to cast x's type into usize")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cht(dim: u32, table: Vec<Vec<u32>>, odd: u32, len: usize) -> ChainedHashTable {
        ChainedHashTable { dim, table, odd, len }
    }

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

    #[test]
    fn hash() {
        let cht1 = cht(1, vec![], 2579419223, 0);
        assert_eq!(cht1.hash(790641268), 1);
        assert_eq!(cht1.hash(2204740408), 0);
        assert_eq!(cht1.hash(3442113750), 1);

        let cht2 = cht(16, vec![], 1881984275, 0);
        assert_eq!(cht2.hash(3783082547), 47890);
        assert_eq!(cht2.hash(103666855), 50532);
        assert_eq!(cht2.hash(2941348170), 44307);

        let cht3 = cht(32, vec![], 694784213, 0);
        assert_eq!(cht3.hash(3066815969), 457036853);
        assert_eq!(cht3.hash(59612175), 96618619);
        assert_eq!(cht3.hash(3151035214), 1664808934);
    }
}
