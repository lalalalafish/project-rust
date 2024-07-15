//Review the basic concepts of rust language
//Part0-Comments
//Single-line comment
/* BLock comment */
// ///Doc comments
// //! Doc comments
fn main() {
    println!("Part1-Variable");
    println!("1.1-Mut and Inmut");
    let mut x = 5;
    println!("The value of x is: {x}");
    x=6;
    println!("The value of x is: {x}");

    println!("\n1-2-Constants");
    const CONST_NAME:i32 = 60 * 60 * 3;
    println!("The value of CONST_NAME is: {CONST_NAME}");

    println!("\n1.3-Shadowing");
    let spaces = "    ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
    println!("----------------------------------------------------------------------");
    println!("Part2-Data Type");
    println!("2.1-Scalar Types");
    println!("2.1.1-Integer Types");
    println!("i8-i16-i32(default)-i64-i128-isize");
    println!("u8-u16-u32(default)-u64-u128-usize");
    let x1:i8 = 127;
    let x2:i16 = 32767;
    let x3:i32 = 2147483647;
    println!("The value of x1(i8): {x1}");
    println!("The value of x2(i16): {x2}");
    println!("The value of x3(i32): {x3}");
    println!("\n2.1.1.1-Integer literals");
    let dec_x = 1_000;
    let hex_x = 0x12AA;
    let oct_x = 0o123;
    let bin_x = 0b1111_0000;
    let byte_x = b'A';
    println!("The value of dec_x: {dec_x}");
    println!("The value of hex_x: {hex_x}");
    println!("The value of oct_x: {oct_x}");
    println!("The value of bin_x: {bin_x}");
    println!("The value of byte_x: {byte_x}");
    println!("2.1.2-Floating-Point Types");
    let float_x:f64 = 12.12121212;
    println!("The value float_x is {float_x}");
    println!("2.1.3-Boolean Types");
    let f = false;
    let t: bool = true;
    println!("The value of f: {f}");
    println!("The value of t: {t}");
    println!("2.1.4-Character Type");
    let c='Z';
    println!("The value of c: {c}");
    println!("2.2-Compound Types");
    println!("2.2.1-Tuple Type");
    let tup:(i32,f64,u8)=(500,6.4,1);
    let (x,y,z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {five_hundred}");
    println!("2.2.2-Array Type");
    let a = [1,2,3,4,5];
    let b = [3;5];
    let c: [i32;3] = [1,2,3];
    let element_a = a[2];
    let element_b = b[0];
    let element_c = c[2];
    println!("The value of element_a: {element_a}");
    println!("The value of element_b: {element_b}");
    println!("The value of element_c: {element_c}");
    println!("2.2.3-Vector type(remaining)");
    println!("----------------------------------------------------------------------");
    println!("Part3-Function");
    let return_value = another_function(66);
    println!("The return value of another_function: {return_value}");
    //KEY: Statement and Expression
    println!("----------------------------------------------------------------------");
    println!("Part4-Control FLow");
    println!("4.1-If Expressions");
    let number = 11;
    println!("The value of number: {number}");
    if number < 5{
        println!("number < 5");
    }else if number < 10{
        println!("5 < number < 10");
    }else{
        println!("number > 10")
    }

    let number = if number < 10 {12} else {8};
    println!("The value of number: {number}");

    println!("4.2-Loop");
    let mut counter =0;
    let result = loop{
        println!("The value of counter: {counter}");
        counter += 1;
        if counter == 5{
            break counter*2;
        }
    };
    println!("The value of counter: {counter}");
    println!("The value of result: {result}");
    println!("4.3-Conditional Loop-While");
    let mut number = 5;
    while number != 0{
        print!("{number} ");
        number -=1;
    }
    println!("4.3-Conditional Loop-for");
    let a = [10,20,30,40,50];
    for element in a{
        print!("{element} ");
    }
    println!();

    for number in (1..4).rev(){
        print!("{number} ");
    }
    println!("\n--------------------------------END--------------------------------");
}

fn another_function(x:i32)->i32{
    println!("Call the another_function");
    println!("The value of argument x: {x}");
    1234
}
