use num_bigint::{BigInt, RandBigInt};
use num_traits::One;

pub fn floyds_cycle_detection<F, P>(init: BigInt, next: &F, finished: &P) -> (BigInt, BigInt)
where
    F: Fn(&BigInt) -> BigInt + ?Sized,
    P: Fn(&BigInt, &BigInt) -> bool + ?Sized,
{
    let mut tortoise = init;
    let mut hare = next(&tortoise);
    while !finished(&tortoise, &hare) {
        tortoise = next(&tortoise);
        hare = next(&next(&hare));
    }
    (tortoise, hare)
}

pub fn generate_pseudorandom_fn(n: &'_ BigInt) -> impl Fn(&BigInt) -> BigInt + '_ {
    move |x| (x.pow(2) + random_integer(n)) % n
}

fn random_integer(bound: &BigInt) -> BigInt {
    rand::thread_rng().gen_bigint_range(&BigInt::one(), bound)
}
