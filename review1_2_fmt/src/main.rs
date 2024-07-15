//Review std::fmt
//Contents:
//1. Positional parameters
//2. Named parameters
//3. Formatting parameters
//  3.1. Width
//  3.2. Fill/Alignment
//  3.3. Sign/#/0
//  3.4. Precision
//  3.5. Localization
//  3.6. Escaping
//  3.7. SUMMARY

fn main() {
    println!("Part1-traits");
    let mut output = format!("Hello");
    println!("{output}");
    output = format!("Hello, {}","world");
    println!("{output}");
    output = format!("The number is: {}",1);
    println!("{output}");
    output = format!("{:?}",(3,4));
    println!("{output}");
    output = format!("{value}",value=4);
    println!("{output}");
    let p = "people";
    output = format!("Hello {p}!");
    println!("{output}");
    output = format!("{}{}",1,2);
    println!("{output}");
    output = format!("{:04}",42);
    println!("{output}");
    output = format!("{:#?}",(100,200));
    println!("{output}");

    println!("\nPart2-Positional parameters");
    //1. The index begin at 0
    //2. The parameters that explicitly named do not affect 
    //   parameters that inexplicitly named.
    //3. All arguments must be used in the format string
    output = format!("{1} {} {0} {}",1,2);
    println!("{output}");

    println!("\nPart3-Named parameters");
    //1. If the named parameter does not appear in the argument list,
    //   format! will reference a variable with that name in the current scope
    let text = "text in the current scope";
    println!("{text}",text = "text in the argument list");
    println!("{text}");
    //2. Named parameter must be put at the end of the argument list
    println!("{age} {0}",12,age = "GOOD");
    
    println!("\nPart4-Formatting parameters");
    println!("4.1-Width");
    println!("Hello, {:5}!","123");
    println!("Hello, {:1$}!","123",5);
    println!("Hello, {1:0$}!",5, "123");
    let width = 5;//usize
    println!("Hello, {:width$}!","123");
    println!("4.2-Fill/Alignment");
    println!("4.2.1.-Default Alignment");
    //By default: numerics is right-aligned. Non-numerics is left-aligned
    println!("Hello, {:5}!","123");
    println!("Hello, {:5}!",123);
    println!("4.2.2-Fill/Alignment");
    println!("Hello, {:^5}!",123);
    println!("Hello, {:<5}!",123);
    println!("Hello, {:>5}!",123);
    println!("Hello, {:0^5}!",123);
    println!("Hello, {:z^5}!",123);
    println!("Hello, {:@^5}!",123);
    
    println!("4.3-Sign/#/0");
    println!("Hello,{:+}!",123);
    println!("Hello, {:#?}!",123);
    println!("Hello, {:#x}!",123);
    println!("Hello, {:#X}!",123);
    println!("Hello, {:#o}!",123);
    println!("Hello, {:#b}!",123);
    println!("Hello, {:0>5}!",-12);
    println!("Hello, {:05}!",-12);
    println!("Hello, {:#04b}!",2);
    println!("Hello, {:#05b}!",2);
    println!("Hello, {:+#05b}!",2);
    println!("4.4-Precision");
    println!("For non-numeric types: ");
    println!("\tHello, {:.5}!","12.12345");
    println!("\tHello, {:.3}!","12.12345");
    println!("For numeric types:(only floating-point numbers) ");
    println!("\tHello, {:.5}!",12.12345);
    println!("\tHello, {:.3}!",12.12345);
    println!("\tHello, {:.*}!",3, 12.12345);
    println!("\tHello, {:.1$}!",12.12345,3);
    println!("\tHello, {1:.0$}!",3,12.12345);
    println!("\tHello, {:.prec$}!",12.12345,prec=3);
    println!("\tHello, {:%^10.*}!",3,12.12345);
    println!("4.5-Escaping");
    println!("Hello, {{!");
    println!("Hello, }}!");
    println!("4.6-SUMMARY");
    println!("Hello, {argument:@^+#012.prec$?   }!",argument = 12.12345,prec = 4);
    println!("Hello, {argument:@^+#12.prec$?  }!",argument = 12.12345,prec = 4);
}
