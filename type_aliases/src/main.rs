#[allow(dead_code)]

enum VerVerboseEnumOfThingsToDoWithNumbers{
    Add,
    Subtract,
}

//Type alias creation
type Operations = VerVerboseEnumOfThingsToDoWithNumbers;

fn main(){
    #[allow(unused_variables)]
    let x = Operations::Add;
}