fn collatz(mut num: u32) -> usize {
    let mut steps: usize = 0;
    while num > 1 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = 3 * num + 1;
        }
        steps += 1;
    }
    steps
}

fn main() {
    let n: u32 = 5;
    println!("collatz({}): {}", n, collatz(n));
}
