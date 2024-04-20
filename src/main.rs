use rustic_factors::cli;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match cli::run(&args) {
        Ok(result) => println!("{result}"),
        Err(cli::Error::ParseIntErr) => eprintln!("Please provide a number in range [0, 2⁵¹²)"),
        Err(cli::Error::IncorrectNumArgs) => eprintln!("Usage: {} <command> <number>", args[0]),
        Err(cli::Error::CommandNotFound(commands)) => {
            eprintln!("Unknown command. Available options: {commands}.")
        }
    }
}
