use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let input: Vec<String> = env::args().collect();
    let string = &input[1];
    let filename = &input[2];
    println!("Searching: {:?}", string);
    println!("In file: {:?}", filename);

    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong");
    println!("With text: \n{:?}", contents);
}
