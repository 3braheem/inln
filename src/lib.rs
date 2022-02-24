use std::env;
use std::error::Error;
use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;

#[derive(Debug,StructOpt)]
#[structopt(name = "inln", about = "A file parser to read line, word, or char count.")]
struct Opt {
    /// Find the file's line count 
    #[structopt(short = "l", long = "line")]
    line: bool,
    /// Find the file's word count
    #[structopt(short = "w", long = "word")]
    word: bool,
    /// Find the file's character count
    #[structopt(short = "c", long = "char")]
    chars: bool,
    /// The file to run inln on
    #[strcutopt(name = "FILE", parse(from_os_str))]
    input: String,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Opt::from_args(); 
    let contents = fs::read_to_string(args.input)?;
    let result = match args {
        Opt { lines } => inln_lines(&args, &contents),
        Opt { words } => inln_words(&args, &contents),
        Opt { chars } => inln_chars(&args, &contents),
    };
    
    Ok(())
}

pub fn inln_lines(args: &Opt, contents: &str) -> usize {
    if args.lines {
        contents
            .lines()
            .count()
    }
}

pub fn inln_words(args: &Opt, contents: &str) -> usize {
    if args.words {
        contents
            .split_whitespace()
            .count()
    }
}

pub fn inln_chars(args: &Opt, contents: &str) -> usize {
    if args.chars {
        contents
            .replace("\n", "")
            .replace(" ", "")
            .chars()
            .count()
    }
}
