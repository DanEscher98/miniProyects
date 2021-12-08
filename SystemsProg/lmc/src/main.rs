extern crate lmc;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};

fn main() {
    let matches = App::new("Little Man Computer")
        .version("1.0")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("Indicates the file path")
                .takes_value(true),
        )
        .get_matches();

    // If --file is present, read the values from a file
    let reader: Box<dyn BufRead> = match matches.value_of("file") {
        Some(file_path) => {
            let path = std::path::PathBuf::from(file_path);
            let file = File::open(path).expect("File not found");
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin())),
    };

    // Fill mailbox
    let mut mailbox: [i32; 100] = [0; 100];
    for (i, value) in reader.lines().enumerate() {
        mailbox[i] = value
            .unwrap()
            .trim()
            .parse()
            .expect(&format!("NaN on line {}", i + 1));
        if i == 99 {
            break;
        }
    }

    lmc::compute(mailbox);
}
