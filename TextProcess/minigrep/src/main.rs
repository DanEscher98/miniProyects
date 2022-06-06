extern crate minigrep;
use std::{env, process};
use minigrep::UsrInput;

fn main() {
    let input: Vec<String> = env::args().collect();
    let values = UsrInput::new(&input).unwrap_or_else(|err| {
        eprintln!("Error at argument parsing: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(values) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
