use rand::Rng;
use std::ops::Range;

pub fn floyds_cycle_detection<F, P>(init: u128, next: &F, finished: &P) -> (u128, u128)
where
    F: Fn(u128) -> u128 + ?Sized,
    P: Fn(u128, u128) -> bool + ?Sized,
{
    let mut tortoise = init;
    let mut hare = next(tortoise);
    while !finished(tortoise, hare) {
        tortoise = next(tortoise);
        hare = next(next(hare));
    }
    (tortoise, hare)
}

pub fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn generate_psudeorandom_fn(n: u128) -> impl Fn(u128) -> u128 {
    let c = random_integer(1..n);
    move |x| (x * x + c) % n
}

fn random_integer(r: Range<u128>) -> u128 {
    rand::thread_rng().gen_range(r)
}
