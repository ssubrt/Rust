#[derive(Debug,Display)] // this is a debug and display macro

struct User{
    name: String,
    age: u32,
}


// this is Display trait not macro
impl Display for User{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "User {{ age: {} }}",  self.age)
    }
};

// this is Debug trait not macro
impl Debug for User{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "This is the USer Debug Struct name: {}, age: {}", self.name, self.age)
    }
}

// Note Rather than this we can use #[derive(Debug, Display)] above the struct


fn main(){
    let u1 = User{
        name: String::from("Subrat"),
        age: 22
    };

    print!("User1: {}", u1); // Debug trait allows us to print the struct using {:?}
    print!("USer Name is: {}", u1.name); // Display
    print!("User Age is: {}", u1.age);

}