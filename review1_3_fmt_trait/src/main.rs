use std::fmt;

#[derive(Debug)]
struct Vector2D{
    x:isize,
    y:isize,
}

impl fmt::Display for Vector2D{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{
        write!(f,"({}, {})",self.x, self.y)
    }
}

impl fmt::Binary for Vector2D{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{
        let magnitude = (self.x * self.x + self.y*self.y) as f64;
        let magnitude = magnitude.sqrt();
        let decimals = f.precision().unwrap_or(3);
        let string = format!("{magnitude:.decimals$}");
        f.pad_integral(true, "", &string)
    }
}
fn main() {
    let vector = Vector2D{x:3, y:4};
    println!("{vector}");//imple fmt::Display
    println!("{vector:?}");
    println!("{vector:.3b}");//impl fmt::Binary
    println!("{vector:10.3b}");//impl fmt::Binary
}
