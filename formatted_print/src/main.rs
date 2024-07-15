fn main(){
    //-----------------------------std::fmt--------------------------------------
    //1 format!: write formatted text to `String` and return this `String` value
    let text:String;
    text = format!("Try to write formatted text to String");
    println!("{}",text);

    //2 print!: 
    //WRONG: text = print!("Print the text to the console without a newline");
    print!("print the text to the console without newline");

    //3 println!:
    println!("Print the text to the console with a newline");

    //4 eprint!:
    eprint!("print the text to the standard error `io::stderr` without newline\n");

    //5 eprintln!:
    eprintln!("print the text to the standard error `io::stderr` with newline");

    //`std::fmt` contains many traits which govern the display of text.
    //The base form of two important ones:
    //`fmt::Debug`: Use the `{:?}` marker. Format text for debugging purposes
    //`fmt::Display`: Use the `{}` marker. Format text in a more elegant, user firendly fashion
    //      * Implementing the `fmt::Display` trait automatically implements the `ToString` trait 
    //        which allows us to convert the type to String
    //      * Here we used `fmt::Display`because the std provides implementation for these types.

    //-------------------------------Formatted print-------------------------------------------

    //1 print with arguments
    //1.1 `{}` automatically repalced with any arguments
    println!("{} days", 31);

    //NOTE: ONLY types that implement `fmt::Display` can be formatted with `{}`
    //User-defined types do not implement `fmt::Dispaly` by default
    //See 2.2 error checking2

    //1.2 Positional arguments can be used. the number specify which argument should be used at this position.
    //SHOULD PRINT: Alice, this is Bob. Bob, this is Alice.
    //Usually for using arguements multi times in a `println!` statements
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    
    //1.3 Can named arguments
    println!("{subject}{verb}{object}",
            object = " the lazy dog",
            subject = "the quick brown fox ",
            verb = " jumps over ");
    
    //1.4 Can format arguments with `:`
    //Usually used for outputting formatted integer numbers
    println!("Base 10:                {}",69420);//SHOULD PRINT: 69420
    println!("Base 2:                 {:b}",69420);//SHOULD PRINT: 1000011100101100
    println!("Base 8:                 {:0}",69420);//SHOULD PRINT: 207454
    println!("Base 16:                {:x}",69420);//SHOULD PRINT: 10f2c

    //1.5 Right-justify and left-justify with a specified width and filling
    println!("{number:>5}",number=1);//SHOULD PRINT:     1
    println!("{number:0>5}",number=1);////SHOULD PRINT: 00001
    println!("{number:<5}",number=1);//SHOULD PRINT: 1    
    println!("{number:0<5}",number=1);//SHOULD PRINT: 10000
    //Use named arguments in format specifier with `$`
    println!("{number:0>width$}",number=1,width=5);//SHOULD PRINT: 00001

    //1.6 (Usually used) Can capture the argument from a surrounding variable
    let number:f64 = 1.0;
    let width:usize = 5;
    println!("{number:0>width$}");

    //2.1 Error checking1
    // Rust even checks to make sure the correct number of arguments are used.
    // println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"

    //2.2 Error checking2
    //Disable `dead_code` which warn against unused module 
    // #[allow(dead_code)];
    // struct Structure(i32);
    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    //------------------------------Practice-----------------------------------
    //Problem(pasted): Add a println! macro call that prints: Pi is roughly 3.142 by 
    //controlling the number of decimal places shown. For the purposes of this 
    //exercise, use let pi = 3.141592 as an estimate for pi. 
    //(Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)
    let pi = 3.141592;
    println!("Pi is roughly {:.3}",pi);
}