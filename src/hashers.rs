use rand::{Rng, SeedableRng};
use rand_pcg::Mcg128Xsl64;


pub trait DimHasher {
    fn hash(&self, x: u64, dim: u32) -> u64;
}

#[derive(Debug, Clone)]
pub struct Multiplicative {
    z: u64,
}

impl Multiplicative {
    pub fn new() -> Self {
        let rng = rand_pcg::Pcg64Mcg::from_os_rng();
        Self { z: Self::odd_random_range(rng) }
    }

    pub fn with_seed(state: u64) -> Self {
        let rng = rand_pcg::Pcg64Mcg::seed_from_u64(state);
        Self { z: Self::odd_random_range(rng) }
    }

    fn odd_random_range(mut rng: Mcg128Xsl64) -> u64 {
        2 * rng.random_range(u64::MIN..u64::MAX / 2) + 1
    }
}

impl DimHasher for Multiplicative {
    fn hash(&self, x: u64, dim: u32) -> u64 {
        self.z.overflowing_mul(x).0 >> (u64::BITS - dim)
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
