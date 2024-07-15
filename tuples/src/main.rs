//Tuples can be used as function arguments and as return values
fn reverse(pair: (i32,bool))->(bool,i32){
    let (int_param, bool_param) = pair;//Destructure
    (bool_param,int_param)
}


#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);

fn main(){
    //A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                    -1i8, -2i16, -3i32, -4i64,
                     0.1f32, 0.2f64,
                     'a', true);

    //Tuple indexing:
    println!("Long tuple first value is {}", long_tuple.0);
    println!("Long tuple second value is {}", long_tuple.1);

    //Tuples can be tuple members
    let tuple_of_tuples = ((1u8,2u16,2u32),(4u64,-1i8),-2i16);

    let long_tuple2 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("Too long tuple: {:?}", long_tuple2);
    // [!!!]But long Tuples (more than 12 elements) cannot be printed.
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);

    let pair = (1,true);
    println!("Pair is {:?}",pair);
    println!("The reserve pair is {:?}",reverse(pair));

    //[!!!]
    println!("One element tuple: {:?}",(5u32,));
    println!("Just an integer: {:?}",(5u32));

    //Destructure
    let tuple = (1, "hello", 4.5, true);
    let (a,b,c,d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}",a,b,c,d);

    //structure != tuple
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}",matrix);
}
