use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Danyiel Sánchez <danyiel5978@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .allow_invalid_utf8(true)
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .help("Do not print new line")
                .takes_value(false)
                .short('n'),
        )
        .get_matches();
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
