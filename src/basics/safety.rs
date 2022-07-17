

pub(super) fn demo_memory_safety() {

    let enigma: i32;
    // println!("{}", enigma);  // doesn't compile since variable not initalized

    if true{
        enigma = 45;
    }
    // println!("{}", enigma); // Still a problem since compiler still doesn't know if enigma will be initalized at run time
    else  {
        enigma = 0;
    }
    println!("{}", enigma);

    /*
    C Example
    gcc src/primitive/memory_safety.c -o target/memory_safety
    Undefined behaviour
    */

}
