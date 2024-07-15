use std::fmt;

#[derive(Debug)]//For comparsion
struct MinMax(i64,i64);

impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"({}, {})",self.0,self.1)
    }
}

#[derive(Debug)]//For comparsion
struct Point2D{
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main(){
    let minmax = MinMax(0,14);
    println!("Compare structures: ");
    println!("Display: {}",minmax);
    println!("Debug: {:?}",minmax);

    let big_range = MinMax(-300,300);
    let small_range = MinMax(-3,3);
    println!("\nUse the fmt::Display implementation: \nThe big range is {big} and the small is {small}\n",
            small = small_range,
            big = big_range);
    
    let point = Point2D{x:3.3, y: 7.2};
    println!("Compare points: ");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    //{:b}: need the fmt::Binary trait implementation
}