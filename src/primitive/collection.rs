use std::collections::{HashMap};



// Vector
pub(super) fn demo_vector() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);
    let x = v.pop();
    println!("First {} and Last {}", v[0], x.unwrap_or(0));

    let v1 = vec![2,4,6];

    for i in v1.iter() {
        println!("{}", i);
    }

}


// Hashmap
pub(super) fn demo_hashmap() {
    let mut h: HashMap<&str, i32> = HashMap::new();
    h.insert("A", 1);
    h.insert("B", 2);

    for (key, value) in &h {
        println!("{} / {}", key, value);
    }

    let get_a = h.get(&"A").unwrap();
    println!("{}", get_a);

    let have_b = h.remove(&"B").unwrap();
    println!("{}", have_b);

    for (key, value) in &h {
        println!("{} / {}", key, value);
    }
}

/*
Other collections 
VecDeque - ring dequeue to implement double ended queue. add or remove items from front or back 
Hashset - remove duplicates
LinkedList - quick at adding or deleting an arbitrary number
BinaryHeap - always pops out the max value
BtreeMap - HashMap variation. Keys always sorted
BTreeSet - Hashset variation. Keys always sorted
*/ 


// Enum
pub(super) enum Color {
    Red,
    // Green,
    // Blue,
}


// let color = Color::Red;

// Enum
pub(super) enum DispenserItem {
    Empty,
    Ammo(u8),
    Things(String, i32),    // Tuple
    // Place {x: i32, y:i32},   // Annonymous Struct
}

// Initialize Many ways but can only be 1 at a time
// use DispenserItem::*;
// let item = Place(x: 24, y: 48);


// functions & methods wiuth Enum
impl DispenserItem{
    pub(super) fn display(&self){}
}

// Generic
/* 
// if null value needed
pub(super) enum Option<T> {
    Some(T),
    None,
}

#[must_use]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

*/