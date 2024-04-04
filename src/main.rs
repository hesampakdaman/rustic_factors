use rustic_factors::algorithms;
use rustic_factors::Factorization;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <algorithm> <number>", args[0]);
        std::process::exit(1);
    }
    let method = &args[1];
    let n: u128 = args[2]
        .parse()
        .expect("Please provide a valid positive integer");
    match method.as_str() {
        "pollards_rho" => println!("{}", Factorization::new::<algorithms::PollardsRho>(n)),
        "trial_division" => println!("{}", Factorization::new::<algorithms::TrialDivision>(n)),
        _ => eprintln!("Unknown algorithm. Available options: pollards_rho, trial_division"),
    }
}
