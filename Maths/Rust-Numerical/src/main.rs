#[warn(dead_code)]
//use std::io::BufRead;
// use console::Term;
// use dialoguer::Input;

fn main() {
    let vec     = obtener_vector();
    let prom    = promedio(vec);
    clasifica(vec, prom);
}
