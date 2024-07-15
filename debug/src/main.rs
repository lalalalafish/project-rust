#[derive(Debug)]//Derive the fmt::Debug implementation for `Structure`
struct Structure(i32);//Structure: a structure which contains a single `i32`

#[derive(Debug)]
struct Deep(Structure);//Deep: a structure which contains a structure `Structure`

fn main() {
    //Printing with {:?}
    println!("{:?} months in a year",12);
    println!("{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
            actor="actor's");

    //`Structure` is printable
    println!("Now {:?} will print!", Structure(3));

    //The problem with `derive` is there is no control over how the results look
    println!("Now {:?} will print!", Deep(Structure(7)));
}
