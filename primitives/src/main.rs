
fn main(){
    
    //Variables can be type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0;//Regular annotation
    let an_integer = 5i32;//Suffix annotation

    //Or a default will be used
    let default_float = 3.0;//f64
    let default_integer = 7;//i32

    //A type can also be inferred from context
    let mut inferred_type = 12;//i64. Ifferred from another line
    inferred_type = 4233333i64;

    let mut mutable = 12;//Mutable i32
    mutable = 21;

    //mutable = true;//Error: Type of a variable can't be changed

    let mutable = true;//Shadowing
}