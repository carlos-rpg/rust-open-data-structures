use std::hash;
use rand;

pub struct Multiplicative {
    state: u64,
    odd: u64,
    dim: u32,
}

impl Multiplicative {
    pub fn new(dim: u32) -> Self {
        assert!(dim > 0 && dim <= u64::BITS, "dim == 0 || dim > u64::BITS");
        Multiplicative {
            state: 0,
            odd: 2 * rand::random_range(u64::MIN..u64::MAX / 2) + 1,
            dim,
        }
    }
}

impl hash::Hasher for Multiplicative {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        let x = bytes.iter()
            .enumerate()
            .map(|(i, &byte)| (byte as u64) << i * 8)
            .sum::<u64>();

        self.state = self.odd.overflowing_mul(x).0 >> (u64::BITS - self.dim);
    }
}


#[cfg(test)]
mod tests {
    use std::hash::{Hash, Hasher};
    use super::*;

    #[test]
    fn new() {
        let h1 = Multiplicative::new(1);
        assert_eq!(h1.dim, 1);
        assert_eq!(h1.state, 0);
        assert!(h1.odd % 2 == 1);

        let h2 = Multiplicative::new(32);
        assert_eq!(h2.dim, 32);
        assert_eq!(h2.state, 0);
        assert!(h2.odd % 2 == 1);

        let h3 = Multiplicative::new(64);
        assert_eq!(h3.dim, 64);
        assert_eq!(h3.state, 0);
        assert!(h3.odd % 2 == 1);
    }

    #[test]
    #[should_panic]
    fn multiplicative_new_low_dim() {
        let _h = Multiplicative::new(0);
    }

    #[test]
    #[should_panic]
    fn multiplicative_new_high_dim() {
        let _h = Multiplicative::new(65);
    }

    #[test]
    fn multiplicative_reproducible() {
        let mut h1 = Multiplicative::new(1);
        assert_eq!(0.hash(&mut h1), 0.hash(&mut h1));
        assert_eq!(42.hash(&mut h1), 42.hash(&mut h1));
        assert_eq!(1234567890.hash(&mut h1), 1234567890.hash(&mut h1));

        let mut h2 = Multiplicative::new(32);
        assert_eq!(0.hash(&mut h2), 0.hash(&mut h2));
        assert_eq!(42.hash(&mut h2), 42.hash(&mut h2));
        assert_eq!(1234567890.hash(&mut h2), 1234567890.hash(&mut h2));

        let mut h3 = Multiplicative::new(32);
        assert_eq!(0.hash(&mut h3), 0.hash(&mut h3));
        assert_eq!(42.hash(&mut h3), 42.hash(&mut h3));
        assert_eq!(1234567890.hash(&mut h3), 1234567890.hash(&mut h3));
    }

    #[test]
    fn hash_values() {
        let mut h1 = Multiplicative {
            state: 0,
            odd: 17675664392375410501,
            dim: 1,
        };
        769936456459913124_u64.hash(&mut h1);
        assert_eq!(h1.finish(), 0);
        4993990495206945374_u64.hash(&mut h1);
        assert_eq!(h1.finish(), 1);
        6909495363674708222_u64.hash(&mut h1);
        assert_eq!(h1.finish(), 1);

        let mut h2 = Multiplicative {
            state: 0,
            odd: 10886466572363013235,
            dim: 32,
        };
        10168802271749888757_u64.hash(&mut h2);
        assert_eq!(h2.finish(), 3310380457);
        18339155737800036837_u64.hash(&mut h2);
        assert_eq!(h2.finish(), 1773933754);
        285347091100835473_u64.hash(&mut h2);
        assert_eq!(h2.finish(), 453384951);

        let mut h3 = Multiplicative {
            state: 0,
            odd: 1939403831449563455,
            dim: 64,
        };
        15344511071369365520_u64.hash(&mut h3);
        assert_eq!(h3.finish(), 12818618549666319344);
        14518584061463575402_u64.hash(&mut h3);
        assert_eq!(h3.finish(), 10276551606605506838);
        15761423750684663989_u64.hash(&mut h3);
        assert_eq!(h3.finish(), 18276548952320700811);
    }
}
