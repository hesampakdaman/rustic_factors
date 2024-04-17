use bnum::types::U512;
use rustic_factors::commands::CommandMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match cli(&args) {
        Ok(result) => println!("{result}"),
        Err(CliErr::ParseIntErr) => eprintln!("Please provide a number in range [0, 2⁵¹²)"),
        Err(CliErr::IncorrectNumArgs) => eprintln!("Usage: {} <command> <number>", args[0]),
        Err(CliErr::CommandNotFound(commands)) => {
            eprintln!("Unknown command. Available options: {commands}.")
        }
    }
}

fn cli(args: &[String]) -> Result<String, CliErr> {
    if args.len() != 3 {
        return Err(CliErr::IncorrectNumArgs);
    }
    let input = parse(args)?;
    let cmd_map = CommandMap::default();
    match cmd_map.get(&input.command_name) {
        Some(cmd) => Ok(cmd.run(&input.number)),
        None => Err(CliErr::CommandNotFound(cmd_map.available_commands())),
    }
}

fn parse(args: &[String]) -> Result<ParsedInput, CliErr> {
    let command_name = String::from(&args[1]);
    let n: U512 = args[2].parse().map_err(|_| CliErr::ParseIntErr)?;
    Ok(ParsedInput {
        number: n,
        command_name,
    })
}

struct ParsedInput {
    number: U512,
    command_name: String,
}

#[derive(PartialEq, Debug)]
enum CliErr {
    CommandNotFound(String),
    ParseIntErr,
    IncorrectNumArgs,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_cases() {
        let cmap = CommandMap::default();
        for command in cmap.available_commands().split(", ") {
            assert!(cli(&[
                String::from("rustic_factors"),
                String::from(command),
                String::from("123")
            ])
            .is_ok());
        }
    }

    #[test]
    fn unsupported_command() {
        match cli(&[
            String::from("rustic_factors"),
            String::from("unsupported command"),
            String::from("123"),
        ]) {
            Err(CliErr::CommandNotFound(_)) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn too_few_args() {
        assert_eq!(
            cli(&[String::from("rustic_factors")]).unwrap_err(),
            CliErr::IncorrectNumArgs,
        );
    }
}
