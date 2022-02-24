fn main() {
    if let Err(e) = inln::run() {
        eprintln!("inln reported an error: {}", e);
        std::process::exit(1);
    };
}
