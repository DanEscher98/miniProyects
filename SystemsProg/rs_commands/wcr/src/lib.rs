use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("Danyiel Colin <danyiel5978@gmail.com>")
        .about("Rusty Word Counter")
        .arg(
            Arg::new("files")
                .allow_invalid_utf8(true)
                .value_name("FILE")
                .help("Input file(s)")
                .multiple_values(true)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .help("print the newline counts")
                .conflicts_with("bytes")
                .takes_value(false),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .help("print the words count")
                .takes_value(false),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("print the bytes count")
                .takes_value(false),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .help("print the characters count")
                .takes_value(false)
                .conflicts_with("bytes"),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let mut chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
        chars = false;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    fn print_file(mut file: Box<dyn BufRead>, config: &Config) {
        if let Some(num_bytes) = config.bytes {
            let mut handle = file.take(num_bytes as u64);
            let mut buffer = vec![0; num_bytes];
            let bytes_read = handle.read(&mut buffer).unwrap();
            print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
        } else {
            let mut line = String::new();
            for _ in 0..config.lines {
                let bytes = file.read_line(&mut line).unwrap();
                if bytes == 0 {
                    break;
                }
                print!("{}", line);
                line.clear();
            }
        }
    }
    if config.files.len() == 1 {
        let filename = &config.files[0];
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => print_file(file, &config),
        }
    } else {
        for filename in config.files.iter() {
            match open(&filename) {
                Err(err) => eprintln!("Failed to open {}: {}", filename, err),
                Ok(file) => {
                    println!("==>{}<==", &filename);
                    print_file(file, &config);
                }
            }
            println!();
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}
