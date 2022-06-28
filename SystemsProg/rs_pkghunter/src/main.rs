use clap::Parser;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Parser)]
#[clap(
    author = "Danyiel Colin",
    version = "1.0",
    about = "A simple Package Hunter"
)]
struct Arguments {
    pkg_name: String,
    max_depth: usize,
}

fn get_arguments() {
    let args: Vec<_> = std::env::args().collect();
    let max_depth = if args.len() > 2 {
        args[2].parse().unwrap()
    } else {
        usize::MAX
    };
    println!("{:?}", count(&args[1], max_depth));
}

fn count(dir_name: &str, max_depth: usize) -> std::io::Result<usize> {
    let mut count = 0;
    // queue to store next dirs to explore
    let mut queue = VecDeque::new();
    // start with current directory
    queue.push_back((PathBuf::from("."), 0));
    loop {
        if queue.is_empty() {
            break;
        }
        let (path, crr_depth) = queue.pop_back().unwrap();
        if crr_depth > max_depth {
            continue;
        }
        // consider it only if it is a dictionary
        for dir in fs::read_dir(path)? {
            let dir = dir?;
            if dir.file_name() == dir_name {
                // we have a match, so stop exploring further
                count += 1;
            } else {
                // not a match so check its sub-dirs
                queue.push_back((dir.path(), crr_depth + 1));
            }
        }
    }
    return Ok(count);
}

fn main() {
    get_arguments();
}
