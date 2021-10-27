#[warn(dead_code)]
use mathru::elementary::Trigonometry as Trig;

fn main() {
    println!("Hello, world!");
    println!("{}", sum_inner_lattice(5, false));
}

fn sum_inner_lattice(n: i64, unit_c: bool) -> f32 {
    let mut ans: f32    = 0.0;
    let theta           = std::f32::consts::PI;
    let r: f32          = if unit_c {1.0} else {Trig::csc(theta) / 2.0};
    // if unit_c.unwrap_or(true) {1} else {csc(theta)/2};
    for i in 1..=n/2 {
        ans += 2.0*r*(theta * i as f32).sin();
    }
    ans -= (n as f32 * r/2.0) * if n & 1 == 0 {1.0} else {0.0};
    return ans * n as f32;
}
