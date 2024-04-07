use bnum::types::U512;
use rustic_factors::algorithms;
use rustic_factors::Factorization;
use std::env;

fn main() {
    if let Err(msg) = run(env::args().collect()) {
        eprintln!("{msg}");
        std::process::exit(1)
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    if args.len() < 3 {
        return Err(format!("Usage: {} <algorithm> <number>", args[0]));
    }
    let method = &args[1];
    let n: U512 = args[2]
        .parse()
        .map_err(|_| String::from("Please provide a valid positive integer"))?;
    match method.as_str() {
        "pollards_rho" => println!("{}", Factorization::new::<algorithms::PollardsRho>(&n)),
        "trial_division" => println!("{}", Factorization::new::<algorithms::TrialDivision>(&n)),
        _ => {
            return Err(String::from(
                "Unknown algorithm. Available options: pollards_rho, trial_division",
            ));
        }
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_cases() {
        for method in ["pollards_rho", "trial_division"] {
            assert!(run(vec![
                String::from("rustic_factors"),
                String::from(method),
                String::from("123")
            ])
            .is_ok());
        }
    }

    #[test]
    fn too_few_args() {
        assert_eq!(
            run(vec![String::from("rustic_factors")]).unwrap_err(),
            String::from("Usage: rustic_factors <algorithm> <number>")
        );
    }

    #[test]
    fn unknown_method() {
        assert_eq!(
            run(vec![
                String::from("rustic_factors"),
                String::from("unknown_method"),
                String::from("123")
            ])
            .unwrap_err(),
            String::from("Unknown algorithm. Available options: pollards_rho, trial_division")
        );
    }
}
