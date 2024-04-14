use bnum::types::U512;
use rustic_factors::commands::CommandMap;
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
    let cmd_map = CommandMap::default();
    let cmd = cmd_map.get(&method)?;
    Ok(cmd.run(&n))
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
        let cmap = CommandMap::default();
        for method in cmap.available_methods().split(", ") {
            assert!(run(vec![
                String::from("rustic_factors"),
                String::from(method),
                String::from("123")
            ])
            .is_ok());
        }
    }

    #[test]
    fn unsupported_method() {
        assert!(run(vec![
            String::from("rustic_factors"),
            String::from("unsupported method"),
            String::from("123")
        ])
        .is_err());
    }

    #[test]
    fn too_few_args() {
        assert_eq!(
            run(vec![String::from("rustic_factors")]).unwrap_err(),
            String::from("Usage: rustic_factors <algorithm> <number>")
        );
    }
}
