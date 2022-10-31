
use hello_rust::calculate_weight;  // from lib.rs
use rand::{thread_rng, Rng};  // external package
use std::io;
mod advanced;
mod basics;
mod primitive;
mod control_flow;
mod struct_trait;

// use std::collections::HashMap;
// search https://crates.io/ for package


fn main() {
    println!("Welcome, hello-rust training!");



        // Do while loop with break
        loop {

            println!("\nMake your selection. Press anything else to exit\n");
            println!("1 - learn basics");
            println!("2 - learn to use lib.rs");
            println!("3 - scalar data-types");
            println!("4 - compound data-types(tuple,array,string)");
            println!("5 - control flow examples");
            println!("6 - ownership examples");
            println!("7 - struct & trait examples");
            println!("8 - collections(vector,hashmap,enum)");
            println!("9 - File Reader");
            println!("10 - Closures");

            let mut selection = String::new();
    
            io::stdin()
                .read_line(&mut selection)
                .expect("Failed to read line");
    

            match selection.trim() {
                "1" =>  {
                    // calling a module
                    basics::run_demo();
                },

                "2" => {
                    // Using lib.rs and package from crate.io
                    println!("{}", calculate_weight(16.0, 2.5));
                    let x = thread_rng().gen_range(0..100);
                    println!("{}", x);
                },

                "3" => {
                    // Scalar Types
                    primitive::print_characters();
                },

                "4" => {
                    // Compound Types
                    primitive::demo_compound_types();
                },

                "5" => {
                    // Public Function from within a module
                    control_flow::conditions::check_temperature(35);
                    control_flow::conditions::check_temperature(5);
                    control_flow::conditions::check_temperature(25);
    
                    control_flow::run_demo();
                },

                "6" => {
                    // This fancy stuff either gets the first argument as a String, or prints
                    // usage and exits if an argument was not supplied to the program.
                    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
                        println!("Please supply an argument to this program.");
                        std::process::exit(-1);
                    });
                    basics::ownership_example(&mut arg);
                },

                "7" =>  {
                    struct_trait::demo_trait();
                },

                "8" => {
                    // Collections
                    primitive::demo_collections();
                },

                "9" => {
                    // IO
                    basics::read_file();
                },

                "10" => {
                    // Closures
                    advanced::demo_closure();
                },

                _ => break,
            }
        }

}

// 11

