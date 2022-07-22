use std::io;

pub fn check_temperature(temp: i32) {

    if temp > 30 {
        // curly braces are mandatory!
        println!("really hot outside!");
    } else if temp < 10 {
        println!("really cold, don't go out!");
    } else {
        println!("temperature is OK");
    }

    // if is an expression!
    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("today is {}", day);

    // 20+ hot, <20 cold
    println!(
        "it is {}",
        if temp > 20 {
            "hot"
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    );

    // Remember, if is an expression. All blocks must return same datatype
    println!(
        "it is {}",
        if temp > 20 {
            if temp > 30 {
                "very hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    );
}


pub(super) fn while_for_loops() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    // iter
    for element in a.iter() {
        println!("the value is: {element}");
    }

    // range stop at n - 1
    for number in (1..5).rev() {
        println!("{number}!");
    }

    // range inclusive of n
    for number in (1..=3).rev() {
        println!("{number}!");
    }
}


pub(super) fn guessing_game() {

    /*
    // infinite loop
    loop {
        println!("again!");
    }
    */
    println!("Guess the number!");

    // Do while loop with break
    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");
        if guess.to_string().starts_with("q")  {
            break;
        }
    }

}

