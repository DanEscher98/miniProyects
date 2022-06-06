use rand::prelude::*;
use std::fmt;

struct Chromosome<T> {
    length: u32,
    genes: Vec<T>,
    fitness: Fitness
}

struct Fitness { // Should be Ord
    value: i32
}



fn main() {
    println!("Hello.");
}

fn get_best() {
    print!("hi");
}


// struct Circle {
//     radius: i32,
//     center: (f32, f32)
// }
// 
// fn print_type<T>(_: &T) {
//     println!("Type {}", std::any::type_name::<T>());
// }
// 
// fn main() {
//     let mut x: u8;
//     println!("Hello world!");
//     let mut y: u16;
//     let mut rng = thread_rng();
//     for i in 0..10 {
//         x = random();
//         y = rng.gen_range(0..=i);
//         println!("x: {}, y: {}", x, y);
//     }
//     let arrows_iter = "➡⬈⬆⬉⬅⬋⬇⬊".chars();
//     println!("Lets go in this direction: {:?}",
//         arrows_iter.choose_multiple(&mut rng, 2));
//     //    arrows_iter.choose(&mut rng).unwrap());
// 
//     let mut nums = [1, 2, 3, 4, 5];
//     let v = vec![1, 2, 3, 4, 5, 6];
//     let sample: Vec<&u32> = v.iter().choose_multiple(&mut rng, 10);
//     println!("A sample: {:?}", sample);
//     print_type(&sample);
//     println!("Hi");
// 
//     nums.shuffle(&mut rng);
//     println!("I shuffled my {:?}", nums);
// }
