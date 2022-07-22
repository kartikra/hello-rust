// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
//  trait Bite...


// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).
//
// #[derive(Debug)] // include this line right before your struct definition
// struct Grapes...


// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of this file.
//
// impl Bite for...



pub fn demo_trait() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    //
    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)
    //
    let mut carrot = Carrot { percent_left: 100.0 };
    bunny_nibbles(&mut carrot, &mut 3);
    println!("Bunny carrot nibbles after {} bites : {:?}", 3, carrot);

    let mut grapes = Grapes { amount_left: 100 };
    bunny_nibbles(&mut grapes, &mut 10);
    println!("Bunny grapes nibbles after {} bites : {:?}", 10, grapes);

}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Grapes{
    amount_left: i32,
}

trait Bite {
	fn bite(self: &mut Self);
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.amount_left -= 1;
    }
}


#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}


impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

fn bunny_nibbles<T: Bite>(item: &mut T, no_of_bites: &mut i32) {
    while *no_of_bites > 0 {
        item.bite();
        *no_of_bites -= 1;
    }
}



/*   STRUCT & TRAIT Example

struct RedFox {
	enemy: bool,
	life: u8,
}


// initialize
let fox = RedFox {
	enemy: true,
	lise: 70,
}

OR

// Preferred route. Use associcated functions
impl RedFox {
	fn new() -> Self {
		Self {
			enemy: true,
			life: 70,
		}
	}

	fn function()
	// methods
	fn move(self)
	fn borrow(&self)
	fn mut_borrow(&mut self)

}


let fox = Redfox::new();
let life_left = fox.life;
fox.enemy = false;
fox.some_method();


*/