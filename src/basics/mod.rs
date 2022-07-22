
mod variable;
mod scope;
mod safety;
mod ownership;


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

