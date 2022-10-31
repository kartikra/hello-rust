mod scalar;
mod compound;
mod collection;
use collection::{Color,DispenserItem::*};


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

pub fn demo_collections() {
    println!("\n Demo Vector");
    collection::demo_vector();
    println!("\n Demo Hashmap");
    collection::demo_hashmap();

    println!("\n Demo Enum");

    let color = Color::Red;

    let item = Empty;
    item.display();

    let item = Ammo(4);
    item.display();

    let item = Things("hat".to_string(), 7);
    item.display();


}