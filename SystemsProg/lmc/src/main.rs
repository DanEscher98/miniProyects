use clap::{Arg, App};
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use crate::lmc;

fn main() {
    let matches = App::new("Little Man Computer")
        .version("1.0")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .help("Indicates the file path")
            .takes_value(true))
        .get_matches();
    println!("Hello, world!");

    // If --file is present, read the values from a file
    let reader: Box<dyn BufRead> = match matches.value_of("file") {
        Some(file_path) => {
            let path = std::path::PathBuf::from(file_path);
            let file = File::open(path).expect("File not found");
            Box::new(BufReader::new(file))
        },
        None => Box::new(BufReader::new(io::stdin()))
    };

    // Fill mailbox
    let mut mailbox: [i32; 100] = [0; 100];
    for (i, value) in reader.lines().enumerate() {
        mailbox[i] = value.unwrap()
            .trim()
            .parse()
            .expect(&format!("NaN on line {}", i + 1));
        if i == 99 {
            break;
        }
    }
}
