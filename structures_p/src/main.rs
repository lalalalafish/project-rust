use std::fmt;

#[allow(dead_code)]//ignore the dead_code lint

#[derive(Debug)]
struct Point{
    x: f32,
    y: f32,
}

struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        write!(f,"({}, {})",self.x,self.y)
    }
}

impl fmt::Display for Rectangle{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        write!(f,"\n{}\n{}",self.top_left,self.bottom_right)
    }
}


fn rect_area(rectangle: Rectangle)->f32{
    let Rectangle{top_left:point1, bottom_right: point2} = rectangle;
    let Point{x:x1,y:y1} = point1;
    let Point{x:x2,y:y2} = point2;

    (x1-x2).abs() * (y1-y2).abs()
}

fn square(point: Point, num: f32) -> Rectangle{
    let top:Point = point;
    let bottom: Point = Point{x:top.x + num,y:top.y-num};

    Rectangle{top_left: top,bottom_right: bottom}
}

fn main(){
    let rec = Rectangle{top_left:Point{x: 1.1, y: 5.1},bottom_right:Point{x:5.1, y:1.1}};
    println!("The rectangle is {}",rec);
    println!("The area of the rectangle is {}",rect_area(rec));

    let point = Point{x:1.1, y:5.1};
    let length:f32 = 4.0;
    println!("The point is {}",point);
    println!("The length is {}",length);
    println!("The rectange is {}",square(point,length));
}