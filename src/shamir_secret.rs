use num_bigint::BigUint;
use num_traits::{Zero, One};
use rand::Rng;

pub struct ShamirSecret {
    prime: BigUint,
}

impl ShamirSecret {
    pub fn new(prime: BigUint) -> Self {
        ShamirSecret { prime }
    }

    fn eval_coefficient_tuple_at_x(&self, poly: &[BigUint], x: &BigUint) -> BigUint {
        let mut accum = BigUint::zero();

        for coeff in poly.iter().rev() {
            accum = (&accum * x) % &self.prime;
            accum = (&accum + coeff) % &self.prime;
        }

        accum
    }

    pub fn make_random_shamir_pool(&self, key: BigUint, minimum: usize, shares: usize) -> Result<(BigUint, Vec<(usize, BigUint)>), String> {
        if minimum > shares {
            return Err("can't have a threshold higher than the shares".to_string());
        }

        let mut rng = rand::thread_rng();
        let mut poly: Vec<BigUint> = Vec::with_capacity(minimum);

        poly.push(key.clone());

        for _ in 1..minimum {
            let random_bytes = (0..self.prime.bits() / 8 + 1)
                .map(|_| rng.gen::<u8>())
                .collect::<Vec<u8>>();
            let random_num = BigUint::from_bytes_be(&random_bytes) % &self.prime;
            poly.push(random_num);
        }

        let mut points = Vec::with_capacity(shares);
        for i in 1..=shares {
            let x = BigUint::from(i);
            let y = self.eval_coefficient_tuple_at_x(&poly, &x);
            points.push((i, y));
        }

        Ok((poly[0].clone(), points))
    }

    fn extended_euclidean_gcd(&self, a: &BigUint, b: &BigUint) -> (BigUint, BigUint) {
        let mut x = BigUint::zero();
        let mut last_x = BigUint::one();
        let mut y = BigUint::one();
        let mut last_y = BigUint::zero();
        let mut a_val = a.clone();
        let mut b_val = b.clone();

        while !b_val.is_zero() {
            let quot = &a_val / &b_val;

            let temp_a = a_val.clone();
            a_val = b_val.clone();
            b_val = temp_a % &a_val;

            let temp_x = x.clone();
            x = if &last_x >= &(&quot * &x) {
                &last_x - &quot * &x
            } else {
                &self.prime - ((&quot * &x - &last_x) % &self.prime)
            };
            last_x = temp_x;

            let temp_y = y.clone();
            y = if &last_y >= &(&quot * &y) {
                &last_y - &quot * &y
            } else {
                &self.prime - ((&quot * &y - &last_y) % &self.prime)
            };
            last_y = temp_y;
        }

        (last_x, last_y)
    }

    fn division_modulo_prime(&self, num: &BigUint, den: &BigUint) -> BigUint {
        let (inv, _) = self.extended_euclidean_gcd(den, &self.prime);
        (num * inv) % &self.prime
    }

    fn lagrange_interpolation(&self, x: &BigUint, x_s: &[BigUint], y_s: &[BigUint]) -> BigUint {
        let k = x_s.len();
        assert_eq!(k, y_s.len(), "x_s and y_s must have the same length");

        let unique_check: std::collections::HashSet<_> = x_s.iter().collect();
        assert_eq!(unique_check.len(), k, "points for the lagrange interpolation are not distinct");

        let mut nums = Vec::with_capacity(k);
        let mut dens = Vec::with_capacity(k);

        for i in 0..k {
            let mut others = x_s.to_vec();
            let cur = others.remove(i);

            let mut num_product = BigUint::one();
            for o in &others {
                let diff = if x >= o {
                    x - o
                } else {
                    &self.prime - (o - x)
                };
                num_product = (&num_product * diff) % &self.prime;
            }
            nums.push(num_product);

            let mut den_product = BigUint::one();
            for o in &others {
                let diff = if &cur >= o {
                    &cur - o
                } else {
                    &self.prime - (o - &cur)
                };
                den_product = (&den_product * diff) % &self.prime;
            }
            dens.push(den_product);
        }

        let mut den = BigUint::one();
        for d in &dens {
            den = (&den * d) % &self.prime;
        }

        let mut num = BigUint::zero();
        for i in 0..k {
            let term = self.division_modulo_prime(
                &((&nums[i] * &den * &y_s[i]) % &self.prime),
                &dens[i]
            );
            num = (&num + term) % &self.prime;
        }

        let result = self.division_modulo_prime(&num, &den);
        (&result + &self.prime) % &self.prime
    }

    pub fn recover_secret(&self, shares: &[(usize, BigUint)]) -> Result<BigUint, String> {
        if shares.len() < 2 {
            return Err("cannot use less than two shares".to_string());
        }

        let x_s: Vec<BigUint> = shares.iter().map(|(x, _)| BigUint::from(*x)).collect();
        let y_s: Vec<BigUint> = shares.iter().map(|(_, y)| y.clone()).collect();

        Ok(self.lagrange_interpolation(&BigUint::zero(), &x_s, &y_s))
    }
}
