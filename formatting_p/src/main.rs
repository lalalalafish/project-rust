use std::fmt::{self,Formatter,Display};

struct Color{
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        let red:i64 = self.red.into();
        let green:i64 = self.green.into();
        let blue:i64 = self.blue.into();
        let calculation:i64 = (red * 65536) + (green * 256) + blue;
        write!(f,"RGB ({}, {}, {}) {:>#08X}",self.red,self.green,self.blue,calculation)
    }
}

fn main(){
    for color in [
        Color {red: 128, green: 255, blue: 90},
        Color {red: 0, green: 3, blue: 254},
        Color {red: 0, green: 0, blue: 0},
    ]{
        println!("{}",color);
    }
}