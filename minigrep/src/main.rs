use minigrep::Config;
use std::{env, process};

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = Config::build(&args).unwrap_or_else(|error| {
    //     eprintln!("There is a problem in parsing arguments: {error}");
    //     process::exit(1);
    // });
    // Refactor with iterators below

    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("There is a problem in parsing arguments: {error}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
