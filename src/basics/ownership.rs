

pub(super) fn demo_ownership(){

    let s1 = String::from("abc");
    // do_stuff(s1);
    // println!("{}", s1);  // Error since s1 has been consumed and moved by function

    // workaround is to pass variables but this is not too convenient
    let mut s2  = String::from("abc");
    s2 = do_stuff_2(s2);

    // So lets use reference. Pass reference to s2. 
    // reference (and not the value) get borrowed
    // reference to this string goes out of scope and gets dropped after function execution. Borrowing ends. Value of s1 never moved
    // Under the hood, pointer is created by Rust and deleted
    // Pointers are managed using rule called lifetime. References must always be valid
    do_stuff_3(&s1);
    println!("{}", s1);  // abc

    // Mutable string example
    do_stuff_4(&mut s2);
    println!("{}", s2);  // Hi, abc

}


fn do_stuff_2(s: String) -> String{
    s
}

fn do_stuff_3(s: &String) {
    // do stuff
    println!("{}", s);  // abc
}

fn do_stuff_4(s: &mut String){
    // do stuff
    s.insert_str(0, "Hi, ");

    /*
    . operator derefrences reference to the value
    (*s).insert_str()
    *s = String::from("Replacement")
    */

}


pub(super) fn demo_references(arg: &mut String) {

    // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
    // prints whether the contents of the String is plural or singular. Then uncomment and run this
    // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
    // String reference
    //
    inspect(&arg);

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    //
    change(arg);
    println!("I have many {}", arg);

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    //
    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    };

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "add" that takes *references* to two integer arguments,
    // dereferences them and adds them together, and returns the result.
    //
    println!("1 + 2 = {}, even via references", add(&1, &2));

}

fn inspect(s: &String){
    println!("{}", s);  
}

fn change(s: &mut String){
    s.push_str("s");  
}

fn eat(s: &mut String) -> bool{
    if s.starts_with("b") && s.contains("a") {
        true
    } else {
        false
    } 
}

fn add(i: &i32, j: &i32) -> i32 {
    i + j
}