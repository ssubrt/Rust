

#[derive(Debug)];

struct User{
    name: String,
    email: String,
    age: u32
};


fn main(){
    let u1 = User{
        name: String::from("Alice"),
        email: String::from("F0NwI@example.com"),
        age: 30
    };

    println!("User1: {:?}", u1); // Debug trait allows us to print the struct using {:?}
    // println!("User1: {}",u1); // Display
}