fn used_function(){}

#[allow(dead_code)]//an attribute that disables the `dead_code` lint
fn unused_function(){}

#[allow(dead_code)]//PRACTICE
fn noisy_unused_function(){}//Another unused function for practice

fn main(){
    used_function();
}