use bnum::types::U512;
use rustic_factors::commands::Commands;
use std::env;

fn main() {
    match run(env::args().collect()) {
        Ok(result) => println!("{result}"),
        Err(msg) => eprintln!("{msg}"),
    }
}

fn run(args: Vec<String>) -> Result<String, String> {
    if args.len() < 3 {
        return Err(format!("Usage: {} <algorithm> <number>", args[0]));
    }
    let (n, method) = parse(args)?;
    Commands::default().execute(&n, &method)
}

fn parse(args: Vec<String>) -> Result<(U512, String), String> {
    let method = String::from(&args[1]);
    let n: U512 = args[2]
        .parse()
        .map_err(|_| String::from("Please provide a valid positive integer <= 2⁵¹²"))?;
    Ok((n, method))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_cases() {
        for method in [
            "fermats_factorization_method",
            "miller_rabin",
            "pollards_rho",
            "trial_division",
        ] {
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
            String::from("Unknown algorithm. Available options: fermats_factorization_method, miller_rabin, pollards_rho, trial_division")
        );
    }
}
