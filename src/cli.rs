use crate::commands::CommandMap;
use bnum::types::U512;

pub fn run(args: &[String]) -> Result<String, Error> {
    let input = ParsedInput::try_from(args)?;
    println!("{}...", &input);
    let cmd_map = CommandMap::default();
    match cmd_map.get(&input.command_name) {
        Some(cmd) => Ok(cmd.run(&input.number).to_string()),
        None => Err(Error::CommandNotFound(cmd_map.available_commands())),
    }
}

struct ParsedInput {
    number: U512,
    digit_len: usize,
    command_name: String,
}

impl TryFrom<&[String]> for ParsedInput {
    type Error = Error;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        let slice: &[String; 3] = value.try_into().map_err(|_| Error::IncorrectNumArgs)?;
        Ok(ParsedInput {
            number: slice[2].parse().map_err(|_| Error::ParseIntErr)?,
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
pub enum Error {
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
            assert!(run(&[
                String::from("rustic_factors"),
                String::from(command),
                String::from("123")
            ])
            .is_ok());
        }
    }

    #[test]
    fn unsupported_command() {
        match run(&[
            String::from("rustic_factors"),
            String::from("unsupported command"),
            String::from("123"),
        ]) {
            Err(Error::CommandNotFound(_)) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn too_few_args() {
        assert_eq!(
            run(&[String::from("rustic_factors")]).unwrap_err(),
            Error::IncorrectNumArgs,
        );
    }
}
