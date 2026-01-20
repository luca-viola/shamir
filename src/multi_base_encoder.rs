use num_bigint::BigUint;
use num_traits::Zero;

pub struct MultiBaseEncoder {
    base: usize,
    chars: Vec<u8>,
}

impl MultiBaseEncoder {
    pub fn new(base: usize) -> Self {
        let chars: Vec<u8> = vec![
            48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81,
            82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
            112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 33, 35, 36, 37, 38, 40, 41, 42, 43, 45, 59, 60, 61,
            62, 63, 64, 94, 95, 96, 123, 124, 125, 126, 34, 39, 46, 47, 58, 91, 92, 93
        ];

        MultiBaseEncoder { base, chars }
    }

    pub fn encode(&self, mut number: BigUint) -> String {
        if number.is_zero() {
            return "0".to_string();
        }

        let mut key = String::new();
        let base_uint = BigUint::from(self.base);

        while !number.is_zero() {
            let remainder = &number % &base_uint;
            let mod_val = remainder.to_u32_digits();
            let mod_idx = if mod_val.is_empty() { 0 } else { mod_val[0] as usize };

            key.push(self.chars[mod_idx] as char);
            number = number / &base_uint;
        }

        key.chars().rev().collect()
    }

    pub fn decode(&self, alphanumeric: &str) -> BigUint {
        let mut value = BigUint::zero();
        let length = alphanumeric.len();
        let base_uint = BigUint::from(self.base);

        for (index, ch) in alphanumeric.chars().enumerate() {
            let digit_byte = ch as u8;
            let position = self.chars.iter().position(|&x| x == digit_byte)
                .expect(&format!("Invalid character: {}", ch));

            let power = length - index - 1;
            let mut base_power = BigUint::from(1u32);
            for _ in 0..power {
                base_power = base_power * &base_uint;
            }

            value = value + (BigUint::from(position) * base_power);
        }

        value
    }
}
