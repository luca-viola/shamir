use num_bigint::BigUint;
use num_traits::One;

pub struct MersennePrimes {
    exponents_of_two: Vec<u32>,
}

impl MersennePrimes {
    pub fn new() -> Self {
        MersennePrimes {
            exponents_of_two: vec![2, 3, 5, 7, 13, 17, 19, 31, 61, 89, 107, 127, 521, 607, 1279, 2203, 2281, 3217, 4253],
        }
    }

    pub fn get_mersenne_prime(&self, n: usize) -> BigUint {
        let exponent = self.exponents_of_two[n];
        // Returns 2^exponent - 1
        (BigUint::one() << exponent) - BigUint::one()
    }
}
