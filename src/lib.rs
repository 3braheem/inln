use std::error::Error;
use structopt::StructOpt;
use std::fs;

#[derive(Debug,StructOpt)]
#[structopt(name = "inln", about = "A file parser to read line, word, or char count.")]
struct Opt {
    /// Find the file's line count 
    #[structopt(short = "l", long = "line")]
    lines: bool,
    /// Find the file's word count
    #[structopt(short = "w", long = "word")]
    words: bool,
    /// Find the file's character count
    #[structopt(short = "c", long = "char")]
    chars: bool,
    /// The file to run inln on
    #[structopt(name = "FILE", parse(from_str))]
    input: String,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Opt::from_args(); 
    let contents = fs::read_to_string(args.input)?;
    if args.lines {
        let res = inln_lines(&args.lines, &contents).unwrap();
        println!("The line count of the file is: {}", res);
    }
    if args.words {
        let res = inln_words(&args.words, &contents).unwrap();
        println!("The word count of the file is: {}", res);
    }
    if args.chars {
        let res = inln_chars(&args.chars, &contents).unwrap();
        println!("The character count of the file is: {}", res);
    } 
    Ok(())
}

pub fn inln_lines(required: &bool, contents: &str) -> Option<usize> {
    if *required {
       Some(contents
            .lines()
            .count()
       )   
    } else {
        None
    }
}

pub fn inln_words(required: &bool, contents: &str) -> Option<usize> {
    if *required {
        Some(contents
            .split_whitespace()
            .count()
        )    
    } else {
        None
    }
}

pub fn inln_chars(required: &bool, contents: &str) -> Option<usize> {
    if *required {
        Some(contents
            .replace("\n", "")
            .replace(" ", "")
            .chars()
            .count()
        )    
    } else {
        None
    }
}
