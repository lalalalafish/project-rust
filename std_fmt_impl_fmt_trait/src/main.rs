use std::fmt;

#[derive(Debug)]
struct Vector2D{
    x: isize,
    y: isize,
}

impl fmt::Display for Vector2D{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        //f: implements the `Write` trait, which is what the write! macro is expecting.
        //This formatting ignores the vairous flags provided to format strings
        write!(f,"({}, {})",self.x, self.y)
    }
}

//Different traits allow different forms of output of a type. 
//The meaning of this format is to print the magnitude of a vactor
impl fmt::Binary for Vector2D{
    fn fmt(&self,f:&mut fmt::Formatter<'_>) -> fmt::Result{
        let magnitude = (self.x * self.x + self.y * self.y) as f64;
        let magnitude = magnitude.sqrt();
        
        //Respect the formatting flags by using the helper method
        //`pad_integral` on the Formatter object.
        //`pad` function can be used to pad strings
        let decimals = f.precision().unwrap_or(3);
        let string = format!("{magnitude:.decimals$}");
        f.pad_integral(true,"", &string)
    }
}

fn main(){
    let myvector = Vector2D {x:3,y:4};
    
    println!("{myvector}");//PRINT: (3,4)
    println!("{myvector:?}");//PRINT: Vector2D {x:3,y: 4}
    println!("{myvector:10.3b}");//PRINT:       5.000
}