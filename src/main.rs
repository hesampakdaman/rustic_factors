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
    if let Err(msg) = run(method, n) {
        eprintln!("{msg}");
        std::process::exit(1)
    }
}

fn run(method: &str, n: u128) -> Result<(), &str> {
    match method {
        "pollards_rho" => println!("{}", Factorization::new::<algorithms::PollardsRho>(n)),
        "trial_division" => println!("{}", Factorization::new::<algorithms::TrialDivision>(n)),
        _ => {
            return Err("Unknown algorithm. Available options: pollards_rho, trial_division");
        }
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pollards_rho() {
        assert!(run("pollards_rho", 123).is_ok());
    }

    #[test]
    fn test_trial_division() {
        assert!(run("trial_division", 123).is_ok());
    }

    #[test]
    fn test_unknown_method() {
        assert_eq!(
            run("unknown_method", 123).unwrap_err(),
            "Unknown algorithm. Available options: pollards_rho, trial_division"
        );
    }
}
