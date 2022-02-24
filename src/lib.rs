use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub action: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let action = args.next().ok_or("No command specified.")?;
        let filename = args.next().ok_or("No filename found.")?;
        Ok(Config {
            action,
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = inln_parse(&config.action, &contents);
    match config.action.as_str() {
        "l" => println!("The line count is: {}", results),
        "w" => println!("The word count is: {}", results),
        "c" => println!("The character count is: {}", results),
        _ => {
            eprintln!("Wrong arguments: Use l for lines, w for words, or c for characters");
            std::process::exit(1);
        }
    }
    Ok(())
}

pub fn inln_parse<'a>(action: &str, contents: &'a str) -> usize {
    match action {
        "l" => {
            contents
                .lines()
                .count()
        }
        "w" => {
            contents
                .split_whitespace()
                .count()
        }
        "c" => {
            contents
                .replace("\n", "")
                .replace(" ", "")
                .chars()
                .count()
        }
        _ => 0
    }
}
