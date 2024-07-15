#[allow(dead_code)]//ignore the dead_code lint

#[derive(Debug)]
//[1.1] Struct definition
struct Person{
    name: String,
    age: u8,
}

//[1.2] A unit struct definition
struct Unit;

//[1.3] A tuple struct definition
struct Pair(i32,f32);

//[1.4] A struct with two fields
struct Point{
    x: f32,
    y: f32,
}

//[1.5] Struct can be used as fields of another struct
#[allow(dead_code)]//ignore the dead_code lint
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

fn main(){
    //[2.1] Create struct
    let name = String::from("Peter");
    let age = 27;
    let peter = Person{name,age};
    println!("{:?}",peter);

    //[2.2]
    let point: Point = Point{x:10.3,y:0.4};
    let another_point: Point = Point{x:5.2,y:0.2};

    //[3] Struct access
    println!("point coordinates: ({}, {})",point.x,point.y);

    //[2.3] Make new instantiate by useing struct update syntax
    let bottom_right = Point{x:5.2, ..another_point};
    println!("second point: ({}, {})",bottom_right.x, bottom_right.y);

    //[4] Destructure
    let Point{x: left_edge, y: top_edge} = point;
    let _rectangle = Rectangle{
        top_left: Point{x:left_edge,y: top_edge},
        bottom_right: bottom_right,
    };

    //[2.4] Instantiate a unit struct
    let _unit = Unit;

    //[2.5] Instantiate a tuple struct
    let pair = Pair(1,0.1);
    //[3]
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //[4.1]
    let Pair(integer,decimal) = pair;
    println!("pair contains {:?} and {:?}",integer,decimal);

}

