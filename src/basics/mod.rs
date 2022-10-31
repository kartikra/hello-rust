
mod variable;
mod scope;
mod safety;
mod ownership;
mod io;


fn area_of(width: i32, height: i32) -> i32 {
    width * height
}
fn volume_of(width: i32, height: i32, depth: i32) -> i32 {
    width * height * depth
}

pub fn run_demo(){
    variable::demo_variables();
    scope::demo_scope();
    safety::demo_memory_safety();

    // Use of Private Functions
    let area = area_of(5, 4);
    println!("Area is {}", area);
    let volume = volume_of(5, 4, 10);
    println!("Volume is {}", volume);
}

pub fn ownership_example(arg: &mut String){
    ownership::demo_ownership();
    ownership::demo_references(arg);
}

pub fn read_file() {
    io::read_file("foo");
    io::read_file("/Users/kartik/Documents/RustLand/hello-rust/src/basics/poem.txt");
    let contents = io::read_file_vec("/Users/kartik/Documents/RustLand/hello-rust/src/basics/poem.txt").expect("error");
    for line in contents.iter(){
        println!("{}", line);
    }

}