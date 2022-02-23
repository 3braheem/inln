use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = args.next().ok_or("No query specified.")?;
        let filename = args.next().ok_or("No filename found.")?;
        Ok(Config {
            query,
            filename,
        })
    }
}
