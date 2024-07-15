#![crate_name = "r1_4_documentation"]

///A human being is represented here
pub struct Person{
    ///A person must have a name, no matter how much Juliet may hate it
    name: String,
}//end struct Person

impl Person{
    ///Creates a person with the given name.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //You can have rust code between fences inside the comments
    /// //If you pass --test to `rustdoc`, it will even test if for you!
    /// use r1_4_documentation::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str)-> Person{
        Person{
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name](Person::name)" to the `Person` it is called on
    pub fn hello(&self){
        println!("Hello, {}", self.name);
    }
}

fn main() {
    let john = Person::new("John");
    john.hello();
}//end main
