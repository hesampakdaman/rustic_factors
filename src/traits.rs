pub trait Factorize {
    fn factorize(&self, n: u128) -> Vec<u128>;
}

pub trait PrimalityTest {
    fn is_prime(&self, p: u128) -> bool;
}
