mod closures;


pub fn demo_closure(){

    // Annonymous function
    let add = |x, y| { x + y };
    let c = add(1, 2);
    println!("{}", c);

    // || {x + y} // valid closure
    // || {}  // still valid closure

    let s = "my mine".to_string();
    let f = || {
        println!("{}", s)   // borrows reference to s. However compiler won't let us send this to another thread
    };
    f();

    // move forces closure to move any variable it references into itself and take ownership of it
    // s is owned by the cosure and will be owned as long as the closure itself goes out of scope
    let f = move || {
        println!("{}", s)   // borrows reference to s. However compiler won't let us send this to another thread
    };
    f();

    // functional style programming
    let v = vec![2, 4, 6];

    /*
        let sum = v.iter()
                    .map(|x| x * 3)
                    .filter(|x| *x > 10)
                    .fold(0, |acc, x| acc + x);  // setting initial sum to 0
    */

    let sum = v.iter()
                    .map(|x| x * 3)
                    .filter(|x| *x > 10)
                    .fold(0, |acc, x| acc + x);
    println!("Sum of vector elements with 3*x > 10 comes to {}", sum);


}



/*
use std::thread;

fn main() {

    let handle = thread::spawn( move || {
            // do stuff in a child thread
    });

    // do stuff simaltaneously in the main thread;

    // wait until thread has exited
    handle.join().unwrap();

}

// thread is heavy and there is cost to swtiching context. Use async await instead of thread if waiting for IO

*/