#![allow(dead_code)]

use crate::traits::{Factorize, PrimalityTest, PrimeFactorization};
use num_bigint::BigInt;

pub(crate) fn check_factorization<F: PrimeFactorization>(n: u32, factors: &[u32]) {
    let expected: Vec<BigInt> = factors.iter().map(|&d| BigInt::from(d)).collect();
    let mut actual = F::prime_factorization(&BigInt::from(n));
    actual.sort_unstable();
    assert_eq!(actual, expected, "Test failed for n = {}", n);
}

pub(crate) fn check_factor<F: Factorize>(n: u32, factor: u32) {
    let expected = BigInt::from(factor);
    let actual = F::factorize(&BigInt::from(n));
    assert_eq!(actual, expected, "Test failed for n = {}", n);
}

pub(crate) fn check_prime<P: PrimalityTest>(p: u32, expected: bool) {
    assert_eq!(P::is_prime(&BigInt::from(p)), expected);
}
