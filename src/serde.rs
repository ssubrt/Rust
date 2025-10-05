
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug)]


struct User{
    username: String,
    password: String,
}


fn main(){
    println!("Hello, world!");

    let u = User{
        username: String::from("admin"),
        password: String::from("123456"),
    };

    let s = serde_json::to_string(&u);
    // let user_string = s.unwrap();
    match s{
        Ok(str) => println!("Serialized: {}", str),
        Err(e) => println!("Error converting to string {}", e),
    };

    let user_string  = serde_json::to_string(&u).unwrap();
    let d : Result<User, serde_json::Error> = serde_json::from_str(&user_string);
    match d{
        Ok(user) => println!("Deserialized: {}", user),
        Err(e) => println!("Error converting to string {}", e),
    };
}


