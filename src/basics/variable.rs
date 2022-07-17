pub(super) fn demo_variables(){

    // Constant
    const WRAP_FACTOR:f64 = 9.9;
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;

    // can be a global constant. constants in,ine to complie time and very fast

    let mut rabbit = 2;
    let (bunnies, carrot) = (36, 24);
    /* *Variables Immutable by default. Reason - 
    1. Safety (few bugs), 
    2. Concurrency(parallel threads can't change) and 
    3. Speed (knowing value won't change helps  otpimize run times)
    */
    rabbit += 5;

    // bunnies = 34;  // this wil be an error


    println!("{}, {}, {}", bunnies, carrot, rabbit);
    println!("{}", WRAP_FACTOR);


    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("Firing {} of my {} missiles...", ready, missiles);

}
