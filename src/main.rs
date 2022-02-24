use inln::Config;
use std::env;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Argument parsing error: {}", err);
        std::process::exit(1);
    });
    if let Err(e) = inln::run(config) {
        eprintln!("There was an error: {}", e);
        std::process::exit(1);
    }
}
