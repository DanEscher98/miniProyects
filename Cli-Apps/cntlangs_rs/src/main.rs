use clap::Parser;
use cntlangs::{classify_files, cli::Arguments};
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let args = Arguments::parse();
    let path = {
        match &args.path {
            Some(pathname) => {
                let mut path_buf = PathBuf::new();
                path_buf.push(Path::new(&pathname));
                path_buf
            }
            None => env::current_dir().unwrap(),
        }
    };
    let _file_data = classify_files(&path);
    // format_output(file_data);
    dbg!(args);
}
