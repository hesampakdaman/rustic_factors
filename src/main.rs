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
    let input = ParsedInput::try_from(args)?;
    let cmd_map = CommandMap::default();
    match cmd_map.get(&input.command_name) {
        Some(cmd) => Ok(format!("{}\n{}", input, cmd.run(&input.number))),
        None => Err(CliErr::CommandNotFound(cmd_map.available_commands())),
    }
}

struct ParsedInput {
    number: U512,
    digit_len: usize,
    command_name: String,
}

impl TryFrom<&[String]> for ParsedInput {
    type Error = CliErr;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        let slice: &[String; 3] = value.try_into().map_err(|_| CliErr::IncorrectNumArgs)?;
        Ok(ParsedInput {
            number: slice[2].parse().map_err(|_| CliErr::ParseIntErr)?,
            digit_len: slice[2].len(),
            command_name: slice[1].to_string(),
        })
    }
}

impl std::fmt::Display for ParsedInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Using command {} with number {} ({} digits)",
            self.command_name, self.number, self.digit_len
        )
    }
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
