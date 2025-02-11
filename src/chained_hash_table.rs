use rand::random_range;
use std::mem::swap;

#[derive(Debug, Clone)]
pub struct ChainedHashTable {
    dim: u32,
    table: Vec<Vec<u32>>,
    odd: u32,
    len: usize
}

#[derive(Debug, PartialEq)]
pub enum Error {
    KeyAlreadyExists,
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

    pub fn hash(&self, x: u32) -> usize {
        let y = self.odd.overflowing_mul(x).0 >> (u32::BITS - self.dim);
        y.try_into().expect("Unable to cast x's type into usize")
    }

    pub fn contains(&self, x: u32) -> bool {
        let row = &self.table[self.hash(x)];
        row.iter().any(|y| *y == x)
    }

    pub fn add(&mut self, x: u32) -> Result<(), Error> {
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

    fn resize(&mut self, dim: u32) {
        let mut other = Self::initialize(dim);
        swap(self, &mut other);

        for row in other.table {
            for x in row {
                let i = self.hash(x);
                self.table[i].push(x);
                self.len += 1;
            }
        }
    }

    fn size_invarian_holds(&self) -> bool {
        self.len() <= self.table.len()
    }
}

impl PartialEq for ChainedHashTable {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for row in &self.table {
            for x in row {
                if !other.contains(*x) {
                    return false;
                }
            }
        }
        true
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

    #[test]
    fn contains() {
        let cht1 = cht(
            2, 
            vec![
                vec![3618622750],
                vec![2688180142, 3783518487],
                vec![],
                vec![3826048670],
            ],
            49460819,
            4,
        );
        assert!(cht1.contains(3618622750));
        assert!(cht1.contains(3783518487));
        assert!(cht1.contains(3826048670));
        assert!(!cht1.contains(42))
    }

    #[test]
    fn partial_eq_same_len() {
        let cht1 = ChainedHashTable {
            dim: 1, 
            table: vec![
                vec![2450321026],
                vec![1985059619, 2014097083, 3473442836],
            ],
            odd: 1059656881,
            len: 4,
        };
        let cht2 = ChainedHashTable {
            dim: 2, 
            table: vec![
                vec![1985059619],
                vec![2450321026, 2014097083],
                vec![3473442836],
                vec![],
            ],
            odd: 3366355585,
            len: 4,
        };
        let cht3 = ChainedHashTable {
            dim: 1, 
            table: vec![
                vec![0],
                vec![2450321026, 1985059619, 3473442836],
            ],
            odd: 949054937,
            len: 4,
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
                vec![2450321026],
                vec![1985059619, 2014097083],
            ],
            odd: 1059656881,
            len: 3,
        };
        let cht2 = ChainedHashTable {
            dim: 2, 
            table: vec![
                vec![1985059619],
                vec![2450321026, 2014097083],
                vec![3473442836],
                vec![],
            ],
            odd: 3366355585,
            len: 4,
        };
        assert_ne!(cht1, cht2);
    }

    #[test]
    fn resize() {
        let cht1 = ChainedHashTable {
            dim: 1, 
            table: vec![
                vec![425364276, 3151484320, 4161335175, 3053504173, 1496293961],
                vec![3199143396, 3057340300, 3235619151],
            ],
            odd: 1610605069,
            len: 8,
        };
        let mut cht2 = cht1.clone();

        cht2.resize(2);
        assert_eq!(cht1, cht2);
        cht2.resize(3);
        assert_eq!(cht1, cht2);
        cht2.resize(4);
        assert_eq!(cht1, cht2);
        cht2.resize(3);
        assert_eq!(cht1, cht2);
        cht2.resize(2);
        assert_eq!(cht1, cht2);
        cht2.resize(1);
        assert_eq!(cht1, cht2);
    }

    #[test]
    fn add() {
        let mut cht1 = ChainedHashTable {
            dim: 2,
            table: vec![vec![]; 4],
            odd: 2799304215,
            len: 0,
        };
        let out1 = cht1.add(42);
        let out2 = cht1.add(101);
        let out3 = cht1.add(0);
        let out4 = cht1.add(58008);
        let out5 = cht1.add(0);

        assert_eq!(out1, Ok(()));
        assert_eq!(out2, Ok(()));
        assert_eq!(out3, Ok(()));
        assert_eq!(out4, Ok(()));
        assert_eq!(out5, Err(Error::KeyAlreadyExists));

        let cht2 = ChainedHashTable {
            dim: 1,
            table: vec![
                vec![42, 101, 0, 58008],
                vec![],
            ],
            odd: 1,
            len: 4,
        };
        assert_eq!(cht1, cht2);
    }
}
