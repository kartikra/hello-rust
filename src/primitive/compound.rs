

pub(super) fn demo_tuple() {

    // Tuple 
    let info = (1, 3.3, 999);
    println!("Tupe values are 0={} 1={} 2={}", info.0, info.1, info.2);

    let (jets, fuel, ammo) =  info; // errity = 3
    println!("Tupe values are jets={} fuel={} ammo={}", jets, fuel, ammo);


}

// Array
pub(super) fn demo_array() {
    let a1 = [1, 2, 3];
    println!("Array a1 = {:?}", a1);

    let buf: [u8; 3]  = [1, 2, 3];  // specify length after ;
    println!("Array buf = {:?}", buf);

    // Arrays are limited to length of 32. Use Vectors

}

// String
pub(super) fn demo_string() {

    // new string
    // let mut s = String::new();

    // let data = "initial contents";
    // let s = data.to_string();

    // the method also works on a literal directly:
    // let s = "initial contents".to_string();
    // let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    println!("s1 is {}", s1);

    // String Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // println!("s1 is {}", s1); --> this will give an error
    println!("s3 is {}", s3);

    // String Indexing
        // this will give an error. Can't Index!!
        // let s1 = String::from("hello");
        // let h = s1[0]; 


    // String Iteration
    /*
    word.bytes()  --> iterate over bytes            works fine for english letters (that overlaps with ascii)
    word.chars()  --> iterate over unicode scalars
    */
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Split words in a sentence
    let databases = "MySQL\nSQL_Server\tOracle\nPostgreSQL\n".split_whitespace();
    for db in databases {
          println!("{}", db)
    }

}

