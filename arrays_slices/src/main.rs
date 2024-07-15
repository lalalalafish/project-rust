use std::mem;//for size_of_val() to proof that array is stack allocated

//Function analyze_slice: to brrows a slice
fn analyze_slice(slice: &[i32]){
    println!("First element of the slice: {}", slice[0]);//[1.1]Access the elements of slice
    println!("The slice has {} elements", slice.len());//[1.2]Access the length of slice
}

fn main(){
    //[2.1] Fixed-size array definition
    let xs: [i32;5] = [1,2,3,4,5];

    //[2.2] Array initialization with the same value
    let ys: [i32;500] = [0;500];

    //[2.3.1] Array access
    println!("First element of the array: {}",xs[0]);
    println!("Second element of the array: {}",xs[1]);
    println!("Length of the array: {}",xs.len());
    //[2.3.2] Array access safely: arrayName.get(index)
    //Return an `Option` enum variant(Some and None value)
    for i in 0..xs.len()+1 {
        match xs.get(i){
            Some(xval) => println!("{}: {}",i,xval),
            None => println!("Slow down! {} is too far!",i),
        }
    }


    //[2.4] Array is stack allocated
    println!("Array occupies {} bytes",mem::size_of_val(&xs));

    //[2.5] Array can be automatically borrowed as slices
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);
    println!("Borrow a section of array as a slice.");
    analyze_slice(&ys[1..4]);

    //Empty slice `&[]`
    let empty_array: [u32;0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);//Same but more verbose冗长

    //Difference between slice and array
    // Out of bound indexing on array causes compile time error.
    // println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    // println!("{}", xs[..][5]);
}