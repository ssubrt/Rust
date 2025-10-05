
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]


struct User{
    username: String,
    password: String,
}


fn main(){
    println!("Hello world!");

    let u = User{
        username: String::from("harkirat"),
        password: String::from("123456"),
    };

    let mut v : Vec<u8> = Vec::new();

    let ans = u.serialize(&mut v);

    // match ans{
    //     Ok(_) => println!("Serialized: {:?}", v),
    //     Err(e) => println!("Error serializing: {}", e),
    // }

    println!("Serialized: {:?}", v);

    let user = User::try_from_slice(&v).unwrap();
    println!("userName is : {}", user.username);
}

