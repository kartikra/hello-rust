
mod variable;
mod scope;
mod safety;
pub mod control_loop;

pub fn run_demo(){
    variable::demo_variables();
    scope::demo_scope();
    safety::demo_memory_safety();
}



