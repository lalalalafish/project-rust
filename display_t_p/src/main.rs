use std::fmt;

struct Vector(Vec<i32>);

impl fmt::Display for Vector{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"[")?;
        let vec = &self.0;
        for(count, v) in vec.iter().enumerate(){
            if count != 0{write!(f,", ")?;}
            write!(f,"{}: {}",count,v)?;
        }
        write!(f,"]")
    }
}

fn main(){
    let v = Vector(vec![1,2,3,]);
    println!("{}",v);
}