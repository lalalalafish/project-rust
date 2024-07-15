//[1] enum type definition
enum WebEvent{
    PageLoad,
    PageUnload,
    KeyPress(char),//tuple-like struct
    Paste(String),
    Click{x:i64,y:i64},//c-like struct
}

//[2] Funtion takes a `WebEvent` enum as an argument
fn inspect(event: WebEvent){
    match event{
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed '{}'",c),
        WebEvent::Paste(s) => println!("Paste \"{}\"",s),
        WebEvent::Click{x,y} => println!("Clicked at ({}, {})",x,y),
    }
}

fn main(){
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());//Create an owned `String` from a string slice
    let click = WebEvent::Click{x:20,y:80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}