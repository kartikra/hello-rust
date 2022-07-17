
use hello_rust::calculate_weight;
use rand::{thread_rng, Rng};
mod basics;

// use std::collections::HashMap;
// search https://crates.io/ for package


fn main() {
    println!("Hello, world!");

    // calling a module
    basics::run_demo();

    // Public Function from within a module
    basics::control_loop::demo_conditions();

    // Using lib.rs
    println!("{}", calculate_weight(16.0, 2.5));
    let x = thread_rng().gen_range(0..100);
    println!("{}", x);


}