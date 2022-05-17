use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("Danyiel Colin <danyiel5978@gmail.com>")
        .about("Rusty Head")
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
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Set the desired number lines")
                .conflicts_with("bytes")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .help("Set the desired bytes number"),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count: {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count: {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
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
