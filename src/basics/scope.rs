

pub(super) fn demo_scope() {

    let x = 5;
    {
        let x = 50;
        let y = 99;
        println!("{}, {}", x, y)
    }
    println!("{}", x);

    // println!("{}, {}", x, y) // this will fail y not in scope

    // Shadow same symbol
    let mut z = 7;
    let z = z;
    println!("{}", z);

    let meme = "cowbell";
    let meme = meme.to_uppercase();
    println!("{}", meme);


}
