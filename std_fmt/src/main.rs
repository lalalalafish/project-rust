fn main() {
   //------------------------------Usage------------------------------
   let mut text:String;
   text = format!("Hello");//PRINT: Hello
   println!("{}",text);

   text = format!("Hello, {}!", "world");//PRINT: Hello, world
   println!("{}",text);

   text = format!("The number is {}",1);//PRINT: The number is 1
   println!("{}",text);

   text = format!("{:?}",(3,4));//PRINT (3,4) (remaining)
   println!("{}",text);

   text = format!("{value}",value=4);//PRINT: 4
   println!("{}",text);

   let people = "people";
   text = format!("Hello, {people}!");//PRINT: Hello, people!
   println!("{}",text);

   text = format!("{} {}",1,2);//PRINT: 1 2
   println!("{}",text);

   text = format!("{:04}",42);//PRINT: 0042
   println!("{}",text);
   
   text = format!("{:#?}",(100,200));//PRINT: 
                              //(
                              //    100,
                              //    200
                              //)
    println!("{}",text);
   //------------------------------Position Parameters------------------------------
    text = format!("{1} {} {0} {}",1,2);//PRINT: 2 1 1 2
    println!("{}",text);
   //------------------------------Named Parameters------------------------------
   text = format!("{argument}",argument = "test");//PRINT: test
   println!("{}",text);

   text = format!("{name} {}", 1, name = 2);//PRINT: 2 1
   println!("{}",text);

   text = format!("{a} {c} {b}", a="a", b='b', c=3);//PRINT: a 3 b
   println!("{}",text);

   //reference a variable that name in the current scope when the named parameter does not appear in the argument list
   let argument = 2 + 2;
   text = format!("{argument}");//PRINT: 4
   println!("{}",text);

   fn make_string(a:u32,b:&str)->String{
      format!("{b} {a}")//the return value
   }

   make_string(927, "Label");//PRINT: Label 927

   // text = format!("{age} {1}",age = 12,1);//cannot put positional numbers after using named parameter
   // println!("{}",text);

   //----------------------------Formatting Parameters------------------------------------
   //----------------------------Width------------------------------------
   //PRINT: Hello x     !
   println!("Hello {:5}!","x");
   println!("Hello {:1$}!","x",5);
   println!("Hello {1:0$}!",5,"x");//recommended
   println!("Hello {:width$}!","x",width = 5);//recommended
   let width = 5;
   println!("Hello,{:width$}!","x");//recommended
   //----------------------------Fill/Alignment------------------------------------
   assert_eq!(format!("Hello {:<5}!","x"), "Hello x    !");
   assert_eq!(format!("Hello {:-<5}!","x"),"Hello x----!");
   assert_eq!(format!("Hello {:^5}!","x"), "Hello   x  !");
   assert_eq!(format!("Hello {:>5}!","x"), "Hello     x!");

   println!("Hello {:^15}!",format!("{:?}",Some("hi")));
   //----------------------------Sign/#/0------------------------------------
   assert_eq!(format!("Hello {:+}!", 5),"Hello +5!");
   assert_eq!(format!("{:#x}!",27),"0x1b!");
   assert_eq!(format!("Hello {:05}!",5),"Hello 00005!");
   assert_eq!(format!("Hello {:05}!",-5),"Hello -0005!");
   assert_eq!(format!("{:#010x}!",27),"0x0000001b!");
   //----------------------------Precision------------------------------------
   //All should PRINT: Hello x is 0.01000
   println!("Hello {0} is {1:.5}","x",0.01);
   println!("Hello {1} is {2:.0$}",5,"x", 0.01);
   println!("Hello {0} is {2:.1$}","x",5,0.01);
   println!("Hello {} is {:.*}","x",5,0.01);
   println!("Hello {1} is {2:.*}",5,"x",0.01);
   println!("Hello {} is {2:.*}","x", 5, 0.01);
   println!("Hello {} is {number:.prec$}","x",prec=5,number=0.01);

   println!("{}, `{name:.*}` has 3 fractional digits","Hello",3,name=1234.56);
   println!("{}, `{name:.*}` has 3 characters","Hello",3,name="1234.56");
   println!("{}, `{name:>8.*}` has 3 right-aligned characters","Hello",3,name="1234.56");

   println!("{0:.1$}",1.2345,3);
   println!("{0:.1$}",1.2355,3);
   //----------------------------Escaping------------------------------------
   println!("Hello {{}}");
   println!("{{ Hello");
   println!("Hello }}");
   //----------------------------Formatting Trait-------------------------------
   assert_eq!(format!("{} {:?}",3,4),"3 4");
   assert_eq!(format!("{} {:?}",'a','b'),"a 'b'");
   assert_eq!(format!("{} {:?}","foo\n","bar\n"),"foo\n \"bar\\n\"");
}
