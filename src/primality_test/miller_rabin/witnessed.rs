use super::utils;

pub struct Witnessed {
    number: u128,
    decomposed: DecomposedExponent,
}

impl Witnessed {
    pub fn new(number: u128) -> Self {
        let exponent = number - 1;
        let decomposed = DecomposedExponent::new(exponent);
        Self { number, decomposed }
    }

    pub fn by(&self, base: u128) -> bool {
        let mut x = utils::modular_exponentiation(base, self.decomposed.odd_factor, self.number);
        for _ in 1..=self.decomposed.even_exponent {
            if self.is_nontrivial_sqrt_of_1(x) {
                return true;
            }
            x = (x * x) % self.number;
        }
        // at this point x = a^{n-1} % n
        // recall that a^{n-1} % n must eq to 1
        // if n is prime
        utils::fails_fermats_test(x)
    }

    fn is_nontrivial_sqrt_of_1(&self, x: u128) -> bool {
        let squared = (x * x) % self.number;
        squared == 1 && x != 1 && x != self.number - 1
    }
}

struct DecomposedExponent {
    even_exponent: u32,
    odd_factor: u128,
}

impl DecomposedExponent {
    /// Decomposes `exponent` into `even_exponent` and `odd_factor`,
    /// where `exponent = 2^even_exponent * odd_factor`.
    pub fn new(exponent: u128) -> Self {
        let even_exponent = utils::highest_power_of_2_divisor(exponent);
        let odd_factor = exponent / 2u128.pow(even_exponent);
        Self {
            even_exponent,
            odd_factor,
        }
    }
}
