//extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess the number!");

    loop {
        println!("Please input a number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    let addr_1 = IPAddr::IPv4(127, 0, 0, 1);
    //let addr_2 = IPAddr::IPv6(String::from("::1"));
    match addr_1 {
        IPAddr::IPv4(a, b, c, d)
            => println!("{}.{}.{}.{}", a, b, c, d),
        _   => println!("It's not and IPv4 address")
    }

    if let IPAddr::IPv4(127, 0, 0, 1) = addr_1 {
        println!("localhost");
    }

    let var_1: Quizas<u32> = Quizas::Nada;
    match var_1 {
        Quizas::Algun(n) => println!("Un numero {}", n),
        Quizas::Nada     => println!("No es número")
    }
}

enum IPAddr {
    IPv4(u8, u8, u8, u8),
    IPv6(String)
}

enum Quizas<T> {
    Algun(T),
    Nada
}
