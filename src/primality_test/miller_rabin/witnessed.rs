use super::utils;

pub struct Witnessed {
    n: u128,
    t: u32,
    u: u128,
}

impl Witnessed {
    pub fn new(n: u128) -> Self {
        let t = utils::highest_power_of_2_divisor(n - 1);
        let u = (n - 1) / 2u128.pow(t);
        Self { n, t, u }
    }

    pub fn by(&self, a: u128) -> bool {
        // n - 1 = 2^t * u
        let mut x = utils::modular_exponentiation(a, self.u, self.n);
        for _ in 1..=self.t {
            if self.is_nontrivial_sqrt_of_1(x) {
                return true;
            }
            x = (x * x) % self.n;
        }
        // at this point x = a^{n-1} % n
        // recall that a^{n-1} % n must eq to 1
        // if n is prime
        utils::fails_fermats_test(x)
    }

    fn is_nontrivial_sqrt_of_1(&self, x: u128) -> bool {
        let squared = (x * x) % self.n;
        squared == 1 && x != 1 && x != self.n - 1
    }
}
