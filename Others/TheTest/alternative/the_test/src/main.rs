use std::iter::Iterator;

use rand::{self, Rng};

fn main() {
    let vec = init_vec(40, false);
    println!("Vector = {:?}", vec);
}

// fn init_rand_vec(size: u64) -> Vec<u64> {
//     let mut rng = rand::thread_rng();
//     (0..size).map(|_| rng.gen_range(0..size)).collect()
// }

fn init_vec(size: u64, _ordered: bool) -> Vec<u64> {
    let upper = size / 2;
    RandInterator::new(upper, 0).take(size as usize).collect()
}

struct RandInterator {
    range: u64,
    scale: u64,
    _ordered: bool,
}

impl RandInterator {
    fn new(range: u64, scale: u64) -> Self {
        RandInterator {
            range,
            scale,
            _ordered: false,
        }
    }
}

/// ```
/// let var
/// let x
/// ```
impl Iterator for RandInterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();
        Some(rng.gen_range(self.scale..(self.range + self.scale)))
    }
}

fn dumb() -> Box<dyn Iterator<Item = u64>> {
    Box::new(RandInterator::new(1, 2))
}
