use num_bigint::BigInt;

pub trait PrimeFactorization {
    fn prime_factorization(n: &BigInt) -> Vec<BigInt>;
}

pub trait Factorize {
    fn factorize(n: &BigInt) -> BigInt;
}

pub trait PrimalityTest {
    fn is_prime(p: &BigInt) -> bool;
}
