use rand;


pub trait DimHasher {
    fn hash(&self, x: u64, dim: u32) -> u64;
}

pub struct Multiplicative {
    odd: u64,
}

impl Multiplicative {
    pub fn new() -> Self {
        Multiplicative { 
            odd: 2 * rand::random_range(u64::MIN..u64::MAX / 2) + 1
        }
    }
}

impl DimHasher for Multiplicative {
    fn hash(&self, x: u64, dim: u32) -> u64 {
        assert!(dim > 0 && dim <= u64::BITS, "dim == 0 || dim > u64::BITS");
        self.odd.overflowing_mul(x).0 >> (u64::BITS - dim)
    }
}


#[cfg(test)]
mod tests_multiplicative {
    use super::*;

    #[test]
    fn new() {
        let h1 = Multiplicative::new();
        assert!(h1.odd % 2 == 1);

        let h2 = Multiplicative::new();
        assert!(h2.odd % 2 == 1);

        let h3 = Multiplicative::new();
        assert!(h3.odd % 2 == 1);
    }

    #[test]
    fn hash() {
        let h1 = Multiplicative {
            odd: 17675664392375410501,
        };
        assert_eq!(h1.hash(769936456459913124, 1), 0);
        assert_eq!(h1.hash(4993990495206945374, 1), 1);
        assert_eq!(h1.hash(6909495363674708222, 1), 1);

        let h2 = Multiplicative {
            odd: 10886466572363013235,
        };
        assert_eq!(h2.hash(10168802271749888757, 32), 3310380457);
        assert_eq!(h2.hash(18339155737800036837, 32), 1773933754);
        assert_eq!(h2.hash(285347091100835473, 32), 453384951);

        let h3 = Multiplicative {
            odd: 1939403831449563455,
        };
        assert_eq!(h3.hash(15344511071369365520, 64), 12818618549666319344);
        assert_eq!(h3.hash(14518584061463575402, 64), 10276551606605506838);
        assert_eq!(h3.hash(15761423750684663989, 64), 18276548952320700811);
    }

    #[test]
    #[should_panic]
    fn hash_low_dim() {
        let h = Multiplicative { odd: 13};
        let _ = h.hash(42, 0);
    }

    #[test]
    #[should_panic]
    fn hash_high_dim() {
        let h = Multiplicative { odd: 13 };
        let _ = h.hash(42, 65);
    }
}
