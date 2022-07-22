mod scalar;
mod compound;


pub fn print_characters(){
    scalar::integer_types();
    scalar::character_types();
}

pub fn demo_compound_types() {
    println!("\n Demo Tuple");
    compound::demo_tuple();
    println!("\n Demo Array");
    compound::demo_array();
    println!("\n Demo String");
    compound::demo_string();
}