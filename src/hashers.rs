use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;


pub trait DimHasher {
    fn hash(&self, x: u64, dim: u32) -> u64;
}

#[derive(Debug, Clone)]
pub struct Multiplicative {
    z: u64,
}

impl Multiplicative {
    pub fn new() -> Self {
        let rng = Pcg64Mcg::from_os_rng();
        Self { z: Self::odd_random_range(rng) }
    }

    pub fn with_seed(state: u64) -> Self {
        let rng = Pcg64Mcg::seed_from_u64(state);
        Self { z: Self::odd_random_range(rng) }
    }

    fn odd_random_range(mut rng: Pcg64Mcg) -> u64 {
        2 * rng.random_range(u64::MIN..u64::MAX / 2) + 1
    }
}

impl DimHasher for Multiplicative {
    fn hash(&self, x: u64, dim: u32) -> u64 {
        self.z.overflowing_mul(x).0 >> (u64::BITS - dim)
    }
}


pub struct Tabulation {
    r: u32,
    tab: Vec<Vec<u64>>,
}

impl Tabulation {
    pub fn new(r: u32) -> Self {
        let rng = Pcg64Mcg::from_os_rng();
        Self { r, tab: Self::make_tab(r, rng) }
    }

    pub fn with_seed(r: u32, state: u64) -> Self {
        let rng = Pcg64Mcg::seed_from_u64(state);
        Self { r, tab: Self::make_tab(r, rng) }
    }

    fn make_tab(r: u32, mut rng: Pcg64Mcg) -> Vec<Vec<u64>> {
        assert!(r.is_power_of_two(), "r is not power of 2");

        let n_rows = (u64::BITS / r)
            .try_into()
            .expect("Unable to cast u32 into usize");

        let n_cols = 2usize
            .checked_pow(r)
            .expect("2^r can't fit in usize");

        let mut tab = Vec::with_capacity(n_rows);
        for _ in 0..n_rows {
            let mut row = Vec::with_capacity(n_cols);
            for _ in 0..n_cols {
                row.push(rng.random_range(u64::MIN..u64::MAX));
            }
            tab.push(row);
        }
        tab
    }

    fn get(&self, i: usize, x: u64) -> u64 {
        let j = x >> i as u32 * self.r & u64::MAX >> u64::BITS - self.r;
        self.tab[i][j as usize]
    }
}

impl DimHasher for Tabulation {
    fn hash(&self, x: u64, dim: u32) -> u64 {
        let tabs = (0..self.tab.len())
            .map(|i| self.get(i, x))
            .fold(0, |acc, t| acc ^ t);

        tabs >> (u64::BITS - dim)
    }
}


#[cfg(test)]
mod tests_multiplicative {
    use super::*;

    #[test]
    fn new() {
        let h1 = Multiplicative::new();
        assert!(h1.z % 2 == 1);
        let h2 = Multiplicative::new();
        assert!(h2.z % 2 == 1);
    }

    #[test]
    fn with_seed() {
        let h1 = Multiplicative::with_seed(0);
        assert!(h1.z % 2 == 1);
        assert_eq!(h1.z, 6198063878555692195);
        let h2 = Multiplicative::with_seed(42);
        assert!(h2.z % 2 == 1);
        assert_eq!(h2.z, 10580897095847554457);
    }

    #[test]
    fn hash() {
        let h1 = Multiplicative {
            z: 17675664392375410501,
        };
        assert_eq!(h1.hash(769936456459913124, 1), 0);
        assert_eq!(h1.hash(4993990495206945374, 1), 1);
        assert_eq!(h1.hash(6909495363674708222, 1), 1);

        let h2 = Multiplicative {
            z: 10886466572363013235,
        };
        assert_eq!(h2.hash(10168802271749888757, 32), 3310380457);
        assert_eq!(h2.hash(18339155737800036837, 32), 1773933754);
        assert_eq!(h2.hash(285347091100835473, 32), 453384951);

        let h3 = Multiplicative {
            z: 1939403831449563455,
        };
        assert_eq!(h3.hash(15344511071369365520, 64), 12818618549666319344);
        assert_eq!(h3.hash(14518584061463575402, 64), 10276551606605506838);
        assert_eq!(h3.hash(15761423750684663989, 64), 18276548952320700811);
    }

    #[test]
    #[should_panic]
    fn hash_low_dim() {
        let h = Multiplicative { z: 13};
        let _ = h.hash(42, 0);
    }

    #[test]
    #[should_panic]
    fn hash_high_dim() {
        let h = Multiplicative { z: 13 };
        let _ = h.hash(42, 65);
    }
}


#[cfg(test)]
mod test_tabulation {
    use super::*;

    #[test]
    fn new() {
        let _t1 = Tabulation::new(1);
        let _t2 = Tabulation::new(4);
        let _t3 = Tabulation::new(16);
    }

    #[test]
    #[should_panic]
    fn new_invalid_r() {
        let _t1 = Tabulation::new(0);
    }

    #[test]
    fn with_seed() {
        let _t1 = Tabulation::with_seed(1, u64::MIN);
        let _t2 = Tabulation::with_seed(4, 101);
        let _t3 = Tabulation::with_seed(16, u64::MAX);
    }

    #[test]
    fn hash() {
        let t1 = Tabulation::new(1);
        assert_eq!(t1.hash(u64::MIN, 1), t1.hash(u64::MIN, 1));
        assert_eq!(t1.hash(11, 32), t1.hash(11, 32));
        assert_eq!(t1.hash(u64::MAX, 64), t1.hash(u64::MAX, 64));

        let t3 = Tabulation::new(16);
        assert_eq!(t3.hash(u64::MIN, 1), t3.hash(u64::MIN, 1));
        assert_eq!(t3.hash(151, 32), t3.hash(151, 32));
        assert_eq!(t3.hash(u64::MAX, 64), t3.hash(u64::MAX, 64));
    }
}
