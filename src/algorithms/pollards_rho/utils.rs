use bnum::types::U256;
use rand::Rng;

pub fn floyds_cycle_detection<F, P>(init: U256, next: &F, finished: &P) -> (U256, U256)
where
    F: Fn(&U256) -> U256 + ?Sized,
    P: Fn(&U256, &U256) -> bool + ?Sized,
{
    let mut tortoise = init;
    let mut hare = next(&tortoise);
    while !finished(&tortoise, &hare) {
        tortoise = next(&tortoise);
        hare = next(&next(&hare));
    }
    (tortoise, hare)
}

pub fn generate_pseudorandom_fn(n: &'_ U256) -> impl Fn(&U256) -> U256 + '_ {
    let c = random_integer(n);
    move |x| (x.pow(2) + c) % n
}

fn random_integer(bound: &U256) -> U256 {
    rand::thread_rng().gen_range(U256::from(2u8)..*bound)
}
